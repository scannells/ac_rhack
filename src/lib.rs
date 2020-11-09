use std::thread;
use std::time::Duration;

extern crate lpmanipulator;
use lpmanipulator::{ProcessErrors, Process, Internal, MemoryManipulator};

extern crate ctor;
use ctor::ctor;

mod player;
use player::{Player, Enemy};

mod aimbot;
use aimbot::AimBot;

mod helpers;



/// The main struct containing all the information and subcomponents of this hack
struct AcHack {
    /// This is the player we are playing as
    player: Player,
    aimbot: AimBot,
}


impl AcHack {
    fn new() -> Self {
        // get a handle to the current process
        let process = Process::current().unwrap();

        AcHack {
            player: Player::new(&process),
            aimbot: AimBot::new(&process),
        }
    }


    fn run() {
        // This will initialize everything there is
        let mut hack = Self::new();

        // enable no recoil by default
        hack.aimbot.norecoil_spread.enable();

        hack.aimbot.autoshoot.enable();

        // enable infinite ammo
        hack.player.infinite_ammo.enable();

        // enable god mode
        hack.player.god_mode.enable();

        // set the ammo to a funny value
        hack.player.set_ammo(1337);


        hack.player.set_health(1337);

        let mut mem: Internal = Process::current().unwrap().get_mem_access().unwrap();
        loop {
            thread::sleep(Duration::from_secs(2));
            let enems = hack.aimbot.enemies();
            println!("returned a vector of {} enemies", enems.len() );

            let self_pos = hack.player.get_xyz();
            println!("self position = {} {} {}", self_pos[0], self_pos[1], self_pos[2]);
            let self_view = hack.player.get_view();
            println!("self view = {} {} {}", self_view[0], self_view[1], self_view[2]);
            for enem in enems.iter() {
                let pos = enem.get_pos(&mut mem);

                println!("enemy position = {} {} {}", pos[0], pos[1], pos[2]);

            }
            println!("\n\n");
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
        AcHack::run();
    });


}