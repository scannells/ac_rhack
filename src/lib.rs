use std::thread;
use std::time::Duration;

extern crate lpmanipulator;
use lpmanipulator::{ProcessErrors, Process};

extern crate ctor;
use ctor::ctor;

mod player;
use player::{Player, Enemy};

mod norecoil;
use norecoil::NoRecoilSpread;

mod helpers;



/// The main struct containing all the information and subcomponents of this hack
struct AcHack {
    /// This is the player we are playing as
    player: Player,
    norecoil: NoRecoilSpread,
}


impl AcHack {
    fn new() -> Self {
        // get a handle to the current process
        let process = Process::current().unwrap();

        AcHack {
            player: Player::new(&process),
            norecoil: NoRecoilSpread::new(&process),
        }
    }


    fn run() {
        // This will initialize everything there is
        let mut hack = Self::new();

        // enable no recoil by default
        hack.norecoil.enable();

        // enable infinite ammo
        hack.player.infinite_ammo.enable();

        // enable god mode
        hack.player.god_mode.enable();

        // set the ammo to a funny value
        hack.player.set_ammo(1337);


        hack.player.set_health(1337);

        loop {}
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
        AcHack::run();
    });


}