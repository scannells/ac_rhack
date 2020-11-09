extern crate lpmanipulator;
use lpmanipulator::{Process, Internal, MemoryManipulator};

mod norecoil;
use norecoil::NoRecoilSpread;

mod enemy;
use enemy::Enemy;

const PLAYERS_OFF: usize = 0x128330;

pub struct AimBot {
    pub norecoil_spread: NoRecoilSpread,
    enabled: bool,
    enemies_base: usize,
    mem: Internal,
}


impl AimBot {
    pub fn new(process: &Process) -> AimBot {
        AimBot {
            norecoil_spread: NoRecoilSpread::new(process),
            enabled: false,
            enemies_base: process.module("linux_64_client").unwrap().base + PLAYERS_OFF,
            mem: process.get_mem_access().unwrap()
        }
    }

    pub fn enable(mut self) -> Self {
        self.enabled = true;
        self
    }

    pub fn disable(mut self) {
        self.enabled = false;
    }

    pub fn enemies(&mut self) -> Vec<Enemy> {
        Enemy::all(self.enemies_base, &mut self.mem)
    }
}