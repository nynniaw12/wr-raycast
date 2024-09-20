use crate::worldMap::WorldMap;

use super::player::Player;

#[derive(Debug, Clone, Copy)]
pub struct Collision {
    pub perp_wall_dist: f64,
    pub side: i32,
    pub map_x: i32,
    pub map_y: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub dir_y: f64,
    pub dir_x: f64,
}

/* camera_x is the normalized position of camera relative to screen width -1 to 1
 * where 0 is the center of screen
 *
 * player is the player struct with its fov vectors
 *
 * uses dda
 */
impl Ray {
    pub fn new(player: &Player, screen_width: usize, x: usize) -> Self {
        // x pos of camera in camera plane
        let camera_x: f64 = ((2.0 * x as f64) / screen_width as f64) - 1.0;
        // x pos of camera in camera plane
        let dir_x: f64 = player.dir_x + player.plane_x * camera_x; // get an angled ray vector
        let dir_y: f64 = player.dir_y + player.plane_y * camera_x;
        Ray { dir_x, dir_y }
    }
    pub fn cast(&self, player: &Player, world_map: &WorldMap) -> Collision {
        let mut map_x: i32 = player.pos_x as i32; // box of map we are in
        let mut map_y: i32 = player.pos_y as i32;

        // dist to next x or y
        let delta_dist_x = if self.dir_x == 0.0 {
            1e30
        } else {
            (1.0 / self.dir_x).abs()
        };
        let delta_dist_y = if self.dir_y == 0.0 {
            1e30
        } else {
            (1.0 / self.dir_y).abs()
        };
        let perp_wall_dist: f64;

        let step_x: i32; // what direction to step
        let step_y: i32;
        let mut side_dist_x: f64;
        let mut side_dist_y: f64;

        // initial x and y state for dda
        if self.dir_x < 0.0 {
            step_x = -1;
            side_dist_x = (player.pos_x - map_x as f64) * delta_dist_x;
        } else {
            step_x = 1;
            side_dist_x = (map_x as f64 + 1.0 - player.pos_x) * delta_dist_x;
        }

        if self.dir_y < 0.0 {
            step_y = -1;
            side_dist_y = (player.pos_y - map_y as f64) * delta_dist_y;
        } else {
            step_y = 1;
            side_dist_y = (map_y as f64 + 1.0 - player.pos_y) * delta_dist_y;
        }
        // dda
        let mut hit = 0;
        let mut side: i32 = 0; // default value

        while hit == 0 {
            //jump to next map square, either in x-direction, or in y-direction
            if side_dist_x < side_dist_y {
                side_dist_x += delta_dist_x;
                map_x += step_x;
                side = 0;
            } else {
                side_dist_y += delta_dist_y;
                map_y += step_y;
                side = 1;
            }
            //Check if ray has hit a wall
            if world_map[map_x as usize][map_y as usize] > 0 {
                hit = 1
            };
        }

        if side == 0 {
            perp_wall_dist = side_dist_x - delta_dist_x;
        } else {
            perp_wall_dist = side_dist_y - delta_dist_y;
        }

        return Collision {
            side,
            perp_wall_dist,
            map_x,
            map_y,
        };
    }
}
