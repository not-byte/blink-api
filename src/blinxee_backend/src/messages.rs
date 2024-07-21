use candid::{CandidType, Principal};
use ic_cdk::trap;
use serde::Deserialize;

use crate::{state::STATE, user::UserTrait};

// NOTE: Id can be changed to uuid
#[derive(CandidType, Deserialize, Clone)]
pub struct Message {
    id: u64,
    caller: Principal,
    receiver: Principal,
    content: String,
    timestamp: u64,
}

pub type Conversation = Vec<Message>;

#[ic_cdk::update]
fn send_message(receiver: Principal, content: String) {
    let caller = ic_cdk::caller();
    let timestamp = ic_cdk::api::time() / 1_000_000;

    STATE.with_borrow_mut(|state| {
        // TODO: Add proper id selection
        let message = Message {
            id: 0,
            caller,
            receiver,
            content,
            timestamp,
        };

        let (Some(caller), Some(receiver)) = (
            caller.to_user_state(state.to_owned()),
            receiver.to_user_state(state.to_owned()),
        ) else {
            trap(r#"{"message": "User not found"}"#);
        };

        state
            .conversations
            .entry((caller, receiver))
            .or_default()
            .push(message);
    })
}

#[ic_cdk::query]
fn get_messages_with(receiver: Principal) -> Conversation {
    let caller = ic_cdk::caller();
    let Some(conversation) = get_conversation(caller, receiver) else {
        trap(r#"{"message": "Conversation not found"}"#);
    };
    conversation
}

#[ic_cdk::update]
fn remove_message(receiver: Principal, id: u64) {
    let caller = ic_cdk::caller();
    let res = get_conversation_mut(caller, receiver, |conversation| {
        let Some(index) = conversation.iter().position(|v| v.id == id) else {
            trap(r#"{"message": "Message not found"}"#);
        };
        conversation.remove(index);
    });

    if res.is_none() {
        trap(r#"{"message": "Conversation not found"}"#);
    }
}

#[ic_cdk::update]
fn update_message(receiver: Principal, id: u64, new_message: String) {
    let caller = ic_cdk::caller();
    let res = get_conversation_mut(caller, receiver, |conversation| {
        let Some(index) = conversation.iter().position(|v| v.id == id) else {
            trap(r#"{"message": "Message not found"}"#);
        };

        if let Some(v) = conversation.get_mut(index) {
            v.content = new_message.clone();
        }
    });

    if res.is_none() {
        trap(r#"{"message": "Conversation not found"}"#);
    }
}

fn get_conversation(caller: Principal, receiver: Principal) -> Option<Conversation> {
    let (Some(caller), Some(receiver)) = (caller.to_user(), receiver.to_user()) else {
        return None;
    };

    STATE.with_borrow(|state| {
        if let Some(conversation) = state
            .conversations
            .get(&(caller.clone(), receiver.clone()))
            .cloned()
        {
            return Some(conversation);
        }

        if let Some(conversation) = state.conversations.get(&(receiver, caller)).cloned() {
            return Some(conversation);
        }

        None
    })
}

fn get_conversation_mut<F, R>(caller: Principal, receiver: Principal, mut f: F) -> Option<R>
where
    F: FnMut(&mut Conversation) -> R,
{
    let (Some(caller), Some(receiver)) = (caller.to_user(), receiver.to_user()) else {
        return None;
    };

    STATE.with_borrow_mut(|state| {
        if let Some(conversation) = state
            .conversations
            .get_mut(&(caller.clone(), receiver.clone()))
        {
            return Some(f(conversation));
        }

        if let Some(conversation) = state.conversations.get_mut(&(receiver, caller)) {
            return Some(f(conversation));
        }

        None
    })
}
