use crate::{
    // backends::{
    //     wgpu::{
    //         backend::WGPUBackend,
    //         demo::{
    //             control::handle_controls,
    //             draw::{draw_circle, draw_square, draw_triangle},
    //         },
    //     },
    //     GameBackend,
    // },
    core::screen::Screen,
};

pub enum Shape {
    CIRCLE,
    TRIANGLE,
    SQUARE,
}

pub struct Color(u32);
impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color(((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
    }
    pub fn get(&self) -> u32 {
        self.0
    }

    pub fn get_components(&self) -> (u8, u8, u8) {
        let r = ((self.0 >> 16) & 0xFF) as u8;
        let g = ((self.0 >> 8) & 0xFF) as u8;
        let b = (self.0 & 0xFF) as u8;
        (r, g, b)
    }

    pub fn cycle(&mut self, amount: i32) {
        let (r, g, b) = self.get_components();
        let new_r = ((r as i32 + amount) % 256).abs() as u8;
        let new_g = ((g as i32 + amount / 2) % 256).abs() as u8;
        let new_b = ((b as i32 + amount / 3) % 256).abs() as u8;
        self.0 = ((new_r as u32) << 16) | ((new_g as u32) << 8) | (new_b as u32);
    }
}

pub struct DemoApp {
    pub shape: Shape,
    pub color: Color,
    pub screen: Screen,
}

impl DemoApp {
    pub fn new(screen: Screen) -> Self {
        Self {
            color: Color::new(255, 255, 255),
            shape: Shape::CIRCLE,
            screen,
        }
    }
    pub fn c(&mut self) {
        self.shape = Shape::CIRCLE;
    }

    pub fn t(&mut self) {
        self.shape = Shape::TRIANGLE;
    }

    pub fn s(&mut self) {
        self.shape = Shape::SQUARE;
    }
    pub fn up(&mut self) {
        self.color.cycle(25);
    }

    pub fn down(&mut self) {
        self.color.cycle(-25);
    }
}

// #[cfg(target_arch = "wasm32")]
// use wasm_bindgen::prelude::*;

// #[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
// pub async fn run() {
//     let screen = Screen::default();
//     let mut backend = WGPUBackend::new(screen, "Demo".to_string());
//     let mut game_state = DemoApp::new(screen);

//     backend
//         .run(|draw_pixel, inputs, _frame_time| {
//             handle_controls(inputs, &mut game_state);
//             match game_state.shape {
//                 Shape::CIRCLE => {
//                     draw_circle(&mut game_state, draw_pixel);
//                 }
//                 Shape::SQUARE => {
//                     draw_square(&mut game_state, draw_pixel);
//                 }
//                 Shape::TRIANGLE => {
//                     draw_triangle(&mut game_state, draw_pixel);
//                 }
//             }
//         })
//         .await;
// }
