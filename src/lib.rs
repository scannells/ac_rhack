/**
 * This hack is relatively simple. It is loaded into the AssaultCube process through
 * the LD_PRELOAD technique (e.g.) LD_PRELOAD=./hack.so ./assaultcube.sh in the main AC directory.
 * There is a constructor, which runs at load time. It is used to initialize the hack by
 *  - verifying this library is actually loaded into the game and not for example /bin/sh when
         launching AC through ./assaultcube.sh
 *  - finding offsets of code to patch
 *  - generating shellcode on the fly through nasm for hooks
 *  - prepares hooks
 *  - initialized the global AC_HACK variable
 *  - dynamically loads libSDL and obtains a pointer to the SDL_GL_SwapBuffers() function
 *  - spawns a new thread that will listen for keyboard bindings to change settings
 *
 *  By using the LD_PRELOAD technique, this hack hooks the SDL_GL_SwapBuffers() function.
 *  This function will then use the initialized, static variable AC_HACK to perform the logic
 *  it needs to do such as getting player positions, draw ESP boxes etc.
 *  The reason we use statics here is that we don't want to reload the entire hack
 *  for each frame
 */
use std::thread;
use std::time::Duration;

extern crate libloading;
extern crate ctor;
use ctor::ctor;


// include all the different sub modules of this hack as pub for the documentation
pub mod process;
pub mod player;
pub mod aimbot;
pub mod esp;
pub mod util;

// make all their symbols available to the other submodules through 'crate::'
use esp::*;
use aimbot::*;
use player::*;
use process::*;
use util::*;

use player::GodMode;
use crate::player::InfiniteAmmo;



static mut AC_HACK: Option<AcHack> = None;
static mut SDL_DYLIB: Option<libloading::Library> = None;

/// The main struct containing all the information and modules of this hack
struct AcHack {
    /// Exposes an interface to interact with the AC player struct
    pub player: Player,

    pub god_mode: GodMode,

    pub infinite_ammo: InfiniteAmmo,

    /// Used to configure the aimbot
    pub aimbot: AimBot,

    pub esp: ESP,
}


impl AcHack {
    /// Creates a new instance of the AcHack struct
    fn new() -> Self {
        // get a handle to the current process
        let player = Player::player1();
        AcHack {
            aimbot: AimBot::new(),
            esp: ESP::new(),
            god_mode: GodMode::new(),
            infinite_ammo: InfiniteAmmo::new(),
            player,
        }
    }

    /// initializes default settings and launches a new thread that will listen for keyboard
    /// bindings
    fn init() ->Self {
        let mut hack = Self::new();

        // all the following are default settings for this hack
        hack.aimbot.enable();
        hack.aimbot.norecoil_spread.toggle();
        hack.aimbot.enable_autoshoot();
        hack.infinite_ammo.toggle();
        hack.god_mode.toggle();

        hack
    }
}

/// This function is executed when the hack is loaded into the game
/// it is used to initialize the hack, launch a new thread that listens for keyboard bindings etc
#[ctor]
fn load() {

    // Check if the current process has a linux_64_client module (the main AC binary)
    // otherwise don't load the cheat here
    let process = Process::current().expect("Could not use /proc to obtain process information");
    if let Err(_e) = process.module("linux_64_client") {
        return;
    }

    // load libSDL dynamically by finding the module it is loaded at, get it's path and
    // use the libloading crate to dynamically load a pointer to the real SDL_GL_SwapBuffers()
    // function
    let mut found = false;
    let modules = process.modules().expect("Could not parse the loaded modules");
    for module_name in modules.keys() {
        if module_name.contains("libSDL-1.2") {
            unsafe {
                SDL_DYLIB = Some(
                    libloading::Library::new(module_name)
                        .expect("Could not load libSDL")
                )
            };

            found = true;
        }
    }

    if !found {
        panic!("Could not find libSDL-1.2 in current process");
    }

    // let the user know we are loaded
    println!("Successfully loaded the hack into the game...");
    println!("Waiting 5 seconds for the game to initialize it self before touching anything.");


    // Wait 5 seconds in a new thread for the game to initialize
    // If we don't do this step, we might break something
    thread::spawn(|| {
        // Wait around 5 seconds to let the game actually load so that pointers are valid.
        thread::sleep(Duration::from_secs(5));

        // Load the cheat!
        unsafe {
            AC_HACK = Some(AcHack::init());
        }
    });
}


fn forward_to_orig_sdl_swap_buffers() -> i64 {
    // this function is always initialized as we panic in the loading function
    // if it can't be initialized
    unsafe {
        // verify that SDL_DYLIB has already been initialized
        let libsdl = &SDL_DYLIB;
        if !libsdl.is_some() {
            // in case it has not, just return  0. This will render a black screen
            // in the AssaultCube window
            return 0;
        }

        let orig_sdl_swap_buffers:
            libloading::Symbol<unsafe extern "C" fn() -> i64>
            = SDL_DYLIB
            .as_ref()
            .unwrap()
            .get(b"SDL_GL_SwapBuffers\0")
            .expect("Could not find SDL_GL_SwapBuffers() in libSDL");
        orig_sdl_swap_buffers()
    }
}

/// This is the "main" function of this cheat.
/// SDL_GL_SwapBuffers() is called by the game for each frame that is generated
/// for this reason we outsourced initialization to load time of this library and use a global
/// variable for the main AC structure
#[no_mangle]
pub extern "C" fn SDL_GL_SwapBuffers() -> i64 {

    // Rust falsely detects this as an unused mutable
    #![allow(unused_mut)]
    let hack = unsafe {
        &mut AC_HACK
    };

    // verify that the AC_HACK has been loaded and initialized already
    // otherwise just render the frame
    if !hack.is_some() {
        return forward_to_orig_sdl_swap_buffers();
    }
    let mut hack = hack.as_mut().unwrap();

    // here comes the logic of the hack

    // handle ESP logic
    hack.esp.draw();

    // handle aimbot logic
    hack.aimbot.logic();

    // call the real SDL_GL_SwapBuffers() to render the frame and continue with the logic
    forward_to_orig_sdl_swap_buffers()
}




