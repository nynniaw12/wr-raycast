use rl_raycast::{
    backends::{wgpu::backend::WGPUBackend, GameBackend},
    cast::{floor::floor_cast, sprites::sprite_cast, walls::wall_cast},
    core::{screen::Screen, state::GameState},
    input::{handler::handle_input, movement::handle_movement},
};

pub fn main() {
    let screen = Screen::default();
    let mut game_state = GameState::default();
    let mut backend = WGPUBackend::new(screen, "Raycast".to_string());

    pollster::block_on(backend.run(|draw_pixel, inputs, frame_time| {
        handle_input(
            &inputs,
            &mut game_state.player,
            &game_state.world_map,
            &[handle_movement],
            frame_time,
        );
        floor_cast(&mut game_state, draw_pixel);
        wall_cast(&mut game_state, draw_pixel);
        sprite_cast(&mut game_state, draw_pixel);
    }));
}
