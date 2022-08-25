// Copyright (c) 2017 Martijn Rijkeboer <mrr@sru-systems.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::error::Error;
use std::result;

/// A specialized result type for Argon2 operations.
pub type Result<T> = result::Result<T, Error>;

#[derive(Serialize, Deserialize, Clone)]
pub struct Argon2Result {
    pub hash: String,
    pub state: Argon2State,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Argon2State {
    pub hash_map: Value,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Argon2Value {
    first_param: String,
    second_param: String,
    hash: String,
}

impl Argon2Result {
    pub fn new() -> Argon2Result {
        Argon2Result {
            hash: "".to_string(),
            state: Argon2State::new(),
        }
    }

    pub fn set_hash(&mut self, hash: &String) {
        self.hash = hash.to_string();
    }

    pub fn to_json(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl Argon2State {
    pub fn new() -> Argon2State {
        Argon2State {
            hash_map: json!({}),
        }
    }
    pub fn add_value(&mut self, key: String, value: Argon2Value) {
        self.hash_map
            .as_object_mut()
            .unwrap()
            .insert(key, value.to_json_value());
    }
}

impl Argon2Value {
    pub fn new(first_param: String, second_param: String, hash: String) -> Argon2Value {
        Argon2Value {
            first_param,
            second_param,
            hash,
        }
    }
    pub fn new_with_str(first_param: &str, second_param: &str, hash: &str) -> Argon2Value {
        Argon2Value {
            first_param: first_param.to_string(),
            second_param: second_param.to_string(),
            hash: hash.to_string(),
        }
    }
    pub fn to_json_value(&self) -> Value {
        json!({
            "first_param": self.first_param,
            "second_param": self.second_param,
        })
    }
}
