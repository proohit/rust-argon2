#![cfg_attr(feature = "crossbeam-utils", doc = "    lanes: 4,")]
#![cfg_attr(
    feature = "crossbeam-utils",
    doc = "    thread_mode: ThreadMode::Parallel,"
)]
#![cfg_attr(not(feature = "crossbeam-utils"), doc = "    lanes: 1,")]
#![cfg_attr(
    not(feature = "crossbeam-utils"),
    doc = "    thread_mode: ThreadMode::Sequential,"
)]

mod argon2;
mod block;
mod common;
mod config;
mod context;
mod core;
mod encoding;
mod error;
mod memory;
mod result;
mod thread_mode;
mod variant;
mod version;

pub use crate::argon2::*;
pub use crate::config::Config;
pub use crate::error::Error;
pub use crate::result::Result;
pub use crate::thread_mode::ThreadMode;
pub use crate::variant::Variant;
pub use crate::version::Version;
