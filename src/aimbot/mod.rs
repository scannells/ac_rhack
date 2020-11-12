mod norecoil;
use norecoil::NoRecoilSpread;

mod autoshoot;
use autoshoot::AutoShoot;

use crate::player::Player;

pub struct AimBot {
    player: Player,
    pub norecoil_spread: NoRecoilSpread,
    pub autoshoot: AutoShoot,
    enabled: bool,
}


impl AimBot {
    pub fn new() -> AimBot {
        let player = Player::player1();
        AimBot {
            autoshoot: AutoShoot::new(player.base),
            player,
            norecoil_spread: NoRecoilSpread::new(),
            enabled: false,
        }
    }

    pub fn logic(&self) {
        // don't to anything if the aimbot is disabled
        if !self.enabled {
            return
        }

        // we need to get the closest enemy
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