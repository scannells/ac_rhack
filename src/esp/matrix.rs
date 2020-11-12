/**
 * This file performs the mathematics of converting a 3D position into a 2D screen coordinate
 *
*/

use crate::InternalMemory;
use crate::util::{game_base, Vec3};

const PLAYER_VIEW_OFF: usize = 0x13745c;
const VIEW_MATRIX_OFF: usize = PLAYER_VIEW_OFF - 0x80;
//const VIEW_MATRIX_OFF: usize = 0x13739c;


pub struct ViewMatrix {
    base: usize,
}

impl ViewMatrix {
    pub fn new() -> Self {
        ViewMatrix {
            base: game_base() + VIEW_MATRIX_OFF,
        }
    }

    fn read_matrix(&self) -> [[f32; 4]; 4] {
        let mut ret = [[0.0f32, 0.0, 0.0, 0.0]; 4];
        let mut row = 0;
        let mut col = 0;
        for i in 0usize..16 {
            if col == 4 {
                col = 0;
                row += 1;
            }
            ret[row][col] = InternalMemory::read(self.base + i * 4);
            col += 1;
        }

        ret
    }

    pub fn world_to_screen(&self, pos: Vec3, width: i32, height: i32) -> (bool, f32, f32) {
        let matrix = self.read_matrix();

        let screen_x = (pos.x * matrix[0][0]) + (pos.y * matrix[1][0]) + (pos.z * matrix[2][0]) + matrix[3][0];
        let screen_y = (pos.x * matrix[0][1]) + (pos.y * matrix[1][1]) + (pos.z * matrix[2][1]) + matrix[3][1];
        let screen_z = (pos.x * matrix[0][2]) + (pos.y * matrix[1][2]) + (pos.z * matrix[2][2]) + matrix[3][2];
        let screen_w = (pos.x * matrix[0][3]) + (pos.y * matrix[1][3]) + (pos.z * matrix[2][3]) + matrix[3][3];

        if screen_w < 0.1 {
            return (false, 0.0, 0.0);
        }

        let mut ndc = Vec3::from([screen_x, screen_y, screen_z]);
        ndc.x = screen_x / screen_w;
        ndc.y = screen_y / screen_w;
        ndc.z = screen_z / screen_w;

        let x = (width as f32 / 2.0 * ndc.x) + (ndc.x + width as f32 / 2.0);
        let y = -(height as f32 / 2.0 * ndc.y) + (ndc.y + height as f32 / 2.0);

        (true, x ,y)
    }
}