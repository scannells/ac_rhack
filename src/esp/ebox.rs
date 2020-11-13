
use super::*;

use crate::{
    Player,
    ViewMatrix
};


// Copied these values and this ESP scaling method from a tutorial on GuidedHacking (https://www.youtube.com/watch?v=kGDKQXgxIrY&t=1125s)
// they are used for scaling an ESP box
const VIRTUAL_SCREEN_WIDTH: i32 = 800;
const GAME_UNIT_MAGIC: usize = 400;
const PLAYER_HEIGHT: f32 = 7.25;
const PLAYER_WIDTH: f32 = 3.5;
const PLAYER_ASPECT_RATIO: f32 = PLAYER_HEIGHT / PLAYER_WIDTH;

/// Represents an ESP Box on the screen
pub struct ESPBox {
    enemy_color: [GLubyte; 3],
    team_color: [GLubyte; 3],
}

impl ESPBox {
    pub fn new(default_enemy_color: [GLubyte; 3], default_team_color: [GLubyte; 3]) -> Self {
        ESPBox {
            enemy_color: default_enemy_color,
            team_color: default_team_color,
        }
    }

    // scales an ESP box to the distance of the enemy and the width of the window
    fn scale(&self, distance: f32, window_width: i32) -> f32 {
        (GAME_UNIT_MAGIC as f32 / distance) * (window_width / VIRTUAL_SCREEN_WIDTH) as f32
    }

    // draws an ESP box relative to the player position
    pub fn draw_box(&self, client: &Player, player: &Player, window_dimensions: (GLint, GLint)) {

        // red for enemies, green for team mates
        let colors = if player.enemy_of(client) {self.enemy_color} else {self.team_color};
        let line_width: f32 = 0.75;

        // get the position of the enemy
        let client_pos = client.get_pos();

        // get the corresponding 2D coordinates
        let pos = ViewMatrix::new()
            .world_to_screen(client_pos, window_dimensions.0, window_dimensions.1);

        // if the enemy is behind us, don't bother drawing
        if !pos.0 {
            return
        }

        let x = pos.1;
        let y = pos.2;

        // get the distance and scale
        let distance = player.distance_to(client);
        let scale = self.scale(distance, window_dimensions.0);

        let x = x - scale;
        let y = y - scale;
        let width = scale * 2.0;
        let height = scale * PLAYER_ASPECT_RATIO * 2.0;


        // draw the ESP box
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