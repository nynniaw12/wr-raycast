use super::load::load_textures;

#[derive(Debug, Clone)]
pub struct TextureManager {
    pub tex_width: usize,
    pub tex_height: usize,
    pub textures: Vec<Vec<u32>>,
}

impl Default for TextureManager {
    fn default() -> Self {
        Self {
            tex_height: 64,
            tex_width: 64,
            textures: load_textures(64, 64),
        }
    }
}
impl TextureManager {
    pub fn new(tex_width: usize, tex_height: usize, textures: Vec<Vec<u32>>) -> Self {
        Self {
            tex_height,
            tex_width,
            textures,
        }
    }
}
