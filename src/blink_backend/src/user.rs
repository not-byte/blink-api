use crate::{
    error::{Error, ErrorKind},
    update_if_some,
    utils::CallerTrait,
};
use candid::{CandidType, Principal};
use serde::Deserialize;

use crate::{
    anon,
    state::{State, STATE},
};

#[derive(CandidType, Deserialize, Clone, Hash, PartialEq, Eq, Debug)]
pub enum Language {
    Polish,
    English,
}

#[derive(CandidType, Deserialize, Clone, Hash, PartialEq, Eq, Debug)]
pub enum Theme {
    Dark,
    Light,
    System,
}

#[derive(CandidType, Deserialize, Clone, Hash, PartialEq, Eq, Debug)]
pub enum Status {
    Online,
    Away,
    DoNotDisturb,
    Offline,
}

#[derive(CandidType, Deserialize, Clone, Hash, PartialEq, Eq, Debug)]
pub struct User {
    pub principal: Principal,
    pub username: String,
    pub avatar: Option<String>,
    pub language: Language,
    pub theme: Theme,
    pub status: Status,
}

pub trait UserTrait {
    /// Get the user from state
    fn to_user(&self) -> Option<User>;

    /// Get the user from state
    fn to_user_mut(&self) -> Option<&mut User>;

    /// Get the user with state already provided
    fn to_user_state(&self, state: State) -> Option<User>;
}

impl UserTrait for Principal {
    fn to_user(&self) -> Option<User> {
        STATE.with_borrow(|v| v.users.iter().find(|user| user.principal == *self).cloned())
    }

    fn to_user_mut(&self) -> Option<&mut User> {
        STATE.with(|state| {
            // Use unsafe to extend the lifetime of the mutable reference
            // This is safe because we know that the `RefCell` guarantees the borrow rules
            // and the closure passed to `with` ensures the borrows don't escape
            let mut state = state.borrow_mut();
            let users_ptr = state.users.as_mut_ptr();

            for i in 0..state.users.len() {
                let user = unsafe { &mut *users_ptr.add(i) };
                if user.principal == *self {
                    return Some(user);
                }
            }
            None
        })
    }

    fn to_user_state(&self, state: State) -> Option<User> {
        state
            .users
            .iter()
            .find(|user| user.principal == *self)
            .cloned()
    }
}

#[ic_cdk::update]
fn add_user(username: String, avatar: Option<String>) -> Result<(), Error> {
    let caller = anon!();
    STATE.with_borrow_mut(|state| {
        if caller.to_user_state(state.to_owned()).is_some() {
            return Err(ErrorKind::UserAlreadyExists.into());
        }

        state.users.push(User {
            principal: caller,
            username,
            avatar,
            language: Language::English,
            theme: Theme::Dark,
            status: Status::Online,
        });
        Ok(())
    })
}

#[ic_cdk::query]
fn get_user() -> Result<Option<User>, Error> {
    Ok(anon!().to_user())
}

#[ic_cdk::update]
fn update_user(
    username: Option<String>,
    avatar: Option<String>,
    language: Option<Language>,
    theme: Option<Theme>,
    status: Option<Status>,
) -> Result<(), Error> {
    let caller = anon!();
    let Some(user) = caller.to_user_mut() else {
        return Err(ErrorKind::UserDoesNotExist.into());
    };

    update_if_some!(user.username, username);
    update_if_some!(user.avatar, Some(avatar));
    update_if_some!(user.language, language);
    update_if_some!(user.theme, theme);
    update_if_some!(user.status, status);
    Ok(())
}
