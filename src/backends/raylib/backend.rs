use crate::{backends::GameBackend, core::screen::Screen, input::handler::InputAction};
use raylib::prelude::*;

pub struct RaylibBackend {
    pub rl: RaylibHandle,
    pub thread: RaylibThread,
    pub image: Image,
}

impl GameBackend for RaylibBackend {
    fn new(screen: Screen, title: String) -> Self {
        let (rl, thread) = raylib::init()
            .size(screen.width as i32, screen.height as i32)
            .title(&title)
            .build();

        let image = Image::gen_image_color(
            screen.width as i32,
            screen.height as i32,
            raylib::prelude::Color::BLACK,
        );
        Self { rl, thread, image }
    }
    async fn run<F>(&mut self, mut game: F)
    where
        F: FnMut(&mut dyn FnMut(usize, usize, u32), &[InputAction], f64),
    {
        while !self.rl.window_should_close() {
            let mut actions = Vec::new();
            if self.rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT)
                || self.rl.is_key_down(KeyboardKey::KEY_RIGHT_SHIFT)
            {
                actions.push(InputAction::Sprint);
            }
            if self.rl.is_key_down(KeyboardKey::KEY_W) {
                actions.push(InputAction::MoveForward);
            }
            if self.rl.is_key_down(KeyboardKey::KEY_S) {
                actions.push(InputAction::MoveBackward);
            }
            if self.rl.is_key_down(KeyboardKey::KEY_A) {
                actions.push(InputAction::TurnLeft);
            }
            if self.rl.is_key_down(KeyboardKey::KEY_D) {
                actions.push(InputAction::TurnRight);
            }
            let frame_time = self.rl.get_frame_time();

            game(
                &mut |x, y, color| {
                    let r = ((color >> 16) & 0xFF) as u8;
                    let g = ((color >> 8) & 0xFF) as u8;
                    let b = (color & 0xFF) as u8;
                    self.image
                        .draw_pixel(x as i32, y as i32, Color::new(r, g, b, 255));
                },
                &actions,
                frame_time as f64,
            );

            // update screen
            let texture_2d = self
                .rl
                .load_texture_from_image(&self.thread, &self.image)
                .unwrap();
            let fps = self.rl.get_fps();
            let mut d = self.rl.begin_drawing(&self.thread);
            // clear
            d.clear_background(Color::BLACK);
            // textured
            d.draw_texture(&texture_2d, 0, 0, Color::WHITE);
            // FPS counter
            d.draw_text(&fps.to_string(), 12, 12, 20, Color::WHITE);
        }
    }
}
