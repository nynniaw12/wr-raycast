pub fn generate_textures(tex_width: usize, tex_height: usize) -> Vec<Vec<u32>> {
    let mut textures: Vec<Vec<u32>> = vec![vec![0; tex_width * tex_height]; 11];

    for x in 0..tex_width {
        for y in 0..tex_height {
            let xorcolor = ((x * 256 / tex_width) ^ (y * 256 / tex_height)) as u32;
            let ycolor = (y * 256 / tex_height) as u32;
            let xycolor = (y * 128 / tex_height + x * 128 / tex_width) as u32;
            textures[0][tex_width * y + x] =
                0xFF0000 * if x != y && x != tex_width - y { 1 } else { 0 };
            textures[1][tex_width * y + x] = xycolor + (xycolor << 8) + (xycolor << 16);
            textures[2][tex_width * y + x] = (xycolor << 8) + (xycolor << 16);
            textures[3][tex_width * y + x] = xorcolor + (xorcolor << 8) + (xorcolor << 16);
            textures[4][tex_width * y + x] = xorcolor << 8;
            textures[5][tex_width * y + x] =
                0xC00000 * if x % 16 == 0 && y % 16 == 0 { 1 } else { 0 };
            textures[6][tex_width * y + x] = ycolor << 16;
            textures[7][tex_width * y + x] = 0x808080;

            textures[8][tex_width * y + x] = 0x808080;
            textures[9][tex_width * y + x] = 0x808080;
            textures[10][tex_width * y + x] = 0x808080;
        }
    }

    return textures;
}
