use raylib::RaylibHandle;
use crate::{core::player::Player, map::world::WorldMap};

pub type InputHandler = fn(&RaylibHandle, &mut Player, &WorldMap);

pub fn handle_input(rl: &RaylibHandle, player: &mut Player, world_map: &WorldMap, handlers: &[InputHandler]) {
    for handler in handlers {
        handler(rl, player, world_map);
    }
}

