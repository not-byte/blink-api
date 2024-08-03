use ic_cdk::trap;
use state::STATE;
use user::User;
use utils::CallerTrait;

mod conversation;
mod messages;
mod state;
mod user;
mod utils;

#[ic_cdk::query]
fn greet() -> String {
    let caller = anon!();
    format!("Your PrincipalId is: {}", caller)
}

#[ic_cdk::query]
fn get_users() -> Vec<User> {
    STATE.with_borrow(|v| v.users.clone())
}
