use candid::Principal;
use messages::Message;
use state::STATE;
use std::collections::HashMap;
use user::User;

mod messages;
mod state;
mod user;

// NOTE: Remove in prod
#[ic_cdk::query]
fn get_all() -> HashMap<(Principal, Principal), Vec<Message>> {
    let caller = ic_cdk::caller();
    // ic_cdk::println!("{:#?}", caller);
    STATE.with_borrow(|state| {
        return state.conversations.clone();
    })
}
