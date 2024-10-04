use crate::core::state::GameState;

pub fn floor_cast(
    GameState {
        player,
        screen,
        texture_manager,
        ..
    }: &mut GameState,
    draw_pixel: &mut dyn FnMut(usize, usize, u32),
) {
    for y in 0..screen.height {
        // leftmost ray
        let ray_0_dir_x: f64 = player.dir_x - player.plane_x;
        let ray_0_dir_y: f64 = player.dir_y - player.plane_y;

        // rightmost ray -- 0 is center
        let ray_1_dir_x: f64 = player.dir_x + player.plane_x;
        let ray_1_dir_y: f64 = player.dir_y + player.plane_y;

        let p = y as f64 - screen.height as f64 / 2.0; // current y position compared to center of screen
                                                       // Vertical position of the camera.
        let pos_z = 0.5 * screen.height as f64;
        // Horizontal distance from the camera to the floor for the current row.
        // 0.5 is the z position exactly in the middle between floor and ceiling.
        let row_dist = pos_z / p as f64;

        // calculate the real world step vector we have to add for each x (parallel to camera plane)
        // adding step by step avoids multiplications with a weight in the inner loop
        let floor_step_x = row_dist * (ray_1_dir_x - ray_0_dir_x) / screen.width as f64;
        let floor_step_y = row_dist * (ray_1_dir_y - ray_0_dir_y) / screen.width as f64;
        // real world coordinates of the leftmost column. This will be updated as we step to the right.
        let mut floor_x = player.pos_x + row_dist * ray_0_dir_x;
        let mut floor_y = player.pos_y + row_dist * ray_0_dir_y;
        for x in 0..screen.width {
            // the cell coord is simply got from the integer parts of floorX and floorY
            let cell_x = floor_x as i32;
            let cell_y = floor_y as i32;

            // get the texture coordinate from the fractional part
            let tx = (texture_manager.tex_width as f64 * (floor_x - cell_x as f64)) as i32
                & (texture_manager.tex_width as i32 - 1);
            let ty = (texture_manager.tex_height as f64 * (floor_y - cell_y as f64)) as i32
                & (texture_manager.tex_height as i32 - 1);

            floor_x += floor_step_x;
            floor_y += floor_step_y;

            // choose texture and draw the pixel
            let floor_texture = 3;
            let ceiling_texture = 6;
            let mut color: u32;

            // floor
            color = texture_manager.textures[floor_texture]
                [(texture_manager.tex_width as i32 * ty + tx) as usize];
            color = (color >> 1) & 8355711; // make a bit darker

            draw_pixel(x as usize, y as usize, color);

            // ceiling (symmetrical, at screenHeight - y - 1 instead of y)
            color = texture_manager.textures[ceiling_texture]
                [(texture_manager.tex_width as i32 * ty + tx) as usize];
            color = (color >> 1) & 8355711; // make a bit darker
            draw_pixel(x as usize, (screen.height - 1 - y) as usize, color);
        }
    }
}
