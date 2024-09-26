#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub pos_x: f64,
    pub pos_y: f64,

    pub dir_x: f64,
    pub dir_y: f64,

    pub plane_x: f64,
    pub plane_y: f64,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            pos_x: 22.0,
            pos_y: 11.5,
            dir_x: -1.0,
            dir_y: 0.0,
            plane_x: 0.0,
            plane_y: 2.0/3.0 
        }
    }
}
