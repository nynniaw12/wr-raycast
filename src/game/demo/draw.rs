use super::state::DemoGameState;

const BLACK: u32 = 0xFF000000;
pub fn draw_circle(state: &mut DemoGameState, draw_pixel: &mut dyn FnMut(usize, usize, u32)) {
    let color = &state.color;
    let width = state.screen.width as f32;
    let height = state.screen.height as f32;
    let center_x = width / 2.0;
    let center_y = height / 2.0;
    let radius = width.min(height) / 4.0;

    for y in 0..height as usize {
        for x in 0..width as usize {
            let dx = x as f32 - center_x;
            let dy = y as f32 - center_y;
            let distance = (dx * dx + dy * dy).sqrt();

            if distance <= radius {
                draw_pixel(x, y, color.get());
            } else {
                draw_pixel(x, y, BLACK);
            }
        }
    }
}

pub fn draw_square(state: &mut DemoGameState, draw_pixel: &mut dyn FnMut(usize, usize, u32)) {
    let color = &state.color;
    let width = state.screen.width as f32;
    let height = state.screen.height as f32;
    let center_x = width / 2.0;
    let center_y = height / 2.0;
    let half_side = width.min(height) / 4.0;

    for y in 0..height as usize {
        for x in 0..width as usize {
            let dx = x as f32 - center_x;
            let dy = y as f32 - center_y;

            if dx.abs() <= half_side && dy.abs() <= half_side {
                draw_pixel(x, y, color.get());
            } else {
                draw_pixel(x, y, BLACK);
            }
        }
    }
}

pub fn draw_triangle(state: &mut DemoGameState, draw_pixel: &mut dyn FnMut(usize, usize, u32)) {
    let color = &state.color;
    let width = state.screen.width as f32;
    let height = state.screen.height as f32;

    // verts
    let center_x = width / 2.0;
    let center_y = height / 2.0;
    let half_base = width.min(height) / 4.0;
    let height_triangle = half_base * (3.0f32).sqrt();

    let v0 = (center_x - half_base, center_y + height_triangle / 2.0); // bl
    let v1 = (center_x + half_base, center_y + height_triangle / 2.0); // br
    let v2 = (center_x, center_y - height_triangle / 2.0); // t

    // Function to calculate barycentric coordinates
    // helper
    fn barycentric_coords(
        x: f32,
        y: f32,
        v0: (f32, f32),
        v1: (f32, f32),
        v2: (f32, f32),
    ) -> (f32, f32, f32) {
        let denom = (v1.1 - v2.1) * (v0.0 - v2.0) + (v2.0 - v1.0) * (v0.1 - v2.1);
        let lambda1 = ((v1.1 - v2.1) * (x - v2.0) + (v2.0 - v1.0) * (y - v2.1)) / denom;
        let lambda2 = ((v2.1 - v0.1) * (x - v2.0) + (v0.0 - v2.0) * (y - v2.1)) / denom;
        let lambda3 = 1.0 - lambda1 - lambda2;
        (lambda1, lambda2, lambda3)
    }

    for y in 0..height as usize {
        for x in 0..width as usize {
            let xf = x as f32;
            let yf = y as f32;

            let (lambda1, lambda2, lambda3) = barycentric_coords(xf, yf, v0, v1, v2);

            // If all barycentric coordinates are between 0 and 1, the point is inside the triangle
            if lambda1 >= 0.0
                && lambda1 <= 1.0
                && lambda2 >= 0.0
                && lambda2 <= 1.0
                && lambda3 >= 0.0
                && lambda3 <= 1.0
            {
                draw_pixel(x, y, color.get());
            } else {
                draw_pixel(x, y, BLACK);
            }
        }
    }
}
