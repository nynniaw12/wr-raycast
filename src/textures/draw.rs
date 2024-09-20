use crate::{
    core::{
        player::Player,
        raycasting::{Collision, Ray},
    },
    worldMap::WorldMap,
};
use raylib::prelude::*;
use std::cmp::{max, min};

use super::pack::TexturePack;

pub fn affine_tex_map(
    image: &mut Image,
    collision: &Collision,
    player: &Player,
    ray: &Ray,
    world_map: &WorldMap,
    texture_pack: &TexturePack,

    x: usize,
    screen_height: usize,
) {
    let line_height: i32 = (screen_height as f64 / collision.perp_wall_dist) as i32; // height of
    let draw_start: i32 = max(-line_height / 2 + screen_height as i32 / 2, 0); // screen
    let draw_end: i32 = min(
        screen_height as i32 - 1,
        line_height / 2 + screen_height as i32 / 2,
    );
    let tex_num = world_map[collision.map_x as usize][collision.map_y as usize] - 1; // enables texture 0
    let mut wall_x: f64 = if collision.side == 0 {
        player.pos_y + collision.perp_wall_dist * ray.dir_y
    } else {
        player.pos_x + collision.perp_wall_dist * ray.dir_x
    };

    wall_x -= wall_x.floor();
    let mut tex_x = (wall_x * texture_pack.tex_width as f64) as i32;
    if (collision.side == 0 && ray.dir_x > 0.0) || (collision.side == 1 && ray.dir_y < 0.0) {
        tex_x = texture_pack.tex_width as i32 - tex_x - 1;
    }

    // How much to increase the texture coordinate per screen pixel
    let step = (texture_pack.tex_height as f32 / line_height as f32) as f32;
    // Starting texture coordinate
    let mut tex_pos =
        (draw_start as i32 - screen_height as i32 / 2 + line_height / 2) as f32 * step;

    for y in draw_start..draw_end {
        let tex_y = (tex_pos as usize) & (texture_pack.tex_height - 1);
        tex_pos += step;
        let mut color = texture_pack.textures[tex_num as usize][texture_pack.tex_height * tex_y + tex_x as usize];
        if collision.side == 1 {
            color = (color >> 1) & 8355711; // Darken y-sides
        }
        let r = ((color >> 16) & 0xFF) as u8;
        let g = ((color >> 8) & 0xFF) as u8;
        let b = (color & 0xFF) as u8;
        image.draw_pixel(x as i32, y, Color::new(r, g, b, 255));
    }
}
