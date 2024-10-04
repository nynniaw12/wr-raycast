pub mod movement;

use super::map::world::WorldMap;
use crate::{core::player::Player, input::InputAction};

pub type InputHandler = fn(&[InputAction], &mut Player, &WorldMap, frame_time: f64);

pub fn handle_input(
    inputs: &[InputAction],
    player: &mut Player,
    world_map: &WorldMap,
    handlers: &[InputHandler],
    frame_time: f64,
) {
    for handler in handlers {
        handler(inputs, player, world_map, frame_time);
    }
}
