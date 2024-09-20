use raylib::prelude::*;
use rl_raycast::{
    core::{player::Player, raycasting::Ray, screen::Screen},
    input::{handler::handle_input, movement::handle_movement},
    textures::{draw::affine_tex_map, pack::TexturePack},
    worldMap::WORLD_MAP,
};

fn main() {
    let mut player = Player::default();
    let screen = Screen::default();
    let texture_pack = TexturePack::load("/Users/hasanaybarsari/Desktop/github/rl-raycast/src/assets", 64, 64).unwrap();

    // init screen
    let (mut rl, thread) = raylib::init()
        .size(screen.width as i32, screen.height as i32)
        .title("Raycast")
        .build();

    let mut image = Image::gen_image_color(screen.width as i32, screen.height as i32, Color::BLACK);

    while !rl.window_should_close() {
        handle_input(&rl, &mut player, &WORLD_MAP, &[handle_movement]);
        // clear buffer
        image.clear_background(&Color::BLACK);

        for x in 0..screen.width {
            let ray = Ray::new(&player, screen.width, x);
            let collision = ray.cast(&player, &WORLD_MAP);
            // use affine texture mapping to create buffer
            affine_tex_map(
                &mut image,
                &collision,
                &player,
                &ray,
                &WORLD_MAP,
                &texture_pack,
                x,
                screen.height,
            );
        }

        let texture_2d = rl.load_texture_from_image(&thread, &image).unwrap();
        let fps = rl.get_fps();
        let mut d = rl.begin_drawing(&thread);
        // clear
        d.clear_background(Color::BLACK);
        // textured
        d.draw_texture(&texture_2d, 0, 0, Color::WHITE);
        // FPS counter
        d.draw_text(&fps.to_string(), 12, 12, 20, Color::WHITE);
    }
}
