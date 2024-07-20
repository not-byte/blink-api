use candid::{CandidType, Principal};
use ic_cdk::trap;
use serde::Deserialize;

use crate::{state::STATE, user::User};

// NOTE: Id can be changed to uuid
#[derive(CandidType, Deserialize, Clone)]
pub struct Message {
    id: u64,
    sender: User,
    receiver: User,
    content: String,
    timestamp: u64,
}

type Conversation = Vec<Message>;

#[ic_cdk::update]
fn send_message(receiver: Principal, content: String) {
    let sender = ic_cdk::caller();
    let timestamp = ic_cdk::api::time() / 1_000_000;

    STATE.with_borrow_mut(|state| {
        // TODO: Add proper id selection
        let message = Message {
            id: 0,
            sender,
            receiver,
            content,
            timestamp,
        };

        state
            .conversations
            .entry((sender, receiver))
            .or_default()
            .push(message);
    })
}

#[ic_cdk::query]
fn get_messages_with(user: Principal) -> Conversation {
    let caller = ic_cdk::caller();
    STATE.with_borrow(|state| {
        state.conversations.get(&(caller, user)).cloned().unwrap_or(
            state
                .conversations
                .get(&(user, caller))
                .cloned()
                .unwrap_or_default(),
        )
    })
}

#[ic_cdk::update]
fn remove_message(receiver: Principal, id: u64) {
    let caller = ic_cdk::caller();
    STATE.with_borrow_mut(|state| {
        if let Some(conversation) = state.conversations.get_mut(&(caller, receiver)) {
            let Some(index) = conversation.iter().position(|v| v.id == id) else {
                trap(r#"{"message": "Message not found"}"#);
            };
            conversation.remove(index);
        }
    })
}

#[ic_cdk::update]
fn update_message(receiver: Principal, id: u64, new_message: String) {
    let caller = ic_cdk::caller();
    STATE.with_borrow_mut(|state| {
        if let Some(conversation) = state.conversations.get_mut(&(caller, receiver)) {
            let Some(index) = conversation.iter().position(|v| v.id == id) else {
                trap(r#"{"message": "Message not found"}"#);
            };
            if let Some(v) = conversation.get_mut(index) {
                v.content = new_message;
            }
        }
    })
}
