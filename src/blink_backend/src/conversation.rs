use candid::{CandidType, Principal};
use ic_cdk::trap;
use serde::Deserialize;

use crate::{
    anon, messages::Message, state::STATE, update_if_some, user::UserTrait, utils::Filter,
    CallerTrait,
};

// NOTE: Later can add conversation settings here
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Conversation {
    pub id: u64,
    pub name: String,
    pub users: Vec<Principal>,
    pub messages: Vec<Message>,
}

impl Conversation {
    fn new(id: u64, users: Vec<Principal>, name: String) -> Self {
        Self {
            id,
            users,
            name,
            messages: Vec::new(),
        }
    }
}

#[ic_cdk::update]
fn create_conversation(users_: Vec<Principal>) {
    let caller = anon!();
    let mut users = vec![caller];
    users.extend(users_);
    STATE.with_borrow_mut(|state| {
        let name = if users.len() == 2 {
            users
                .iter()
                .find(|&&v| v != caller)
                .unwrap()
                .to_user_state(state.to_owned())
                .unwrap_or_else(|| trap(r#"{"message": "User does not exists"}"#))
                .username
        } else {
            users
                .iter()
                .take(3)
                .map(|v| {
                    v.to_user_state(state.to_owned())
                        .unwrap_or_else(|| trap(r#"{"message": "User does not exists"}"#))
                        .username
                })
                .collect::<Vec<String>>()
                .join(", ")
        };

        state.conversations.push(Conversation::new(
            state.conversations.get_last_id() + 1,
            users,
            name,
        ));
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

        // We can safely unwrap because we know that conversation exists
        if conversation.users.contains(&caller) {
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
        let Some(conversation) = state.conversations.find(conversation_id) else {
            trap(r#"{"message": "Conversation not found"}"#);
        };

        let Some(_) = conversation.users.iter().position(|v| *v == caller) else {
            trap(r#"{"message": "User not in conversation"}"#);
        };

        update_if_some!(conversation.name, name);
    })
}

#[ic_cdk::update]
fn join_conversation(conversation_id: u64) {
    let caller = anon!();
    STATE.with_borrow_mut(|state| {
        let Some(conversation) = state.conversations.find(conversation_id) else {
            trap(r#"{"message": "Conversation not found"}"#);
        };
        conversation.users.push(caller)
    })
}

#[ic_cdk::update]
fn leave_conversation(conversation_id: u64) {
    let caller = anon!();
    STATE.with_borrow_mut(|state| {
        let Some(conversation) = state.conversations.find(conversation_id) else {
            trap(r#"{"message": "Conversation not found"}"#);
        };

        let Some(index) = conversation.users.iter().position(|v| *v == caller) else {
            trap(r#"{"message": "User not in conversation"}"#);
        };

        conversation.users.remove(index)
    });
}

#[ic_cdk::query]
fn get_user_conversations() -> Vec<Conversation> {
    let caller = anon!();
    STATE.with_borrow_mut(|state| state.conversations.filter(caller))
}
