pub mod backends;
pub mod core;
pub mod game;
pub mod input;

#[allow(unused_imports)]
use game::{demo::DemoGame, raycasting::RaycastingGame, Game};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    #[cfg(feature = "demo")]
    {
        DemoGame::start().await;
    }

    #[cfg(feature = "raycasting")]
    {
        RaycastingGame::start().await;
    }
}
