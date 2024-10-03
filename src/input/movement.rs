use super::handler::InputAction;
use crate::{core::player::Player, map::world::WorldMap};

pub fn handle_movement(
    inputs: &[InputAction],
    player: &mut Player,
    world_map: &WorldMap,
    frame_time: f64,
) {
    let mut move_speed = frame_time * 5.0;
    let rot_speed = frame_time * 3.0;

    if inputs.contains(&InputAction::Sprint) {
        move_speed *= 2.0;
    }

    for action in inputs {
        match action {
            InputAction::Sprint => (),
            InputAction::TurnRight => rotate_player(player, -rot_speed),
            InputAction::TurnLeft => rotate_player(player, rot_speed),
            InputAction::MoveBackward => move_player(player, world_map, -move_speed),
            InputAction::MoveForward => move_player(player, world_map, move_speed),
        }
    }
}

fn rotate_player(player: &mut Player, angle: f64) {
    let old_dir_x = player.dir_x;
    player.dir_x = player.dir_x * angle.cos() - player.dir_y * angle.sin();
    player.dir_y = old_dir_x * angle.sin() + player.dir_y * angle.cos();
    let old_plane_x = player.plane_x;
    player.plane_x = player.plane_x * angle.cos() - player.plane_y * angle.sin();
    player.plane_y = old_plane_x * angle.sin() + player.plane_y * angle.cos();
}

fn move_player(player: &mut Player, world_map: &WorldMap, speed: f64) {
    let new_x = player.pos_x + player.dir_x * speed;
    let new_y = player.pos_y + player.dir_y * speed;

    if world_map[new_x as usize][player.pos_y as usize] == 0 {
        player.pos_x = new_x;
    }
    if world_map[player.pos_x as usize][new_y as usize] == 0 {
        player.pos_y = new_y;
    }
}
