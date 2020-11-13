

mod gl_bindings;
use gl_bindings::*;


use crate::Player;

mod ebox;
use ebox::ESPBox;

const ENEMY_ESP_COLOR: [GLubyte; 3] = [252, 18 , 10];
const TEAM_ESP_COLOR: [GLubyte; 3] = [38, 217 , 50];


pub struct ESP {
    player: Player,
    esp_box: ESPBox,
}

impl ESP {

    pub fn new() -> Self {
        ESP {
            player: Player::player1(),
            esp_box: ESPBox::new(ENEMY_ESP_COLOR, TEAM_ESP_COLOR),
        }
    }

    // switches the openGL mode into a 2D matrix and pushes the current state onto a stack
    // so that we can pop it later. It also returns the current window dimensions
    fn switch_to_2d(&self) -> (GLint, GLint) {
        unsafe {
            // save the current state
            gl_bindings::glPushAttrib(GL_ALL_ATTRIB_BITS);

            // save the current matrix
            gl_bindings::glPushMatrix();

            // obtain and set the current viewport (position and dimensions of the window)
            // for the new matrix
            let mut viewport: [GLint; 4] = [0; 4];
            let viewport_ptr = &mut viewport[0] as *mut GLint;
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

            (viewport[2], viewport[3])
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
        let win_dimensions = self.switch_to_2d();

        // obtain a list of all bots
        let players = Player::players();

        for p in players.iter() {
            // filter out dead enemies
            if !p.is_alive() {
                continue
            }


            // draw ESP boxes for the remaining
            self.esp_box.draw_box(p, &self.player, win_dimensions)
        }

        self.restore();
    }

    // can be used by other modules to get information about the window
    pub fn window_dimensions() -> (i32, i32) {
        let mut viewport: [GLint; 4] = [0; 4];
        unsafe {
            let viewport_ptr = &mut viewport[0] as *mut GLint;
            gl_bindings::glGetIntegerv(GL_VIEWPORT, viewport_ptr);
        };
        (viewport[2], viewport[3])
    }
}