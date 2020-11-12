use crate::process::{Process};

mod godmode;
pub use godmode::GodMode;

mod infiniteammo;
pub use infiniteammo::InfiniteAmmo;
use crate::InternalMemory;

/// offset to the player1 pointer from the base of the loaded game
const PLAYER1_OFF: usize = 0x128328;

// there can be only 32 players in a match
const MAX_PLAYERS: usize = 32;

/// offsets from the playerent to fields we want to read / write
const HEALTH_OFF: usize = 0x110;
const AMMO_OFF: usize = 0x150;
const GUNSELECT_OFF: usize = 0x120;
const TEAM_OFF: usize = 0x344;
const STATE_OFF: usize = 0x86;
const PLAYER_POS_OFF: usize = 0x8;
const PLAYER_Y_OFF: usize = 0x8 + 0x8;
const PLAYERS_OFF: usize = 0x128330;

const ALIVE_STATE: u8 = 0;

const PLAYER_VIEW_OFF: usize = 0x13745c;

const PLAYER_ATTACKING_OFF: usize = 0x23c;

static mut PLAYERS_BASE: Option<usize> = None;
static mut PLAYER1_BASE: Option<usize> = None;

pub struct Player {
	pub base: usize,
}

// AssaultCube has a custom vector that holds pointers to enemies,
// which are also Players
#[repr(C)]
#[derive(Clone, Copy)]
struct AcVector {
    player_addresses: usize, // pointer to the buffer of pointers to the enemies
    capacity: i32,          // max size of the buffer
    elements: i32           // how many elements there actually are
}

impl Player {

	/// Creates a struct representing and giving access to the current player
	pub fn player1(process: &Process) -> Self  {
		// There is a global variable called "player1", which is a pointer
		// to the actual, dynamically allocated player struct.
		// In order to obtain the address of the player, just dereference the global pointer
		let ac_base  = process.module("linux_64_client").unwrap().base;
		let player1_ptr = ac_base + PLAYER1_OFF;
		let mut player1_base: u64 = InternalMemory::read(player1_ptr);

		Player {
			base: player1_base as usize
		}
	}


	fn init_addresses(process: &Process) {

	}

	/// Returns a vector of all other players in the lobby
	pub fn players(process: &Process) -> Vec<Self> {
		let players_base = process.module("linux_64_client").unwrap().base + PLAYERS_OFF;

		let vec_of_players = unsafe {
            let vec_ptr = players_base as *const AcVector;
            (*vec_ptr)
        };

		let mut players = Vec::with_capacity(MAX_PLAYERS);

        // fill the vector of enemies
        for i in 0..vec_of_players.elements {
            let player_addr: u64 = InternalMemory::read(vec_of_players.player_addresses + (i * 8) as usize);

            // sometimes pointers are NULL
            if player_addr == 0x0 {
                continue;
            }
            players.push(Player {
                base: player_addr as usize
            });
        }

		players
	}


	/// sets the health of the player to an arbitrary value
	pub fn set_health(&mut self, health: u32)  {
		InternalMemory::write(self.base + HEALTH_OFF, health);
	}


	/// sets the ammo off the current weapon
	pub fn set_ammo(&mut self, ammo: u32)  {
		// the playerstate keeps an index of the current weapon in the ammo array. It is an
		// int so multiply by 4
		let gun: u32 = InternalMemory::read(self.base + GUNSELECT_OFF);
		InternalMemory::write(self.base + AMMO_OFF + (gun * 4) as usize, ammo);
	}

	pub fn get_xyz(&self) -> [f32; 3]  {
		let mut head: [f32; 3] = [0.0; 3];
		for i in 0..3 {
			head[i] = InternalMemory::read(self.base + PLAYER_POS_OFF + i * 4);
		}
		head
	}


	pub fn get_team(&self) -> i32 {
        InternalMemory::read(self.base + TEAM_OFF)
    }

	pub fn is_alive(&self) -> bool {
		InternalMemory::read::<u8>(self.base + STATE_OFF) == ALIVE_STATE
	}

	pub fn shoot(&mut self) {
		InternalMemory::write(self.base + PLAYER_ATTACKING_OFF, 1 as u8);
	}

	pub fn stop_shoot(&mut self) {
		InternalMemory::write(self.base + PLAYER_ATTACKING_OFF, 0 as u8);
	}
}
