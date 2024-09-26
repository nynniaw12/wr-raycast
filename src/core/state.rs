use super::{player::Player, screen::Screen, sprites::SpriteManager, textures::TextureManager};
use crate::map::{
    sprites::{SPRITE_COUNT, SPRITE_MAP},
    world::{WorldMap, WORLD_MAP},
};

pub struct GameState {
    pub player: Player,
    pub screen: Screen,
    pub sprite_manager: SpriteManager,
    pub texture_manager: TextureManager,
    pub image: raylib::prelude::Image,
    pub world_map: WorldMap,
}

impl Default for GameState {
    fn default() -> Self {
        let player = Player::default();
        let screen = Screen::default();
        let texture_manager = TextureManager::load(
            "/Users/hasanaybarsari/Desktop/github/rl-raycast/src/assets",
            64,
            64,
        )
        .unwrap();
        let image = raylib::prelude::Image::gen_image_color(
            screen.width as i32,
            screen.height as i32,
            raylib::prelude::Color::BLACK,
        );
        let sprite_manager = SpriteManager::new(SPRITE_MAP.to_vec(), SPRITE_COUNT, &screen);
        Self {
            image,
            screen,
            player,
            sprite_manager,
            texture_manager,
            world_map: WORLD_MAP,
        }
    }
}
