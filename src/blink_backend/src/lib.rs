use ic_cdk::trap;
use messages::Message;
use state::STATE;
use std::collections::HashMap;
use user::User;
use utils::CallerTrait;

mod messages;
mod state;
mod user;
mod utils;

#[ic_cdk::query]
fn greet() -> String {
    let caller = anon!();
    format!("Your PrincipalId is: {}", caller)
}

// NOTE: Remove in prod
#[ic_cdk::query]
fn get_all() -> HashMap<(User, User), Vec<Message>> {
    // let caller = anon!();
    // ic_cdk::println!("{:#?}", caller);
    STATE.with_borrow_mut(|state| {
        ic_cdk::println!("{:#?}", state.users);
        return state.conversations.clone();
    })
}
