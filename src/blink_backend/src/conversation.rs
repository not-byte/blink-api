use candid::{CandidType, Principal};
use ic_cdk::trap;
use serde::Deserialize;

use crate::{anon, messages::Message, state::STATE, utils::Conversations, CallerTrait};

// NOTE: Later can add conversation settings here
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Conversation {
    pub id: u64,
    pub users: Vec<Principal>,
    pub messages: Vec<Message>,
}

impl Conversation {
    fn new(id: u64, users: Vec<Principal>) -> Self {
        Self {
            id,
            users,
            messages: Vec::new(),
        }
    }
}

#[ic_cdk::update]
fn create_conversation(users: Vec<Principal>) {
    anon!();
    STATE.with_borrow_mut(|state| {
        state
            .conversations
            .push(Conversation::new(state.conversations.get_last_id(), users));
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
