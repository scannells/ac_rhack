use crate::process::{Process};

mod norecoil;
use norecoil::NoRecoilSpread;

mod autoshoot;
use autoshoot::AutoShoot;

use crate::player::Player;

const PLAYERS_OFF: usize = 0x128330;

pub struct AimBot {
    player: Player,
    pub norecoil_spread: NoRecoilSpread,
    pub autoshoot: AutoShoot,
    enabled: bool,
}


impl AimBot {
    pub fn new(process: &Process) -> AimBot {
        let mut player = Player::player1(process);
        AimBot {
            autoshoot: AutoShoot::new(process, player.base),
            player: player,
            norecoil_spread: NoRecoilSpread::new(process),
            enabled: false,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn toggle(&mut self) {
        if self.enabled {
            self.disable();
        } else {
            self.enable();
        }
    }
}