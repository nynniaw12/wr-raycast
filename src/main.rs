use raylib::prelude::*;
use rl_raycast::{
    core::state::GameState,
    cast::{floor::floor_cast, sprites::sprite_cast, walls::wall_cast},
    input::{handler::handle_input, movement::handle_movement},
};

pub fn main() {
    let mut game_state = GameState::default();

    // init screen
    let (mut rl, thread) = raylib::init()
        .size(
            game_state.screen.width as i32,
            game_state.screen.height as i32,
        )
        .title("Raycast")
        .build();

    while !rl.window_should_close() {
        handle_input(
            &rl,
            &mut game_state.player,
            &game_state.world_map,
            &[handle_movement],
        );
        // clear buffer
        game_state.image.clear_background(&Color::BLACK);

        // use affine texture mapping to create buffer
        floor_cast(&mut game_state);
        wall_cast(&mut game_state);
        sprite_cast(&mut game_state);

        // update screen
        let texture_2d = rl
            .load_texture_from_image(&thread, &game_state.image)
            .unwrap();
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

#[no_mangle]
pub extern "C" fn _start() {
    main();
}
