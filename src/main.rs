use raylib::prelude::*;
use rl_raycast::{
    core::{player::Player, raycasting::Ray, screen::Screen},
    input::{handler::handle_input, movement::handle_movement},
    textures::{
        draw::{affine_tex_map_fc, affine_tex_map_wall},
        pack::TexturePack,
    },
    worldMap::WORLD_MAP,
};

fn main() {
    let mut player = Player::default();
    let screen = Screen::default();
    let texture_pack = TexturePack::load(
        "/Users/hasanaybarsari/Desktop/github/rl-raycast/src/assets",
        64,
        64,
    )
    .unwrap();

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

        for y in 0..screen.height {
            // leftmost ray
            let ray_dir_x_0: f64 = player.dir_x - player.plane_x;
            let ray_dir_y_0: f64 = player.dir_y - player.plane_y;

            // rightmost ray -- 0 is center
            let ray_dir_x_1: f64 = player.dir_x + player.plane_x;
            let ray_dir_y_1: f64 = player.dir_y + player.plane_y;

            let ray_0 = Ray::new(ray_dir_x_0, ray_dir_y_0);
            let ray_1 = Ray::new(ray_dir_x_1, ray_dir_y_1);
            let p = y as f64 - screen.height as f64 / 2.0; // current y position compared to center of screen
                                                           // Vertical position of the camera.
            let pos_z = 0.5 * screen.height as f64;
            // Horizontal distance from the camera to the floor for the current row.
            // 0.5 is the z position exactly in the middle between floor and ceiling.
            let row_dist = pos_z / p as f64;

            // also raycasting
            affine_tex_map_fc(
                &mut image,
                &player,
                &ray_0,
                &ray_1,
                &texture_pack,
                &screen,
                y,
                row_dist,
            );
        }
        for x in 0..screen.width {
            // x pos of camera in camera plane
            let camera_x: f64 = ((2.0 * x as f64) / screen.width as f64) - 1.0;
            // x pos of camera in camera plane
            let dir_x: f64 = player.dir_x + player.plane_x * camera_x; // get an angled ray vector
            let dir_y: f64 = player.dir_y + player.plane_y * camera_x;

            let ray = Ray::new(dir_x, dir_y);
            let collision = ray.wall_cast(&player, &WORLD_MAP);
            // use affine texture mapping to create buffer
            affine_tex_map_wall(
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
