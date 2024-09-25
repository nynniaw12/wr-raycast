use super::pack::TexturePack;
use crate::{
    core::{
        player::Player,
        raycasting::{Collision, Ray},
        screen::Screen,
    },
    worldMap::WorldMap,
};
use raylib::prelude::*;
use std::cmp::{max, min};

// goes vertical
pub fn affine_tex_map_wall(
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
        let mut color = texture_pack.textures[tex_num as usize]
            [texture_pack.tex_height * tex_y + tex_x as usize];
        if collision.side == 1 {
            color = (color >> 1) & 8355711; // Darken y-sides
        }
        let r = ((color >> 16) & 0xFF) as u8;
        let g = ((color >> 8) & 0xFF) as u8;
        let b = (color & 0xFF) as u8;
        image.draw_pixel(x as i32, y, Color::new(r, g, b, 255));
    }
}

// floor and ceiling casting
// goes horizontal
pub fn affine_tex_map_fc(
    image: &mut Image,
    player: &Player,
    ray_0: &Ray,
    ray_1: &Ray,

    texture_pack: &TexturePack,
    screen: &Screen,

    y: usize,
    row_dist: f64,
) {
    // calculate the real world step vector we have to add for each x (parallel to camera plane)
    // adding step by step avoids multiplications with a weight in the inner loop
    let floor_step_x = row_dist * (ray_1.dir_x - ray_0.dir_x) / screen.width as f64;
    let floor_step_y = row_dist * (ray_1.dir_y - ray_0.dir_y) / screen.width as f64;
    // real world coordinates of the leftmost column. This will be updated as we step to the right.
    let mut floor_x = player.pos_x + row_dist * ray_0.dir_x;
    let mut floor_y = player.pos_y + row_dist * ray_0.dir_y;
    for x in 0..screen.width {
        // the cell coord is simply got from the integer parts of floorX and floorY
        let cell_x = floor_x as i32;
        let cell_y = floor_y as i32;

        // get the texture coordinate from the fractional part
        let tx = (texture_pack.tex_width as f64 * (floor_x - cell_x as f64)) as i32
            & (texture_pack.tex_width as i32 - 1);
        let ty = (texture_pack.tex_height as f64 * (floor_y - cell_y as f64)) as i32
            & (texture_pack.tex_height as i32 - 1);

        floor_x += floor_step_x;
        floor_y += floor_step_y;

        // choose texture and draw the pixel
        let floor_texture = 3;
        let ceiling_texture = 6;
        let mut color: u32;

        // floor
        color = texture_pack.textures[floor_texture]
            [(texture_pack.tex_width as i32 * ty + tx) as usize];
        color = (color >> 1) & 8355711; // make a bit darker

        let r = ((color >> 16) & 0xFF) as u8;
        let g = ((color >> 8) & 0xFF) as u8;
        let b = (color & 0xFF) as u8;
        image.draw_pixel(x as i32, y as i32, Color::new(r, g, b, 255));

        //ceiling (symmetrical, at screenHeight - y - 1 instead of y)
        color = texture_pack.textures[ceiling_texture]
            [(texture_pack.tex_width as i32 * ty + tx) as usize];
        color = (color >> 1) & 8355711; // make a bit darker

        let r = ((color >> 16) & 0xFF) as u8;
        let g = ((color >> 8) & 0xFF) as u8;
        let b = (color & 0xFF) as u8;
        image.draw_pixel(
            x as i32,
            (screen.height - y - 1) as i32,
            Color::new(r, g, b, 255),
        );
    }
}
