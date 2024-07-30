use candid::Principal;

pub trait CallerTrait {
    /// Shortcut for comparing Principals
    fn is_anonymous(&self) -> bool;
}

impl CallerTrait for Principal {
    fn is_anonymous(&self) -> bool {
        *self == Principal::anonymous()
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
