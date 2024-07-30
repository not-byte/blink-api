use candid::Principal;

use crate::conversation::Conversation;

pub trait CallerTrait {
    /// Shortcut for comparing Principals
    fn is_anonymous(&self) -> bool;
}

impl CallerTrait for Principal {
    fn is_anonymous(&self) -> bool {
        *self == Principal::anonymous()
    }
}

pub trait Conversations {
    fn find(&mut self, conversation_id: u64) -> Option<&mut Conversation>;
    fn filter(&self, caller: Principal) -> Vec<Conversation>;
    fn get_last_id(&self) -> u64;
}

impl Conversations for Vec<Conversation> {
    fn find(&mut self, conversation_id: u64) -> Option<&mut Conversation> {
        self.iter_mut().find(|v| v.id == conversation_id)
    }

    fn filter(&self, caller: Principal) -> Vec<Conversation> {
        self.iter()
            .filter(|v| v.users.contains(&caller))
            .cloned()
            .collect()
    }

    fn get_last_id(&self) -> u64 {
        self.iter().map(|v| v.id).max().unwrap_or(0)
    }
}

#[macro_export]
macro_rules! anon {
    () => {{
        let caller = ic_cdk::caller();
        if caller.is_anonymous() {
            trap(r#"{"message": "User is anonymous"}"#);
        }
        caller
    }};
}

#[macro_export]
macro_rules! update_if_some {
    ($target:expr, $option:expr) => {
        if let Some(v) = $option {
            $target = v;
        }
    };
}
