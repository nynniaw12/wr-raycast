#[derive(Debug, Clone, Copy)]
pub struct Screen {
    pub height: usize,
    pub width: usize,
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            width: 640,
            height: 480,
        }
    }
}
