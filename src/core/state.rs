use super::{player::Player, screen::Screen, sprites::SpriteManager, textures::TextureManager};
use crate::{
    backends::{wgpu::backend::WGPUBackend, GameBackend},
    cast::{floor::floor_cast, sprites::sprite_cast, walls::wall_cast},
    input::{handler::handle_input, movement::handle_movement},
    map::{
        sprites::{SPRITE_COUNT, SPRITE_MAP},
        world::{WorldMap, WORLD_MAP},
    },
};

pub struct GameState {
    pub player: Player,
    pub screen: Screen,
    pub sprite_manager: SpriteManager,
    pub texture_manager: TextureManager,
    pub world_map: WorldMap,
}

impl Default for GameState {
    fn default() -> Self {
        let player = Player::default();
        let screen = Screen::default();
        let texture_manager = TextureManager::default();

        // let texture_manager = TextureManager::load(
        //     "/Users/hasanaybarsari/Desktop/github/rl-raycast/src/assets",
        //     64,
        //     64,
        // )
        // .unwrap();
        let sprite_manager = SpriteManager::new(SPRITE_MAP.to_vec(), SPRITE_COUNT, &screen);
        Self {
            screen,
            player,
            sprite_manager,
            texture_manager,
            world_map: WORLD_MAP,
        }
    }
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    let screen = Screen::default();
    let mut game_state = GameState::default();
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
