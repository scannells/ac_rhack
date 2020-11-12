

mod gl_bindings;
use gl_bindings::*;

mod matrix;
use matrix::ViewMatrix;

use crate::process::{Process, Internal, MemoryManipulator};
use crate::{
    Enemy,
    Player
};

mod ebox;
use ebox::ESPBox;

const PLAYERS_OFF: usize = 0x128330;
const ENEMY_ESP_COLOR: [GLubyte; 3] = [252, 18 , 10];
const TEAM_ESP_COLOR: [GLubyte; 3] = [38, 217 , 50];


pub struct ESP {
    mem: Internal,
    enemies_base: usize,
    player: Player,

    draw_friendly: bool,
    view_matrix: ViewMatrix,
    esp_box: ESPBox,
}

impl ESP {

    pub fn new(process: &Process) -> Self {
        ESP {
            mem: process.get_mem_access().unwrap(),
            enemies_base: process.module("linux_64_client").unwrap().base + PLAYERS_OFF,
            player: Player::new(process),
            esp_box: ESPBox::new(ENEMY_ESP_COLOR, TEAM_ESP_COLOR),
            view_matrix: ViewMatrix::new(process),
            draw_friendly: true
        }
    }

    // switches the openGL mode into a 2D matrix and pushes the current state onto a stack
    // so that we can pop it later
    fn switch_to_2D(&self) -> [GLint; 4] {
        unsafe {
            // save the current state
            gl_bindings::glPushAttrib(GL_ALL_ATTRIB_BITS);

            // save the current matrix
            gl_bindings::glPushMatrix();

            // obtain and set the current viewport (position and dimensions of the window)
            // for the new matrix
            let mut viewport: [GLint; 4] = [0; 4];
            let mut viewport_ptr = &mut viewport[0] as *mut GLint;
            gl_bindings::glGetIntegerv(GL_VIEWPORT, viewport_ptr);
            gl_bindings::glViewport(0, 0, viewport[2], viewport[3]);

            // go into projection mode
            gl_bindings::glMatrixMode(GL_PROJECTION);

            // loads a blank matrix
            gl_bindings::glLoadIdentity();

            gl_bindings::glOrtho(0.0, viewport[2].into(), viewport[3].into(), 0.0, -1.0, 1.0);

            gl_bindings::glMatrixMode(GL_MODELVIEW);
            gl_bindings::glLoadIdentity();
            gl_bindings::glDisable(GL_DEPTH_TEST);

            viewport
        }
    }

    // restores the attributes before leaving the hook
    fn restore(&self) {
        unsafe {
            gl_bindings::glPopMatrix();
            gl_bindings::glPopAttrib();
        }
    }


    pub fn draw(&mut self) {
        // save the current GL state, switch to 2D mode and obtain the window dimenstions
        let viewport = self.switch_to_2D();

        // obtain a list of all bots
        let enemies = Enemy::all(self.enemies_base, &mut self.mem);



        for e in enemies.iter() {
            // filter out dead enemies
            if !e.is_alive(&mut self.mem) {
                continue
            }

            // filter out drawing team mates
            if !self.draw_friendly && e.team(&mut self.mem) == self.player.team() {
                continue
            }

            // draw ESP boxes for the remaining
            self.esp_box.draw_box(e, &mut self.player, viewport, &mut self.view_matrix, &mut self.mem)
        }

        self.restore();
    }
}