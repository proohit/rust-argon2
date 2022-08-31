use serde_json::json;

use crate::{
    context::Context,
    core::Position,
    log_json,
    result::{Argon2Result, Argon2ValueBuilder},
};

pub fn write_state_entry(
    state: &mut Argon2Result,
    position: &Position,
    ref_index: u32,
    ref_lane: u64,
    context: &Context,
) {
    let current_val =
        state
            .state
            .get_memory_state_value(position.lane, position.index, position.pass);

    let mut updated_val_builder = Argon2ValueBuilder::from_argon2_value(current_val);

    updated_val_builder.ref_index(ref_index.to_string());

    let string_ref_lane =
        get_string_ref_lane(position.pass, position.slice, position.lane, ref_lane)
            .unwrap_or(String::from(""));
    updated_val_builder.ref_lane(string_ref_lane);

    // let absolute_ref_iteration = get_ref_iteration(ref_index, context, position);
    // updated_val_builder.ref_iteration(absolute_ref_iteration.to_string());

    state.state.set_memory_state_value(
        position.lane,
        position.index,
        position.pass,
        updated_val_builder.build(),
    );
}

fn get_string_ref_lane(pass: u32, slice: u32, lane: u32, pseudo_rand: u64) -> Option<String> {
    if (pass == 0) && (slice == 0) {
        Some(lane.to_string())
    } else if pass > 0 && slice == 0 {
        Some(pseudo_rand.to_string())
    } else {
        None
    }
}

fn get_ref_iteration(ref_index: u32, context: &Context, position: &Position) -> String {
    let max_ref_iteration_gap = 4 * context.config.lanes as i32;
    let oldest_referencable_iteration =
        std::cmp::min(position.pass as i32 - max_ref_iteration_gap, 0);

    let iteration_size = context.config.mem_cost as i32 / max_ref_iteration_gap;
    let relative_ref_iteration = ref_index as i32 / iteration_size;
    let absolute_ref_iteration = if position.pass == 0 {
        0
    } else {
        std::cmp::max(
            0,
            position.pass as i32 - (max_ref_iteration_gap - relative_ref_iteration),
        )
    };
    if position.pass == 1 {
        log_json(json!({
            "ref_index": ref_index,
            "oldest_referencable_iteration": max_ref_iteration_gap,
            "iteration_size": iteration_size,
            "relative_ref_iteration": relative_ref_iteration,
            "absolute_ref_iteration": absolute_ref_iteration,
        }));
    }
    let index_in_ref_iteration = ref_index as i32 % max_ref_iteration_gap;
    absolute_ref_iteration.to_string()
}
