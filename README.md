This cheat is the result of a side-project I did for fun. It is an Internal Hack for AssaultCube on Linux written in Rust.


## Features

* ESP
* Aimbot
* Autoshoot
* No Recoil
* No Spread
* Infinite Ammo
* Invincibility
* 1-shot-1-kill


## Disclaimer

This hack does not work in multiplayer on purpose and will panic if it is loaded into a non-local game.
The purpose of this cheat was not to ruin other people's fun but to simply play around with a GameCheat in
Rust and on Linux, as most cheats are either written for Windows or in C++ or both.

This software contains no Assault Cube intellectual property, and is not affiliated in any way. 

## Usage

### Building



### Loading the cheat

This cheat uses the Linux `LD_PRELOAD` technique to load the binary into the target process and to hook
`SDL_GL_SwapBuffers()`. 

After building the cheat, run the following command from root directory of this cheat:

```bash
LD_PRELOAD=./target/release/libac_rhack.so PATH/TO/AC/assaultcube.sh
```

### Target binary

I developed this hack on Ubuntu 18.04 on x86_64 architectures. 
At the time of writing, the game can be downloaded from https://assault.cubers.net/.
Within the root of the game directory, there will be an `assaultcube.sh` file that is used to launch 
the game. Internally, this script executes `bin/linux_64_client`. This hack was developed for this
binary and at the time of writing, these are the file hashes for it:

* **MD5**: `819c849fd087ed2c218a8dc660d0f389`
* **SHA-1**: `1d8c23acde3373c10b0ef0dd03ab63194a4537e6`

It matters to execute that version, as offsets will be invalidated by future updates.

