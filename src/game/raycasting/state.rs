use crate::core::{
    player::Player, screen::Screen, sprites::SpriteManager, textures::manager::TextureManager,
};

use super::map::{
    sprites::{SPRITE_COUNT, SPRITE_MAP},
    world::{WorldMap, WORLD_MAP},
};

pub struct RaycastingGameState {
    pub player: Player,
    pub screen: Screen,
    pub sprite_manager: SpriteManager,
    pub texture_manager: TextureManager,
    pub world_map: WorldMap,
}

impl Default for RaycastingGameState {
    fn default() -> Self {
        let player = Player::default();
        let screen = Screen::default();
        let texture_manager = TextureManager::default();
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
