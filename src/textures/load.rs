use image::{GenericImageView, Pixel};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn list_textures(path: &str) -> io::Result<Vec<String>> {
    let mut png_files = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("png") {
            if let Some(file_name) = path.file_name() {
                if let Some(file_str) = file_name.to_str() {
                    png_files.push(file_str.to_string());
                }
            }
        }
    }
    Ok(png_files)
}

pub fn load_textures(base_path: &str, texture_files: &[String], tex_width: usize, tex_height: usize) -> Vec<Vec<u32>> {
    let mut textures: Vec<Vec<u32>> = Vec::new();
    for file in texture_files {
        let path = PathBuf::from(base_path).join(file);
        let path_str = path.to_str().unwrap_or("");
        match load_texture(path_str, tex_width, tex_height) {
            Ok(texture) => textures.push(texture),
            Err(e) => eprintln!("Failed to load texture {}: {}", path_str, e),
        }
    }
    textures
}

fn load_texture(
    path: &str,
    tex_width: usize,
    tex_height: usize,
) -> Result<Vec<u32>, image::ImageError> {
    let img = image::open(Path::new(path))?;
    let resized = img.resize_exact(
        tex_width as u32,
        tex_height as u32,
        image::imageops::FilterType::Nearest,
    );
    let mut texture: Vec<u32> = Vec::with_capacity(tex_width * tex_height);
    for pixel in resized.pixels() {
        let rgba = pixel.2.to_rgba();
        let r = rgba[0] as u32;
        let g = rgba[1] as u32;
        let b = rgba[2] as u32;
        let a = rgba[3] as u32;
        let color = (r << 16) | (g << 8) | b | (a << 24);
        texture.push(color);
    }
    Ok(texture)
}
