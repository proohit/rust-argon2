// Copyright (c) 2017 Martijn Rijkeboer <mrr@sru-systems.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::str::FromStr;

use serde::Deserialize;
use serde_json::Value;

use crate::common;
use crate::thread_mode::ThreadMode;
use crate::variant::Variant;
use crate::version::Version;

/// Structure containing configuration settings.
///
/// # Examples
///
/// ```
/// use argon2::{Config, ThreadMode, Variant, Version};
///
/// let config = Config::default();
/// assert_eq!(config.ad, &[]);
/// assert_eq!(config.hash_length, 32);
/// assert_eq!(config.lanes, 1);
/// assert_eq!(config.mem_cost, 4096);
/// assert_eq!(config.secret, &[]);
/// assert_eq!(config.thread_mode, ThreadMode::Sequential);
/// assert_eq!(config.time_cost, 3);
/// assert_eq!(config.variant, Variant::Argon2i);
/// assert_eq!(config.version, Version::Version13);
/// ```
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Config<'a> {
    /// The associated data.
    pub ad: &'a [u8],

    /// The length of the resulting hash.
    pub hash_length: u32,

    /// The number of lanes.
    pub lanes: u32,

    /// The amount of memory requested (KB).
    pub mem_cost: u32,

    /// The key.
    pub secret: &'a [u8],

    /// The thread mode.
    pub thread_mode: ThreadMode,

    /// The number of passes.
    pub time_cost: u32,

    /// The variant.
    pub variant: Variant,

    /// The version number.
    pub version: Version,
}

impl<'a> Config<'a> {
    pub fn uses_sequential(&self) -> bool {
        self.thread_mode == ThreadMode::Sequential || self.lanes == 1
    }

    pub fn from_json(config_json: &'a str) -> Config<'a> {
        let raw_config: Value = serde_json::from_str(config_json).unwrap();
        let mut config = Config::default();
        config.hash_length = raw_config["hash_length"]
            .as_str()
            .unwrap_or(common::DEF_HASH_LENGTH.to_string().as_str())
            .parse::<u32>()
            .unwrap();
        config.lanes = raw_config["parallelism"]
            .as_str()
            .unwrap_or(common::DEF_LANES.to_string().as_str())
            .parse::<u32>()
            .unwrap();
        config.mem_cost = raw_config["memory"]
            .as_str()
            .unwrap_or(common::DEF_MEMORY.to_string().as_str())
            .parse::<u32>()
            .unwrap();
        config.time_cost = raw_config["iterations"]
            .as_str()
            .unwrap_or(common::DEF_TIME.to_string().as_str())
            .parse::<u32>()
            .unwrap();
        //DEFAULTS
        config.thread_mode = ThreadMode::from_str(
            raw_config["thread_mode"]
                .as_str()
                .unwrap_or(ThreadMode::default().as_str()),
        )
        .unwrap();
        config.variant = Variant::from_str(
            raw_config["variant"]
                .as_str()
                .unwrap_or(Variant::default().as_uppercase_str()),
        )
        .unwrap();
        config.version = Version::from_str(
            raw_config["version"]
                .as_str()
                .unwrap_or(Version::default().as_str()),
        )
        .unwrap();
        config
    }
}

impl<'a> Default for Config<'a> {
    fn default() -> Config<'a> {
        Config {
            ad: &[],
            hash_length: common::DEF_HASH_LENGTH,
            lanes: common::DEF_LANES,
            mem_cost: common::DEF_MEMORY,
            secret: &[],
            thread_mode: ThreadMode::default(),
            time_cost: common::DEF_TIME,
            variant: Variant::default(),
            version: Version::default(),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::config::Config;
    use crate::thread_mode::ThreadMode;
    use crate::variant::Variant;
    use crate::version::Version;

    #[test]
    fn default_returns_correct_instance() {
        let config = Config::default();
        assert_eq!(config.hash_length, 32);
        assert_eq!(config.lanes, 1);
        assert_eq!(config.mem_cost, 4096);
        assert_eq!(config.thread_mode, ThreadMode::Sequential);
        assert_eq!(config.time_cost, 3);
        assert_eq!(config.variant, Variant::Argon2i);
        assert_eq!(config.version, Version::Version13);
    }
}
