use std::cell::RefCell;

use candid::CandidType;
use serde::Deserialize;

use crate::{conversation::Conversation, User};

#[derive(CandidType, Deserialize, Clone)]
pub struct State {
    pub conversations: Vec<Conversation>,
    pub users: Vec<User>,
}

impl State {
    fn new() -> Self {
        Self {
            conversations: Vec::new(),
            users: Vec::new(),
        }
    }
}

thread_local! {
    pub static STATE: RefCell<State> = RefCell::new(State::new());
}

// NOTE: Store conversations on stable storage
#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    STATE.with_borrow(|state| {
        ic_cdk::storage::stable_save((state.clone(),)).unwrap();
    });
}

// NOTE: Get conversations from stable storage
#[ic_cdk::post_upgrade]
fn post_upgrade() {
    STATE.with_borrow_mut(|state| {
        if let Ok((new_state,)) = ic_cdk::storage::stable_restore::<(State,)>() {
            *state = new_state;
        }
    });
}
