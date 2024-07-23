use messages::Message;
use state::STATE;
use std::collections::HashMap;
use candid::Principal;
use user::User;

mod messages;
mod state;
mod user;

#[ic_cdk::query]
fn greet(principal: Principal, name: String) -> String {
    format!("Hello, {} Your PrincipalId is: {}", name, principal)
}

// NOTE: Remove in prod
#[ic_cdk::query]
fn get_all() -> HashMap<(User, User), Vec<Message>> {
    // let caller = ic_cdk::caller();
    // ic_cdk::println!("{:#?}", caller);
    STATE.with_borrow_mut(|state| {
        ic_cdk::println!("{:#?}", state.users);
        return state.conversations.clone();
    })
}
