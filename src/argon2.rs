use crate::config::Config;
use crate::context::Context;
use crate::core;
use crate::encoding;
use crate::memory::Memory;
use crate::result::Argon2Result;

pub fn hash_encoded_js(pwd: String, salt: String, config_json: String) -> String {
    let config = Config::from_json(config_json);
    let mut result = Argon2Result::new();
    hash_encoded(pwd.as_bytes(), salt.as_bytes(), &config, &mut result);
    String::from("")
}

pub fn hash_encoded(pwd: &[u8], salt: &[u8], config: &Config, state: &mut Argon2Result) -> String {
    let context = Context::new(config.clone(), pwd, salt).unwrap();
    let hash = run(&context, state);
    let encoded = encoding::encode_string(&context, &hash);
    return encoded;
}

fn run(context: &Context, state: &mut Argon2Result) -> Vec<u8> {
    let mut memory = Memory::new(context.config.lanes, context.lane_length);
    core::fill_memory_blocks_mt(context, &mut memory, state);
    vec![]
}
