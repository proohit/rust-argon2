use crate::context::Context;
use base64;

struct Options {
    mem_cost: u32,
    time_cost: u32,
    parallelism: u32,
}

pub fn encode_string(context: &Context, hash: &[u8]) -> String {
    format!(
        "${}$v={}$m={},t={},p={}${}${}",
        context.config.variant,
        context.config.version,
        context.config.mem_cost,
        context.config.time_cost,
        context.config.lanes,
        base64::encode_config(context.salt, base64::STANDARD_NO_PAD),
        base64::encode_config(hash, base64::STANDARD_NO_PAD),
    )
}
