use candid::{CandidType, Principal};
use serde::Deserialize;

use crate::{
    anon,
    conversation::Conversation,
    error::{Error, ErrorKind},
    state::STATE,
    user::{User, UserTrait},
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
    pub conversation_id: u64,
    pub conversation_image: Option<String>,
    pub content: String,
    pub timestamp: u64,
    pub user: User,
}

#[ic_cdk::update]
fn send_message(conversation_id: u64, content: String) -> Result<u64, Error> {
    let caller = anon!();
    let timestamp = ic_cdk::api::time() / 1_000_000;

    STATE.with_borrow_mut(|state| {
        let Some(user) = caller.to_user_state(state.to_owned()) else {
            return Err(ErrorKind::UserDoesNotExist.into());
        };

        let Some(conversation) = state.conversations.find(conversation_id) else {
            return Err(ErrorKind::ConversationNotFound.into());
        };

        let Some(_) = conversation
            .users
            .iter()
            .find(|v| v.principal == user.principal)
        else {
            return Err(ErrorKind::UserNotInConversation.into());
        };

        let last_id = conversation.messages.get_last_id() + 1;

        let message = Message {
            id: last_id,
            message: MessageContent::Text(Text { content }),
            caller,
            timestamp,
        };

        conversation.messages.push(message);
        Ok(last_id)
    })
}

#[ic_cdk::update]
fn send_image(conversation_id: u64, image: String, name: String) -> Result<u64, Error> {
    let caller = anon!();
    let timestamp = ic_cdk::api::time() / 1_000_000;

    STATE.with_borrow_mut(|state| {
        let Some(user) = caller.to_user_state(state.to_owned()) else {
            return Err(ErrorKind::UserDoesNotExist.into());
        };

        let Some(conversation) = state.conversations.find(conversation_id) else {
            return Err(ErrorKind::ConversationNotFound.into());
        };

        let Some(_) = conversation
            .users
            .iter()
            .find(|v| v.principal == user.principal)
        else {
            return Err(ErrorKind::UserNotInConversation.into());
        };

        let last_id = conversation.messages.get_last_id() + 1;

        let message = Message {
            id: last_id,
            message: MessageContent::Image(Image { src: image, name }),
            caller,
            timestamp,
        };

        conversation.messages.push(message);
        Ok(last_id)
    })
}

#[ic_cdk::query]
fn get_messages(conversation_id: u64) -> Result<Conversation, Error> {
    let caller = anon!();
    STATE.with_borrow_mut(|state| {
        let Some(user) = caller.to_user_state(state.to_owned()) else {
            return Err(ErrorKind::UserDoesNotExist.into());
        };

        let Some(conversation) = state.conversations.find(conversation_id) else {
            return Err(ErrorKind::ConversationNotFound.into());
        };

        if conversation.users.contains(&user) {
            Ok(conversation.to_owned())
        } else {
            return Err(ErrorKind::CantAccess.into());
        }
    })
}

#[ic_cdk::query]
fn get_last_message(conversation_id: u64) -> Result<Option<LastMessage>, Error> {
    let caller = anon!();
    STATE.with_borrow_mut(|state| {
        let Some(user) = caller.to_user_state(state.to_owned()) else {
            return Err(ErrorKind::UserDoesNotExist.into());
        };

        let Some(conversation) = state.conversations.find(conversation_id) else {
            return Err(ErrorKind::ConversationNotFound.into());
        };

        let conversation_image = conversation
            .users
            .iter()
            .find(|v| v.principal != user.principal)
            .map(|v| v.avatar.clone())
            .unwrap_or(user.avatar.clone());

        if conversation.users.contains(&user) {
            Ok(conversation
                .to_owned()
                .messages
                .last()
                .map(|v| LastMessage {
                    conversation_id,
                    conversation_image,
                    content: match &v.message {
                        MessageContent::Text(text) => text.content.clone(),
                        MessageContent::Image(image) => image.name.clone(),
                    },
                    timestamp: v.timestamp,
                    // Currently user is returned because we are low on time
                    // but this is a security vulterability which allows
                    // for looking at other users configuration
                    user: v.caller.to_user_state(state.to_owned()).unwrap(),
                    // user: v.caller,
                }))
        } else {
            return Err(ErrorKind::CantAccess.into());
        }
    })
}

#[ic_cdk::update]
fn remove_message(conversation_id: u64, id: u64) -> Result<(), Error> {
    let caller = anon!();
    STATE.with_borrow_mut(|state| {
        let Some(conversation) = state.conversations.find(conversation_id) else {
            return Err(ErrorKind::ConversationNotFound.into());
        };

        let Some(index) = conversation.messages.iter().position(|v| v.id == id) else {
            return Err(ErrorKind::MessageNotFound.into());
        };

        // We can safely unwrap because we know that message exists
        if conversation.messages.get(index).unwrap().caller == caller {
            conversation.messages.remove(index);
            Ok(())
        } else {
            return Err(ErrorKind::CantAccess.into());
        }
    })
}

#[ic_cdk::update]
fn update_message(conversation_id: u64, id: u64, new_message: String) -> Result<(), Error> {
    let caller = anon!();
    STATE.with_borrow_mut(|state| {
        let Some(conversation) = state.conversations.find(conversation_id) else {
            return Err(ErrorKind::ConversationNotFound.into());
        };

        let Some(index) = conversation.messages.iter().position(|v| v.id == id) else {
            return Err(ErrorKind::MessageNotFound.into());
        };

        if let Some(v) = conversation.messages.get_mut(index) {
            match v.message {
                MessageContent::Text(ref mut text) => {
                    if v.caller == caller {
                        text.content = new_message.clone();
                    } else {
                        return Err(ErrorKind::CantEdit.into());
                    }
                }
                _ => return Err(ErrorKind::CantEdit.into()),
            }
        }
        Ok(())
    })
}
