use std::collections::HashSet;

use candid::{CandidType, Principal};
use serde::Deserialize;

use crate::{
    anon,
    error::{Error, ErrorKind},
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
fn create_conversation(users_: Vec<Principal>) -> Result<u64, Error> {
    let caller = anon!();
    let mut users = vec![caller];
    users.extend(users_);

    let mut result: Vec<User> = Vec::new();

    for user in users.iter() {
        match user.to_user() {
            Some(u) => result.push(u),
            None => return Err(ErrorKind::UserDoesNotExist.into()),
        }
    }

    STATE.with_borrow_mut(|state| {
        let name = if users.len() == 2 {
            result
                .clone()
                .iter()
                .find(|v| v.principal != caller)
                .unwrap()
                .username
                .clone()
        } else {
            result
                .clone()
                .iter()
                .take(3)
                .map(|v| v.username.clone())
                .collect::<Vec<String>>()
                .join(", ")
        };

        let existing = state.conversations.iter().find(|v| {
            iters_equal_anyorder(v.users.clone().into_iter(), result.clone().into_iter())
        });

        if let Some(conversation) = existing {
            return Err(ErrorKind::ConversationAlreadyExists(conversation.id).into());
        }

        let last_id = state.conversations.get_last_id() + 1;
        state
            .conversations
            .push(Conversation::new(last_id, result, name));
        Ok(last_id)
    })
}

#[ic_cdk::update]
fn remove_conversation(conversation_id: u64) -> Result<(), Error> {
    let caller = anon!();
    STATE.with_borrow_mut(|state| {
        let Some(user) = caller.to_user_state(state.to_owned()) else {
            return Err(ErrorKind::UserDoesNotExist.into());
        };

        let Some(index) = state
            .conversations
            .iter()
            .position(|v| v.id == conversation_id)
        else {
            return Err(ErrorKind::ConversationNotFound.into());
        };

        let conversation = state.conversations.get(index).unwrap();

        let Some(_) = conversation
            .users
            .iter()
            .find(|v| v.principal == user.principal)
        else {
            return Err(ErrorKind::UserNotInConversation.into());
        };

        if conversation.users.contains(&user) {
            state.conversations.remove(index);
        } else {
            return Err(ErrorKind::CantRemoveConversation.into());
        }

        Ok(())
    })
}

#[ic_cdk::update]
fn update_conversation(conversation_id: u64, name: Option<String>) -> Result<(), Error> {
    let caller = anon!();
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
            .position(|v| v.principal == user.principal)
        else {
            return Err(ErrorKind::UserNotInConversation.into());
        };

        update_if_some!(conversation.name, name);
        Ok(())
    })
}

#[ic_cdk::update]
fn add_to_conversation(conversation_id: u64, principals: Vec<Principal>) -> Result<(), Error> {
    let caller = anon!();
    STATE.with_borrow_mut(|state| {
        let Some(user) = caller.to_user_state(state.to_owned()) else {
            return Err(ErrorKind::UserDoesNotExist.into());
        };

        let mut users: Vec<User> = Vec::new();

        for v in principals.iter() {
            match v.to_user_state(state.to_owned()) {
                Some(user) => users.push(user),
                None => return Err(ErrorKind::UserDoesNotExist.into()),
            }
        }

        let Some(conversation) = state.conversations.find(conversation_id) else {
            return Err(ErrorKind::ConversationNotFound.into());
        };

        let Some(_) = conversation.users.iter().position(|v| *v == user) else {
            return Err(ErrorKind::UserNotInConversation.into());
        };

        conversation.users.extend(users);
        let set: HashSet<_> = conversation.users.drain(..).collect(); // dedup
        conversation.users.extend(set.into_iter());
        Ok(())
    })
}

#[ic_cdk::update]
fn leave_conversation(conversation_id: u64) -> Result<(), Error> {
    let caller = anon!();
    STATE.with_borrow_mut(|state| {
        let Some(user) = caller.to_user_state(state.to_owned()) else {
            return Err(ErrorKind::UserDoesNotExist.into());
        };

        let Some(conversation) = state.conversations.find(conversation_id) else {
            return Err(ErrorKind::ConversationNotFound.into());
        };

        let Some(index) = conversation.users.iter().position(|v| *v == user) else {
            return Err(ErrorKind::UserNotInConversation.into());
        };

        conversation.users.remove(index);
        Ok(())
    })
}

#[ic_cdk::query]
fn get_user_conversations() -> Result<Vec<Conversation>, Error> {
    let caller = anon!();
    let Some(user) = caller.to_user() else {
        return Err(ErrorKind::UserDoesNotExist.into());
    };

    STATE.with_borrow_mut(|state| Ok(state.conversations.filter(user)))
}
