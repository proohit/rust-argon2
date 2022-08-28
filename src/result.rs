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
}

#[derive(Default)]
pub struct Argon2ValueBuilder {
    first_param: String,
    second_param: String,
    hash: String,
    ref_lane: String,
    ref_index: String,
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
    pub fn set_value(&mut self, key: String, value: Argon2Value) {
        self.hash_map
            .as_object_mut()
            .unwrap()
            .insert(key, value.to_json_value());
    }

    pub fn set_memory_state_value(
        &mut self,
        lane: u32,
        index: u32,
        iteration: u32,
        value: Argon2Value,
    ) {
        self.set_value(Argon2State::memory_state_key(lane, index, iteration), value);
    }

    pub fn get_memory_state_value(&mut self, lane: u32, index: u32, iteration: u32) -> Argon2Value {
        let value = self
            .hash_map
            .as_object_mut()
            .unwrap()
            .get(&Argon2State::memory_state_key(lane, index, iteration))
            .unwrap_or(&Argon2Value::builder().build().to_json_value())
            .clone();
        serde_json::from_value(value).unwrap()
    }

    pub fn memory_state_key(lane: u32, index: u32, iteration: u32) -> String {
        format!("B^{}[{}][{}]", iteration, lane, index)
    }
}

impl Argon2Value {
    pub fn new(
        first_param: String,
        second_param: String,
        hash: String,
        ref_lane: String,
        ref_index: String,
    ) -> Argon2Value {
        Argon2Value {
            first_param,
            second_param,
            hash,
            ref_index,
            ref_lane,
        }
    }
    pub fn builder() -> Argon2ValueBuilder {
        Argon2ValueBuilder::default()
    }
    pub fn to_json_value(&self) -> Value {
        json!({
            "first_param": self.first_param,
            "second_param": self.second_param,
            "hash": self.hash,
            "ref_lane": self.ref_lane,
            "ref_index": self.ref_index,
        })
    }
}

impl Argon2ValueBuilder {
    pub fn new() -> Argon2ValueBuilder {
        Argon2ValueBuilder {
            first_param: String::new(),
            second_param: String::new(),
            hash: String::new(),
            ref_lane: String::new(),
            ref_index: String::new(),
        }
    }
    pub fn from_argon2_value(value: Argon2Value) -> Argon2ValueBuilder {
        Argon2ValueBuilder {
            first_param: value.first_param,
            second_param: value.second_param,
            hash: value.hash,
            ref_lane: value.ref_lane,
            ref_index: value.ref_index,
        }
    }
    pub fn first_param(mut self, first_param: String) -> Argon2ValueBuilder {
        self.first_param = first_param;
        self
    }
    pub fn second_param(mut self, second_param: String) -> Argon2ValueBuilder {
        self.second_param = second_param;
        self
    }
    pub fn hash(mut self, hash: String) -> Argon2ValueBuilder {
        self.hash = hash;
        self
    }
    pub fn ref_lane(mut self, ref_lane: String) -> Argon2ValueBuilder {
        self.ref_lane = ref_lane;
        self
    }
    pub fn ref_index(mut self, ref_index: String) -> Argon2ValueBuilder {
        self.ref_index = ref_index;
        self
    }

    pub fn build(self) -> Argon2Value {
        Argon2Value::new(
            self.first_param,
            self.second_param,
            self.hash,
            self.ref_lane,
            self.ref_index,
        )
    }
}
