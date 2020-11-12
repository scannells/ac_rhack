
use super::*;

use crate::{
    Player,
    InternalMemory
};


// Copied these values and this ESP scaling method from a tutorial on GuidedHacking (https://www.youtube.com/watch?v=kGDKQXgxIrY&t=1125s)
const VIRTUAL_SCREEN_WIDTH: i32 = 800;
const GAME_UNIT_MAGIC: usize = 400;
const PLAYER_HEIGHT: f32 = 7.25;
const PLAYER_WIDTH: f32 = 2.5;
const EYE_HEIGHT: f32 = 6.5;
const PLAYER_ASPECT_RATIO: f32 = PLAYER_HEIGHT / PLAYER_WIDTH;

pub struct ESPBox {
    enabled: bool,
    team_esp: bool,
    enemy_color: [GLubyte; 3],
    team_color: [GLubyte; 3],
}

impl ESPBox {
    pub fn new(default_enemy_color: [GLubyte; 3], default_team_color: [GLubyte; 3]) -> Self {
        ESPBox {
            enabled: false,
            team_esp: true,
            enemy_color: default_enemy_color,
            team_color: default_team_color,
        }
    }


    // calculates the distance between to enemies
    fn distance(&self, player: &Player, client: &Player) -> f32 {
        let vector = {
            let player_pos = player.get_xyz();
            let enem_pos = client.get_xyz();
            let x = player_pos[0] - enem_pos[0];
            let y = player_pos[1] - enem_pos[1];
            let z = player_pos[2] - enem_pos[2];
            [x, y, z]
        };

        let magnitude = {
            f32::sqrt(
                vector[0].powi(2) +
                    vector[1].powi(2) +
                    vector[2].powi(2)
            )
        };

        magnitude
    }

    fn scale(&self, distance: f32, window_width: i32) -> f32 {
        (GAME_UNIT_MAGIC as f32 / distance) * (window_width / VIRTUAL_SCREEN_WIDTH) as f32
    }

    // draws an ESP box relative to the player position
    pub fn draw_box(&self, client: &Player, player: &Player, window_dimensions: (GLint, GLint), view_matrix: &ViewMatrix) {

        let colors = if client.get_team() != player.get_team() {self.enemy_color} else {self.team_color};
        let line_width: f32 = 0.5;

        // get the position of the enemy
        let client_pos = client.get_xyz();

        // get the corresponding 2D coordinates
        let pos = view_matrix.world_to_screen(client_pos, window_dimensions.0, window_dimensions.1);

        // if the enemy is behind us, don't bother drawing
        if !pos.0 {
            return
        }

        let x = pos.1;
        let y = pos.2;

        let distance = self.distance(player, client);
        let scale = self.scale(distance, window_dimensions.0);

        let x = x - scale;
        let y = y - scale;
        let width = scale * 2.0;
        let height = scale * PLAYER_ASPECT_RATIO * 2.0;



        unsafe {
            glLineWidth(line_width);
            glColor3ub(colors[0], colors[1], colors[2]);
            glBegin(GL_LINE_STRIP);
            glVertex2f(x, y);
            glVertex2f(x + width, y);
            glVertex2f(x + width, y + height);
            glVertex2f(x, y + height);
            glVertex2f(x, y);

            glEnd();
        }
    }
}