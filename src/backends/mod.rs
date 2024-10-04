use std::future::Future;

use crate::{core::screen::Screen, input::InputAction};

// #[cfg(not(target_arch = "wasm32"))]
// pub mod raylib;

pub mod wgpu;

pub trait GameBackend {
    fn new(screen: Screen, title: String) -> Self;
    fn run<F>(&mut self, game_loop: F) -> impl Future<Output = ()>
    where
        F: FnMut(&mut dyn FnMut(usize, usize, u32), &[InputAction], f64);
}
