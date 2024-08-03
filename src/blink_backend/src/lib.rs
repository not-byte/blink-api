use error::{Error, ErrorKind};
use state::STATE;
use user::User;
use utils::CallerTrait;

mod conversation;
mod error;
mod messages;
mod state;
mod user;
mod utils;

#[ic_cdk::query]
fn greet() -> Result<String, Error> {
    let caller = anon!();
    Ok(format!("Your PrincipalId is: {}", caller))
}

#[ic_cdk::query]
fn get_users() -> Vec<User> {
    STATE.with_borrow(|v| v.users.clone())
}
