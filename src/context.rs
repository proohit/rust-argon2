use crate::config::Config;
use crate::result::Result;

#[derive(Debug, PartialEq)]
pub struct Context<'a> {
    pub config: Config<'a>,

    pub lane_length: u32,

    pub memory_blocks: u32,

    pub pwd: &'a [u8],

    pub salt: &'a [u8],

    pub segment_length: u32,
}

impl<'a> Context<'a> {
    pub fn new(config: Config<'a>, pwd: &'a [u8], salt: &'a [u8]) -> Result<Context<'a>> {
        Ok(Context {
            config,
            lane_length: 0,
            memory_blocks: 0,
            pwd,
            salt,
            segment_length: 0,
        })
    }
}
