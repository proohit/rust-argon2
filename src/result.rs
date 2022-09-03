use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::error::Error;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Argon2Result {
    pub hash: String,
    pub state: Argon2State,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Argon2State {
    pub hash_map: Value,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Argon2Value {
    first_param: String,
    second_param: String,
    hash: String,
    ref_lane: String,
    ref_index: String,
    ref_iteration: String,
}

#[derive(Default)]
pub struct Argon2ValueBuilder {
    first_param: String,
    second_param: String,
    hash: String,
    ref_lane: String,
    ref_index: String,
    ref_iteration: String,
}

impl Argon2Result {
    pub fn new() -> Argon2Result {
        Argon2Result {
            hash: "".to_string(),
            state: Argon2State::new(),
        }
    }
}

impl Argon2State {
    pub fn new() -> Argon2State {
        Argon2State {
            hash_map: json!({}),
        }
    }
}

impl Argon2Value {}
