use candid::{CandidType, Principal};
use ic_cdk::trap;
use serde::Deserialize;

use crate::{anon, state::STATE, user::UserTrait, utils::CallerTrait};

// NOTE: Id can be changed to uuid
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Text {
    content: String,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Image {
    name: String,
    src: String,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
// For Some reason this doesn't work
// #[serde(tag = "type", content = "data")]
pub enum MessageContent {
    Text(Text),
    Image(Image),
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Message {
    id: u64,
    message: MessageContent,
    caller: Principal,
    receiver: Principal,
    timestamp: u64,
}

// NOTE: Conversation could be a type in which users are in array which would
// automaticaly implement group chats and maybe simplify getting users
// and could allow for chat customization
// Ex:
// struct Conversation {
//     users: Vec<Principal>,
//     messages: Vec<Message>,
// }
pub type Conversation = Vec<Message>;

#[ic_cdk::update]
fn send_message(receiver: Principal, content: String) {
    let caller = anon!();
    let timestamp = ic_cdk::api::time() / 1_000_000;

    STATE.with_borrow_mut(|state| {
        // TODO: Add proper id selection
        let message = Message {
            id: 0,
            message: MessageContent::Text(Text { content }),
            caller,
            receiver,
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

#[ic_cdk::update]
fn send_image(receiver: Principal, image: String, name: String) {
    let caller = anon!();
    let timestamp = ic_cdk::api::time() / 1_000_000;

    STATE.with_borrow_mut(|state| {
        // TODO: Add proper id selection
        let message = Message {
            id: 0,
            message: MessageContent::Image(Image { src: image, name }),
            caller,
            receiver,
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
    let caller = anon!();
    let Some(conversation) = get_conversation(caller, receiver) else {
        trap(r#"{"message": "Conversation not found"}"#);
    };
    conversation
}

#[ic_cdk::update]
fn remove_message(receiver: Principal, id: u64) {
    let caller = anon!();
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
    let caller = anon!();
    let res = get_conversation_mut(caller, receiver, |conversation| {
        let Some(index) = conversation.iter().position(|v| v.id == id) else {
            trap(r#"{"message": "Message not found"}"#);
        };

        if let Some(v) = conversation.get_mut(index) {
            match v.message {
                MessageContent::Text(ref mut v) => v.content = new_message.clone(),
                _ => trap(r#"{"message": "You can only edit a text message"}"#),
            }
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

    let mut total_conversation = Vec::new();

    STATE.with_borrow(|state| {
        if let Some(conversation) = state
            .conversations
            .get(&(caller.clone(), receiver.clone()))
            .cloned()
        {
            total_conversation.extend(conversation)
        }

        if let Some(conversation) = state.conversations.get(&(receiver, caller)).cloned() {
            total_conversation.extend(conversation)
        }
    });

    total_conversation.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    Some(total_conversation)
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
