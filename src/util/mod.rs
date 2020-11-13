

mod shellcode;
pub use shellcode::*;

mod math;
pub use math::*;

use crate::Process;

static mut GAME_BASE: Option<usize> = None;

/// returns the base address of the laoded AssaultCube process
pub fn game_base() -> usize {
    unsafe {
        if GAME_BASE.is_none() {
            let process = Process::current().expect("Failed to use /proc to obtain process information");
            GAME_BASE = Some(
                process.module("linux_64_client")
                    .expect("Could not find game module in current process")
                    .base
            );
        }
        GAME_BASE.unwrap()
    }
}

