use candid::{CandidType, Principal};
use ic_cdk::trap;
use serde::Deserialize;

use crate::{
    anon,
    conversation::Conversation,
    state::STATE,
    utils::{CallerTrait, Filter},
};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Text {
    pub content: String,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Image {
    pub name: String,
    pub src: String,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum MessageContent {
    Text(Text),
    Image(Image),
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Message {
    pub id: u64,
    pub message: MessageContent,
    pub caller: Principal,
    pub timestamp: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct LastMessage {
    pub content: String,
    pub timestamp: u64,
    pub user: Principal,
}

#[ic_cdk::update]
fn send_message(conversation_id: u64, content: String) {
    let caller = anon!();
    let timestamp = ic_cdk::api::time() / 1_000_000;

    STATE.with_borrow_mut(|state| {
        let Some(conversation) = state.conversations.find(conversation_id) else {
            trap(r#"{"message": "Conversation not found"}"#);
        };

        if !conversation.users.contains(&caller) {
            trap(r#"{"message": "User not in conversation"}"#);
        }

        let message = Message {
            id: conversation.messages.get_last_id() + 1,
            message: MessageContent::Text(Text { content }),
            caller,
            timestamp,
        };

        conversation.messages.push(message);
    })
}

#[ic_cdk::update]
fn send_image(conversation_id: u64, image: String, name: String) {
    let caller = anon!();
    let timestamp = ic_cdk::api::time() / 1_000_000;

    STATE.with_borrow_mut(|state| {
        let Some(conversation) = state.conversations.find(conversation_id) else {
            trap(r#"{"message": "Conversation not found"}"#);
        };

        if !conversation.users.contains(&caller) {
            trap(r#"{"message": "User not in conversation"}"#);
        }

        let message = Message {
            id: conversation.messages.get_last_id() + 1,
            message: MessageContent::Image(Image { src: image, name }),
            caller,
            timestamp,
        };

        conversation.messages.push(message);
    })
}

#[ic_cdk::query]
fn get_messages(conversation_id: u64) -> Conversation {
    let caller = anon!();
    STATE.with_borrow_mut(|state| {
        let Some(conversation) = state.conversations.find(conversation_id) else {
            trap(r#"{"message": "Conversation not found"}"#);
        };

        if conversation.users.contains(&caller) {
            conversation.to_owned()
        } else {
            trap(r#"{"message": "You can't access other conversations"}"#);
        }
    })
}

#[ic_cdk::query]
fn get_last_message(conversation_id: u64) -> Option<LastMessage> {
    let caller = anon!();
    STATE.with_borrow_mut(|state| {
        let Some(conversation) = state.conversations.find(conversation_id) else {
            trap(r#"{"message": "Conversation not found"}"#);
        };

        if conversation.users.contains(&caller) {
            conversation
                .to_owned()
                .messages
                .last()
                .map(|v| LastMessage {
                    content: match &v.message {
                        MessageContent::Text(text) => text.content.clone(),
                        MessageContent::Image(image) => image.name.clone(),
                    },
                    timestamp: v.timestamp,
                    user: v.caller,
                })
        } else {
            trap(r#"{"message": "You can't access other conversations"}"#);
        }
    })
}

#[ic_cdk::update]
fn remove_message(conversation_id: u64, id: u64) {
    let caller = anon!();
    STATE.with_borrow_mut(|state| {
        let Some(conversation) = state.conversations.find(conversation_id) else {
            trap(r#"{"message": "Conversation not found"}"#);
        };

        let Some(index) = conversation.messages.iter().position(|v| v.id == id) else {
            trap(r#"{"message": "Message not found"}"#);
        };

        // We can safely unwrap because we know that message exists
        if conversation.messages.get(index).unwrap().caller == caller {
            conversation.messages.remove(index);
        } else {
            trap(r#"{"message": "You can only remove your own messages"}"#)
        }
    });
}

#[ic_cdk::update]
fn update_message(conversation_id: u64, id: u64, new_message: String) {
    let caller = anon!();
    STATE.with_borrow_mut(|state| {
        let Some(conversation) = state.conversations.find(conversation_id) else {
            trap(r#"{"message": "Conversation not found"}"#);
        };

        let Some(index) = conversation.messages.iter().position(|v| v.id == id) else {
            trap(r#"{"message": "Message not found"}"#);
        };

        if let Some(v) = conversation.messages.get_mut(index) {
            match v.message {
                MessageContent::Text(ref mut text) => {
                    if v.caller == caller {
                        text.content = new_message.clone()
                    } else {
                        trap(r#"{"message": "You can only edit your own messages"}"#)
                    }
                }
                _ => trap(r#"{"message": "You can only edit a text message"}"#),
            }
        }
    });
}
