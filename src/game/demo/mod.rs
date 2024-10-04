use controls::handle_controls;
use draw::{draw_circle, draw_square, draw_triangle};
use state::{DemoGameState, Shape};

use crate::{
    backends::{wgpu::backend::WGPUBackend, GameBackend},
    core::screen::Screen,
};

use super::Game;

pub mod controls;
pub mod draw;
pub mod state;

pub struct DemoGame {}

impl Game for DemoGame {
    async fn start() {
        let screen = Screen::default();
        let mut backend = WGPUBackend::new(screen, "Demo".to_string());
        let mut game_state = DemoGameState::new(screen);

        backend
            .run(|draw_pixel, inputs, _frame_time| {
                handle_controls(inputs, &mut game_state);
                match game_state.shape {
                    Shape::CIRCLE => {
                        draw_circle(&mut game_state, draw_pixel);
                    }
                    Shape::SQUARE => {
                        draw_square(&mut game_state, draw_pixel);
                    }
                    Shape::TRIANGLE => {
                        draw_triangle(&mut game_state, draw_pixel);
                    }
                }
            })
            .await;
    }
}
