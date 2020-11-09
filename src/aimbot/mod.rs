use crate::process::{Process, Internal, MemoryManipulator};

mod norecoil;
use norecoil::NoRecoilSpread;

mod enemy;
use enemy::Enemy;

mod autoshoot;
use autoshoot::AutoShoot;

use crate::player::Player;

const PLAYERS_OFF: usize = 0x128330;

pub struct AimBot {
    player: Player,
    pub norecoil_spread: NoRecoilSpread,
    pub autoshoot: AutoShoot,
    enabled: bool,
    enemies_base: usize,
    mem: Internal,
}


impl AimBot {
    pub fn new(process: &Process) -> AimBot {
        let mut player = Player::new(process);
        AimBot {
            autoshoot: AutoShoot::new(process, player.base),
            player: player,
            norecoil_spread: NoRecoilSpread::new(process),
            enabled: false,
            enemies_base: process.module("linux_64_client").unwrap().base + PLAYERS_OFF,
            mem: process.get_mem_access().unwrap()
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn enemies(&mut self) -> Vec<Enemy> {
        Enemy::all(self.enemies_base, &mut self.mem)
    }
}