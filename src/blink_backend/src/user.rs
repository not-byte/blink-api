use crate::utils::CallerTrait;
use candid::{CandidType, Principal};
use ic_cdk::trap;
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
    fn to_user(&self) -> Option<User>;
    fn to_user_state(&self, state: State) -> Option<User>;
}

impl UserTrait for Principal {
    fn to_user(&self) -> Option<User> {
        STATE.with_borrow(|v| v.users.iter().find(|user| user.principal == *self).cloned())
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
fn add_user(username: String, avatar: Option<String>) {
    let caller = anon!();
    STATE.with_borrow_mut(|state| {
        if caller.to_user_state(state.to_owned()).is_some() {
            trap(r#"{"message": "User already exists"}"#);
        }

        state.users.push(User {
            principal: caller,
            username,
            avatar,
            language: Language::English,
            theme: Theme::Dark,
            status: Status::Online,
        });
    })
}
