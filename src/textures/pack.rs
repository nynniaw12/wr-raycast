use crate::textures::load::list_textures;
use super::{generated::generate_textures, load::load_textures};
use std::io;

#[derive(Debug, Clone)]
pub struct TexturePack {
    pub tex_width: usize,
    pub tex_height: usize,
    pub textures: Vec<Vec<u32>>,
}

impl Default for TexturePack {
    fn default() -> Self {
        Self {
            tex_height: 64,
            tex_width: 64,
            textures: generate_textures(64, 64),
        }
    }
}
impl TexturePack {
    pub fn new(tex_width: usize, tex_height: usize, textures: Vec<Vec<u32>>) -> Self {
        Self {
            tex_height,
            tex_width,
            textures,
        }
    }
    pub fn load(path: &str, tex_width: usize, tex_height: usize) -> io::Result<Self> {
        let tex_paths = list_textures(path)?;
        let textures = load_textures(path, &tex_paths, tex_width, tex_height);

        Ok(Self {
            tex_width,
            tex_height,
            textures,
        })
    }
}
