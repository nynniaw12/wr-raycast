use crate::{core::player::Player, map::world::WorldMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputAction {
    MoveForward,
    MoveBackward,
    TurnLeft,
    TurnRight,
    Sprint,
}
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
