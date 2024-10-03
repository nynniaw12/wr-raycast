use crate::core::state::GameState;
use std::cmp::{max, min};

pub fn sprite_cast(
    GameState {
        sprite_manager,
        player,
        texture_manager,
        screen,
        ..
    }: &mut GameState,
    draw_pixel: &mut dyn FnMut(usize, usize, u32),
) {
    // SPRITE CASTING
    for i in 0..sprite_manager.sprite_count {
        sprite_manager.sprite_order[i] = i as i32;
        sprite_manager.sprite_dist[i] = (player.pos_x - sprite_manager.sprites[i].x)
            * (player.pos_x - sprite_manager.sprites[i].x)
            + (player.pos_y - sprite_manager.sprites[i].y)
                * (player.pos_y - sprite_manager.sprites[i].y)
        //sqrt not taken, unneeded
    }
    sprite_manager.sort();

    for i in 0..sprite_manager.sprite_count {
        // sprite relative to camera
        let sprite_x: f64 =
            sprite_manager.sprites[sprite_manager.sprite_order[i] as usize].x - player.pos_x;
        let sprite_y: f64 =
            sprite_manager.sprites[sprite_manager.sprite_order[i] as usize].y - player.pos_y;
        // inverse matrix transformation
        let inv_det = 1.0 / (player.plane_x * player.dir_y - player.dir_x * player.plane_y);
        let transform_x: f64 = inv_det * (player.dir_y * sprite_x - player.dir_x * sprite_y);
        let transform_y: f64 = inv_det * (-player.plane_y * sprite_x + player.plane_x * sprite_y);

        let sprite_screen_x: i32 =
            ((screen.width as f64 / 2.0) * (1.0 + transform_x / transform_y)) as i32;
        let sprite_height = ((screen.height as f64 / transform_y) as i32).abs();

        //calculate lowest and highest pixel to fill in current stripe
        let draw_start_y = max(-sprite_height / 2 + screen.height as i32 / 2, 0);
        let draw_end_y = min(
            sprite_height / 2 + screen.height as i32 / 2,
            (screen.height - 1) as i32,
        );
        //calculate width of the sprite
        let sprite_width = ((screen.height as f64 / transform_y) as i32).abs();

        let draw_start_x = max(-sprite_width / 2 + sprite_screen_x, 0);
        let draw_end_x = min(
            sprite_width / 2 + sprite_screen_x,
            (screen.width - 1) as i32,
        );

        //loop through every vertical stripe of the sprite on screen
        for stripe in draw_start_x..draw_end_x {
            let tex_x = ((256.0
                * (stripe as f64 - (-sprite_width as f64 / 2.0 + sprite_screen_x as f64))
                * texture_manager.tex_width as f64
                / sprite_width as f64)
                / 256.0) as i32;
            //the conditions in the if are:
            //1) it's in front of camera plane so you don't see things behind you
            //2) it's on the screen (left)
            //3) it's on the screen (right)
            //4) ZBuffer, with perpendicular distance
            if (transform_y > 0.0)
                && (stripe > 0)
                && ((stripe as i32) < (screen.width as i32))
                && transform_y < sprite_manager.z_buffer[stripe as usize]
            {
                for y in draw_start_y..draw_end_y
                //for every pixel of the current stripe
                {
                    let d = (y) * 256 - screen.height as i32 * 128 + sprite_height * 128; //256 and 128 factors to avoid floats
                    let tex_y = ((d * texture_manager.tex_height as i32) / sprite_height) / 256;
                    let color = texture_manager.textures[sprite_manager.sprites
                        [sprite_manager.sprite_order[i] as usize]
                        .tex as usize]
                        [(texture_manager.tex_width as i32 * tex_y + tex_x) as usize];
                    if (color & 0x00FFFFFF) != 0 {
                        draw_pixel(
                            stripe as usize,
                            y as usize,
                            color, //paint pixel if it isn't black, black is the invisible color
                        );
                    }
                }
            }
        }
    }
}
