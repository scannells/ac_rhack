use std::thread;
use std::time::Duration;

extern crate lpmanipulator;
use lpmanipulator::{ProcessErrors, Process};

extern crate ctor;
use ctor::ctor;

mod players;
use players::{Player, Enemy};

mod norecoil;
use norecoil::NoRecoilSpread;

mod infiniteammo;
use infiniteammo::InfiniteAmmo;

/// The main struct containing all the information and subcomponents of this hack
struct Ac_Hack {
    /// This is the player we are playing as
    player: Player,
    norecoil: NoRecoilSpread,
    infinite_ammo: InfiniteAmmo,
}


impl Ac_Hack {
    fn new() -> Self {
        // get a handle to the current process
        let process = Process::current().unwrap();

        Ac_Hack {
            player: Player::player1(&process),
            norecoil: NoRecoilSpread::new(&process),
            infinite_ammo: InfiniteAmmo::new(&process)
        }
    }


    fn run() {
        // This will initialize everything there is
        let mut hack = Self::new();

        // enable no recoil by default
        hack.norecoil.enable();

        // enable infinite ammo
        hack.infinite_ammo.enable();

        // set the ammo to a funny value
        hack.player.set_ammo(1337);

        loop {
            hack.player.set_health(1337);
        }
    }
}

#[ctor]
fn load() {

    // Check if the current process has a linux_64_client module, otherwise don't load the cheat here
    let process = Process::current().unwrap();
    if let Err(e) = process.module("linux_64_client") {
        return;
    }

    println!("Successfully loaded the hack into the game...");


    // Create a new thread in which this hack will run in
    thread::spawn(|| {
        println!("Creating hack thread in game process...");

        // Wait around 5 seconds to let the game actually load so that pointers are valid.
        thread::sleep(Duration::from_secs(5));


        // Load the cheat and run it. We won't return from here
        Ac_Hack::run();
    });


}