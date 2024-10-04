use super::Game;
use crate::{
    backends::{wgpu::backend::WGPUBackend, GameBackend},
    core::screen::Screen,
};
use controls::{handle_input, movement::handle_movement};
use draw::{floor::floor_cast, sprites::sprite_cast, walls::wall_cast};
use state::RaycastingGameState;

pub mod controls;
pub mod draw;
pub mod map;
pub mod state;

pub struct RaycastingGame {}
impl Game for RaycastingGame {
    async fn start() {
        let screen = Screen::default();
        let mut game_state = RaycastingGameState::default();
        let mut backend = WGPUBackend::new(screen, "Raycast".to_string());

        backend
            .run(|draw_pixel, inputs, frame_time| {
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
            })
            .await;
    }
}
