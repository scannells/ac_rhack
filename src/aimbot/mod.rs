extern crate lpmanipulator;
use lpmanipulator::{Process, Internal, MemoryManipulator};

mod norecoil;
use norecoil::NoRecoilSpread;

pub struct AimBot {
    pub norecoil_spread: NoRecoilSpread,
}

impl AimBot {
    pub fn new(process: &Process) -> AimBot {
        AimBot {
            norecoil_spread: NoRecoilSpread::new(process),
        }
    }
}