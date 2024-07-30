use candid::Principal;

use crate::{conversation::Conversation, messages::Message};

pub trait CallerTrait {
    /// Shortcut for comparing Principals
    fn is_anonymous(&self) -> bool;
}

impl CallerTrait for Principal {
    fn is_anonymous(&self) -> bool {
        *self == Principal::anonymous()
    }
}

pub trait Filter<T> {
    /// Find T in Vec<T> giving a mutable reference
    fn find(&mut self, id: u64) -> Option<&mut T>;

    /// Find all T where caller exists
    fn filter(&self, caller: Principal) -> Vec<T>;

    /// Get last id from Vec<T>
    fn get_last_id(&self) -> u64;
}

impl Filter<Conversation> for Vec<Conversation> {
    /// Find Conversation in Vec<Conversation> giving a mutable reference
    fn find(&mut self, conversation_id: u64) -> Option<&mut Conversation> {
        self.iter_mut().find(|v| v.id == conversation_id)
    }

    /// Find all Conversations where caller exists
    fn filter(&self, caller: Principal) -> Vec<Conversation> {
        self.iter()
            .filter(|v| v.users.contains(&caller))
            .cloned()
            .collect()
    }

    /// Get last id from Vec<Conversation>
    fn get_last_id(&self) -> u64 {
        self.iter().map(|v| v.id).max().unwrap_or(0)
    }
}

impl Filter<Message> for Vec<Message> {
    /// Get last id from Vec<Messages>
    fn get_last_id(&self) -> u64 {
        self.iter().map(|v| v.id).max().unwrap_or(0)
    }

    #[allow(unused)]
    fn filter(&self, caller: Principal) -> Vec<Message> {
        unimplemented!()
    }

    #[allow(unused)]
    fn find(&mut self, id: u64) -> Option<&mut Message> {
        unimplemented!()
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
