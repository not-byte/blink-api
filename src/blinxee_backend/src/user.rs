use candid::{CandidType, Principal};
use serde::Deserialize;

#[derive(CandidType, Deserialize, Clone, Hash, PartialEq, Eq)]
pub enum Language {
    Polish,
    English,
}

#[derive(CandidType, Deserialize, Clone, Hash, PartialEq, Eq)]
pub enum Theme {
    Dark,
    Light,
    System,
}

#[derive(CandidType, Deserialize, Clone, Hash, PartialEq, Eq)]
pub enum Status {
    Online,
    Away,
    DoNotDisturb,
    Offline,
}

#[derive(CandidType, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct User {
    principal: Principal,
    username: String,
    password: String,
    avatar: String,
    language: Language,
    theme: Theme,
    status: Status,
}
