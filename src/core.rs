use crate::common;
use crate::context::Context;
use crate::memory::Memory;
use crate::result::Argon2Result;
#[cfg(feature = "crossbeam-utils")]
use crossbeam_utils::thread::scope;

#[derive(Clone, Debug)]
pub struct Position {
    pub pass: u32,
    pub lane: u32,
    pub slice: u32,
    pub index: u32,
}

#[cfg(feature = "crossbeam-utils")]
pub fn fill_memory_blocks_mt(context: &Context, memory: &mut Memory, state: &mut Argon2Result) {
    for p in 0..context.config.time_cost {
        for s in 0..common::SYNC_POINTS {
            let _ = scope(|scoped| {
                for (l, mem) in (0..context.config.lanes).zip(memory.as_lanes_mut()) {
                    let position = Position {
                        pass: p,
                        lane: l,
                        slice: s,
                        index: 0,
                    };
                    scoped.spawn(move |_| {
                        fill_segment(context, &position, mem, state);
                    });
                }
            });
        }
    }
}

fn fill_segment(
    context: &Context,
    position: &Position,
    memory: &mut Memory,
    state: &mut Argon2Result,
) {
}
