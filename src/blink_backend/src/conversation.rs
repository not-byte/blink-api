use std::collections::HashSet;

use candid::{CandidType, Principal};
use ic_cdk::trap;
use serde::Deserialize;

use crate::{
    anon,
    messages::Message,
    state::STATE,
    update_if_some,
    user::{User, UserTrait},
    utils::{iters_equal_anyorder, Filter},
    CallerTrait,
};

// NOTE: Later can add conversation settings here
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Conversation {
    pub id: u64,
    pub name: String,
    pub users: Vec<User>,
    pub messages: Vec<Message>,
}

impl Conversation {
    fn new(id: u64, users: Vec<User>, name: String) -> Self {
        Self {
            id,
            users,
            name,
            messages: Vec::new(),
        }
    }
}

#[ic_cdk::update]
fn create_conversation(users_: Vec<Principal>) -> u64 {
    let caller = anon!();
    let mut users = vec![caller];
    users.extend(users_);
    let users: Vec<User> = users
        .iter()
        .map(|v| {
            v.to_user()
                .unwrap_or_else(|| trap(r#"{"message": "User does not exists"}"#))
        })
        .collect();
    STATE.with_borrow_mut(|state| {
        let name = if users.len() == 2 {
            users
                .clone()
                .iter()
                .find(|v| v.principal != caller)
                .unwrap()
                .username
                .clone()
        } else {
            users
                .clone()
                .iter()
                .take(3)
                .map(|v| v.username.clone())
                .collect::<Vec<String>>()
                .join(", ")
        };

        let existing = state
            .conversations
            .iter()
            .find(|v| iters_equal_anyorder(v.users.clone().into_iter(), users.clone().into_iter()));

        if existing.is_some() {
            trap(r#"{"message": "Conversation already exists"}"#);
        }

        let last_id = state.conversations.get_last_id() + 1;
        state
            .conversations
            .push(Conversation::new(last_id, users, name));
        last_id
    })
}

#[ic_cdk::update]
fn remove_conversation(conversation_id: u64) {
    let caller = anon!();
    STATE.with_borrow_mut(|state| {
        let Some(index) = state
            .conversations
            .iter()
            .position(|v| v.id == conversation_id)
        else {
            trap(r#"{"message": "User not in conversation"}"#);
        };

        let conversation = state.conversations.get(index).unwrap();

        let Some(user) = caller.to_user_state(state.to_owned()) else {
            trap(r#"{"message": "User does not exists"}"#);
        };

        // We can safely unwrap because we know that conversation exists
        if conversation.users.contains(&user) {
            state.conversations.remove(index);
        } else {
            trap(r#"{"message": "You can't remove this conversation"}"#)
        }
    })
}

#[ic_cdk::update]
fn update_conversation(conversation_id: u64, name: Option<String>) {
    let caller = anon!();
    STATE.with_borrow_mut(|state| {
        let Some(user) = caller.to_user_state(state.to_owned()) else {
            trap(r#"{"message": "User does not exists"}"#);
        };

        let Some(conversation) = state.conversations.find(conversation_id) else {
            trap(r#"{"message": "Conversation not found"}"#);
        };

        let Some(_) = conversation.users.iter().position(|v| *v == user) else {
            trap(r#"{"message": "User not in conversation"}"#);
        };

        update_if_some!(conversation.name, name);
    })
}

#[ic_cdk::update]
fn add_to_conversation(conversation_id: u64, principals: Vec<Principal>) {
    let caller = anon!();
    STATE.with_borrow_mut(|state| {
        let Some(user) = caller.to_user_state(state.to_owned()) else {
            trap(r#"{"message": "User does not exists"}"#);
        };

        let users: Vec<User> = principals
            .iter()
            .map(|v| {
                v.to_user_state(state.to_owned()).unwrap_or_else(|| {
                    trap(format!(r#"{{"message": "User does not exists"}}"#).as_str())
                })
            })
            .collect();

        let Some(conversation) = state.conversations.find(conversation_id) else {
            trap(r#"{"message": "Conversation not found"}"#);
        };

        let Some(_) = conversation.users.iter().position(|v| *v == user) else {
            trap(r#"{"message": "User not in conversation"}"#);
        };

        conversation.users.extend(users);
        let set: HashSet<_> = conversation.users.drain(..).collect(); // dedup
        conversation.users.extend(set.into_iter());
    })
}

#[ic_cdk::update]
fn leave_conversation(conversation_id: u64) {
    let caller = anon!();
    STATE.with_borrow_mut(|state| {
        let Some(user) = caller.to_user_state(state.to_owned()) else {
            trap(r#"{"message": "User does not exists"}"#);
        };

        let Some(conversation) = state.conversations.find(conversation_id) else {
            trap(r#"{"message": "Conversation not found"}"#);
        };

        let Some(index) = conversation.users.iter().position(|v| *v == user) else {
            trap(r#"{"message": "User not in conversation"}"#);
        };

        conversation.users.remove(index)
    });
}

#[ic_cdk::query]
fn get_user_conversations() -> Vec<Conversation> {
    let caller = anon!();
    let Some(user) = caller.to_user() else {
        trap(r#"{"message": "User does not exists"}"#);
    };

    STATE.with_borrow_mut(|state| state.conversations.filter(user))
}
