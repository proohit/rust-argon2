use serde::{Deserialize, Serialize};

use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum Version {
    Version10 = 0x10,
    Version13 = 0x13,
}

impl Version {
    pub fn as_u32(&self) -> u32 {
        *self as u32
    }
}

impl Default for Version {
    fn default() -> Version {
        Version::Version13
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_u32())
    }
}
