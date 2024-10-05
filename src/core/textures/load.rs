use crate::ASSETS_DIR;
use image::{GenericImageView, Pixel};

pub fn load_textures(tex_height: usize, tex_width: usize) -> Vec<Vec<u32>> {
    let mut png_files = Vec::new();
    for file in ASSETS_DIR.files() {
        file.contents();
        if let Some(extension) = file.path().extension() {
            if extension == "png" {
                png_files.push(file);
            }
        }
    }
    png_files.sort_by_key(|file| {
        file.path()
            .file_name()
            .and_then(|name| name.to_str())
            .and_then(|s| s.split('_').next())
            .and_then(|part| part.parse::<usize>().ok())
    });

    let mut textures: Vec<Vec<u32>> = Vec::new();
    for file in png_files {
        let img_bytes = file.contents();
        let img = image::load_from_memory(img_bytes).unwrap();
        let resized = img.resize_exact(
            tex_width as u32,
            tex_height as u32,
            image::imageops::FilterType::Nearest,
        );
        println!("Loaded texture at {:?}", file.path());
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
        textures.push(texture);
    }

    return textures;
}
