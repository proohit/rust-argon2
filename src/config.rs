use serde::{Deserialize, Serialize};

use crate::thread_mode::ThreadMode;
use crate::variant::Variant;
use crate::version::Version;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Config<'a> {
    pub ad: &'a [u8],

    pub hash_length: u32,

    pub lanes: u32,

    pub mem_cost: u32,

    pub secret: &'a [u8],

    pub thread_mode: ThreadMode,

    pub time_cost: u32,

    pub variant: Variant,

    pub version: Version,

    pub stop_at_iteration: u32,
}

impl<'a> Config<'a> {
    pub fn from_json(config_json: String) -> Config<'a> {
        Config::default()
    }
}

impl<'a> Default for Config<'a> {
    fn default() -> Config<'a> {
        Config {
            ad: &[],
            hash_length: 0,
            lanes: 0,
            mem_cost: 0,
            secret: &[],
            thread_mode: ThreadMode::default(),
            time_cost: 0,
            variant: Variant::default(),
            version: Version::default(),
            stop_at_iteration: 0,
        }
    }
}
