use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum ThreadMode {
    /// Run in one thread.
    Sequential,

    #[cfg(feature = "crossbeam-utils")]
    /// Run in the same number of threads as the number of lanes.
    Parallel,
}

impl ThreadMode {
    #[cfg(feature = "crossbeam-utils")]
    /// Create a thread mode from the threads count.
    pub fn from_threads(threads: u32) -> ThreadMode {
        if threads > 1 {
            ThreadMode::Parallel
        } else {
            ThreadMode::Sequential
        }
    }
}

impl Default for ThreadMode {
    fn default() -> ThreadMode {
        ThreadMode::Sequential
    }
}
