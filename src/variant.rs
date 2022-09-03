use serde::{Deserialize, Serialize};

use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum Variant {
    Argon2d = 0,
    Argon2i = 1,
    Argon2id = 2,
}

impl Variant {
    pub fn as_lowercase_str(&self) -> &'static str {
        match *self {
            Variant::Argon2d => "argon2d",
            Variant::Argon2i => "argon2i",
            Variant::Argon2id => "argon2id",
        }
    }
}

impl Default for Variant {
    fn default() -> Variant {
        Variant::Argon2i
    }
}

impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_lowercase_str())
    }
}
