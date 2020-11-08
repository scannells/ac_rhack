use std::thread;
use std::time::Duration;

extern crate lpmanipulator;
use lpmanipulator::{ProcessErrors, Process};

extern crate ctor;
use ctor::ctor;

mod players;
use players::{Player, Enemy};

/// The main struct containing all the information and subcomponents of this hack
struct Ac_Hack {
    /// This is the player we are playing as
    player: Player
}


impl Ac_Hack {
    fn new() -> Self {
        // get a handle to the current process
        let process = Process::current().unwrap();

        // load the Self, the player we are playing as
        let player = Player::player1(&process);

        Ac_Hack {
            player: player
            
        }
    }


    fn run() {
        // This will initialize everything there is
        let mut hack = Self::new();

        loop {
            hack.player.set_health(1337);
            hack.player.set_ammo(1337);
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