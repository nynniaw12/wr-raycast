use crate::{
    core::{player::Player, state::GameState},
    map::world::WorldMap,
};
use std::cmp::{max, min};

pub fn wall_cast(
    GameState {
        image,
        player,
        world_map,
        texture_manager,
        sprite_manager,
        screen,
        ..
    }: &mut GameState,
) {
    for x in 0..screen.width {
        // x pos of camera in camera plane
        let camera_x: f64 = ((2.0 * x as f64) / screen.width as f64) - 1.0;
        // x pos of camera in camera plane
        let dir_x: f64 = player.dir_x + player.plane_x * camera_x; // get an angled ray vector
        let dir_y: f64 = player.dir_y + player.plane_y * camera_x;

        let (side, perp_wall_dist, map_x, map_y) = get_coll(dir_x, dir_y, &player, &world_map);

        // use affine texture mapping to create buffer
        let line_height: i32 = (screen.height as f64 / perp_wall_dist) as i32; // height of
        let draw_start: i32 = max(-line_height / 2 + screen.height as i32 / 2, 0); // screen
        let draw_end: i32 = min(
            screen.height as i32 - 1,
            line_height / 2 + screen.height as i32 / 2,
        );
        let tex_num = world_map[map_x as usize][map_y as usize] - 1; // enables texture 0
        let mut wall_x: f64 = if side == 0 {
            player.pos_y + perp_wall_dist * dir_y
        } else {
            player.pos_x + perp_wall_dist * dir_x
        };

        wall_x -= wall_x.floor();
        let mut tex_x = (wall_x * texture_manager.tex_width as f64) as i32;
        if (side == 0 && dir_x > 0.0) || (side == 1 && dir_y < 0.0) {
            tex_x = texture_manager.tex_width as i32 - tex_x - 1;
        }

        // How much to increase the texture coordinate per screen pixel
        let step = (texture_manager.tex_height as f32 / line_height as f32) as f32;
        // Starting texture coordinate
        let mut tex_pos =
            (draw_start as i32 - screen.height as i32 / 2 + line_height / 2) as f32 * step;

        for y in draw_start..draw_end {
            let tex_y = (tex_pos as usize) & (texture_manager.tex_height - 1);
            tex_pos += step;
            let mut color = texture_manager.textures[tex_num as usize]
                [texture_manager.tex_height * tex_y + tex_x as usize];
            if side == 1 {
                color = (color >> 1) & 8355711; // Darken y-sides
            }
            let r = ((color >> 16) & 0xFF) as u8;
            let g = ((color >> 8) & 0xFF) as u8;
            let b = (color & 0xFF) as u8;
            image.draw_pixel(x as i32, y, raylib::prelude::Color::new(r, g, b, 255));
        }
        sprite_manager.z_buffer[x] = perp_wall_dist;
    }
}

// dda
fn get_coll(dir_x: f64, dir_y: f64, player: &Player, world_map: &WorldMap) -> (i32, f64, i32, i32) {
    let mut map_x: i32 = player.pos_x as i32; // box of map we are in
    let mut map_y: i32 = player.pos_y as i32;

    // dist to next x or y
    let delta_dist_x = if dir_x == 0.0 {
        1e30
    } else {
        (1.0 / dir_x).abs()
    };
    let delta_dist_y = if dir_y == 0.0 {
        1e30
    } else {
        (1.0 / dir_y).abs()
    };
    let perp_wall_dist: f64;

    let step_x: i32; // what direction to step
    let step_y: i32;
    let mut side_dist_x: f64;
    let mut side_dist_y: f64;

    // initial x and y state for dda
    if dir_x < 0.0 {
        step_x = -1;
        side_dist_x = (player.pos_x - map_x as f64) * delta_dist_x;
    } else {
        step_x = 1;
        side_dist_x = (map_x as f64 + 1.0 - player.pos_x) * delta_dist_x;
    }

    if dir_y < 0.0 {
        step_y = -1;
        side_dist_y = (player.pos_y - map_y as f64) * delta_dist_y;
    } else {
        step_y = 1;
        side_dist_y = (map_y as f64 + 1.0 - player.pos_y) * delta_dist_y;
    }
    // dda
    let mut hit = 0;
    let mut side: i32 = 0; // default value

    while hit == 0 {
        //jump to next map square, either in x-direction, or in y-direction
        if side_dist_x < side_dist_y {
            side_dist_x += delta_dist_x;
            map_x += step_x;
            side = 0;
        } else {
            side_dist_y += delta_dist_y;
            map_y += step_y;
            side = 1;
        }
        //Check if ray has hit a wall
        if world_map[map_x as usize][map_y as usize] > 0 {
            hit = 1
        };
    }

    if side == 0 {
        perp_wall_dist = side_dist_x - delta_dist_x;
    } else {
        perp_wall_dist = side_dist_y - delta_dist_y;
    }

    return (side, perp_wall_dist, map_x, map_y);
}
