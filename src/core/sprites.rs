use super::screen::Screen;

#[derive(Clone)]
pub struct Sprite {
    pub x: f64,
    pub y: f64,
    pub tex: i32,
}
impl Sprite {
    pub const fn new(x: f64, y: f64, tex: i32) -> Self {
        return Self { x, y, tex };
    }
}

pub struct SpriteManager {
    pub sprites: Vec<Sprite>,
    pub sprite_count: usize,
    pub z_buffer: Vec<f64>,
    pub sprite_order: Vec<i32>,
    pub sprite_dist: Vec<f64>,
}

impl SpriteManager {
    pub fn new(sprites: Vec<Sprite>, sprite_count: usize, screen: &Screen) -> Self {
        // 1D ZBuffer
        let z_buffer: Vec<f64> = vec![0.0; screen.width];

        // sorting of the sprites
        let sprite_order: Vec<i32> = vec![0; sprite_count];
        let sprite_dist: Vec<f64> = vec![0.0; sprite_count];
        Self {
            sprites,
            sprite_count,
            z_buffer,
            sprite_order,
            sprite_dist,
        }
    }
    pub fn sort(&mut self) {
        let mut sprites: Vec<(f64, i32)> = Vec::with_capacity(self.sprite_count as usize);
        for i in 0..self.sprite_count as usize {
            sprites.push((self.sprite_dist[i], self.sprite_order[i]));
        }

        // Sort sprites by self.sprite_distance in ascending self.sprite_order
        sprites.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

        // Restore in reverse self.sprite_order to go from farthest to nearest
        for i in 0..self.sprite_count as usize {
            let index = self.sprite_count as usize - i - 1;
            self.sprite_dist[i] = sprites[index].0;
            self.sprite_order[i] = sprites[index].1;
        }
    }
}
