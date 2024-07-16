use candid::{CandidType, Deserialize, Principal};
use std::{cell::RefCell, collections::HashMap};

// TODO: Add Image type
// NOTE: Id can be changed to uuid
#[derive(CandidType, Deserialize, Clone)]
struct Message {
    id: u64,
    sender: Principal,
    receiver: Principal,
    content: String,
    timestamp: u64,
}

type Conversation = Vec<Message>;

#[derive(Default)]
struct State {
    conversations: HashMap<(Principal, Principal), Conversation>,
}

// TODO: Implement ic-stable-structures
thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

#[ic_cdk::update]
fn send_message(receiver: Principal, content: String) {
    todo!()
}

#[ic_cdk::query]
fn get_messages_with(user: Principal) -> Conversation {
    todo!()
}

#[ic_cdk::update]
fn remove_message(receiver: Principal, id: u64) {
    todo!()
}

#[ic_cdk::update]
fn update_message(receiver: Principal, id: u64, new_message: String) {
    todo!()
}

// NOTE: Remove in prod
#[ic_cdk::query]
fn get_all() -> HashMap<(Principal, Principal), Vec<Message>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
}
