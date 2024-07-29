use std::{cell::RefCell, collections::HashMap};

use candid::CandidType;
use serde::Deserialize;

use crate::{messages::Conversation, User};

#[derive(CandidType, Deserialize, Clone)]
pub struct State {
    pub conversations: HashMap<(User, User), Conversation>,
    pub users: Vec<User>,
}

impl State {
    fn new() -> Self {
        Self {
            conversations: HashMap::default(),
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
