use crate::{core::player::Player, map::world::WorldMap};
use raylib::{ffi::KeyboardKey, RaylibHandle};

pub fn handle_movement(rl: &RaylibHandle, player: &mut Player, world_map: &WorldMap) {
    let frametime = rl.get_frame_time();

    let mut move_speed = (frametime * 5.0) as f64;
    let rot_speed = (frametime * 3.0) as f64;
    if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) || rl.is_key_down(KeyboardKey::KEY_RIGHT_SHIFT) {
        move_speed = move_speed * 2.0;
    }

    if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
        // rot matrix mult
        let old_dir_x = player.dir_x;
        player.dir_x = player.dir_x * (-rot_speed).cos() - player.dir_y * (-rot_speed).sin();
        player.dir_y = old_dir_x * (-rot_speed).sin() + player.dir_y * (-rot_speed).cos();

        let old_plane_x = player.plane_x;
        player.plane_x = player.plane_x * (-rot_speed).cos() - player.plane_y * (-rot_speed).sin();
        player.plane_y = old_plane_x * (-rot_speed).sin() + player.plane_y * (-rot_speed).cos();
    }

    if rl.is_key_down(KeyboardKey::KEY_LEFT) {
        // rot matrix mult
        let old_dir_x = player.dir_x;
        player.dir_x = player.dir_x * (rot_speed).cos() - player.dir_y * (rot_speed).sin();
        player.dir_y = old_dir_x * (rot_speed).sin() + player.dir_y * (rot_speed).cos();

        let old_plane_x = player.plane_x;
        player.plane_x = player.plane_x * (rot_speed).cos() - player.plane_y * (rot_speed).sin();
        player.plane_y = old_plane_x * (rot_speed).sin() + player.plane_y * (rot_speed).cos();
    }
    if rl.is_key_down(KeyboardKey::KEY_DOWN) {
        if world_map[(player.pos_x - player.dir_x * move_speed) as usize][player.pos_y as usize]
            == 0
        {
            player.pos_x -= player.dir_x * move_speed
        }
        if world_map[player.pos_x as usize][(player.pos_y - player.dir_y * move_speed) as usize]
            == 0
        {
            player.pos_y -= player.dir_y * move_speed
        }
    }
    if rl.is_key_down(KeyboardKey::KEY_UP) {
        if world_map[(player.pos_x + player.dir_x * move_speed) as usize][player.pos_y as usize]
            == 0
        {
            player.pos_x += player.dir_x * move_speed
        }
        if world_map[player.pos_x as usize][(player.pos_y + player.dir_y * move_speed) as usize]
            == 0
        {
            player.pos_y += player.dir_y * move_speed
        }
    }
}
