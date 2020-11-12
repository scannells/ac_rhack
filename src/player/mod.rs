
mod godmode;
pub use godmode::GodMode;

mod infiniteammo;
pub use infiniteammo::InfiniteAmmo;
use crate::InternalMemory;
use crate::util::{game_base, Vec3};

/// offset to the player1 pointer from the base of the loaded game
const PLAYER1_OFF: usize = 0x128328;

/// offset to the vector of player pointers from the base of the loaded game
const PLAYERS_OFF: usize = 0x128330;

// there can be only 32 players in a match
const MAX_PLAYERS: usize = 32;

// offsets from the playerent to fields we want to read / write

/// offset from the player base to the team a player is in
const TEAM_OFF: usize = 0x344;

/// offset from the player base to the state (alive, dead etc.) a player has
const STATE_OFF: usize = 0x86;

/// offset to the player position
const PLAYER_POS_OFF: usize = 0x8;

// this value represents a living player (used for aimbot / ESP)
const ALIVE_STATE: u8 = 0;

const PLAYER_VIEW_OFF: usize = 0x13745c;


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
	pub fn player1() -> Self  {
		// There is a global variable called "player1", which is a pointer
		// to the actual, dynamically allocated player struct.
		// In order to obtain the address of the player, just dereference the global pointer
		let ac_base  = game_base();
		let player1_ptr = ac_base + PLAYER1_OFF;
		let player1_base: u64 = InternalMemory::read(player1_ptr);

		Player {
			base: player1_base as usize
		}
	}


	/// Returns a vector of all other players in the lobby
	pub fn players() -> Vec<Self> {
		let players_base = game_base() + PLAYERS_OFF;

		let vec_of_players = unsafe {
            let vec_ptr = players_base as *const AcVector;
            *vec_ptr
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


	/// reads the position of a player and returns it as 3D coordinates
	pub fn get_pos(&self) -> Vec3  {
		let mut head: [f32; 3] = [0.0; 3];
		for i in 0..3 {
			head[i] = InternalMemory::read(self.base + PLAYER_POS_OFF + i * 4);
		}
		Vec3::from(head)
	}

	pub fn distance_to(&self, other: &Player) -> f32 {
		let self_pos = self.get_pos();
		let other_pos = other.get_pos();

		Vec3::distance(self_pos, other_pos)
	}

	pub fn get_team(&self) -> i32 {
        InternalMemory::read(self.base + TEAM_OFF)
    }

	pub fn is_alive(&self) -> bool {
		InternalMemory::read::<u8>(self.base + STATE_OFF) == ALIVE_STATE
	}

	/// returns the point to the entity the user is looking at
	pub fn viewpoint() -> Vec3 {
		let mut view = [0.0 as f32; 3];
		for i in 0..3 {
			view[i] = InternalMemory::read(game_base() + PLAYER_VIEW_OFF);
		}

		Vec3::from(view)
	}

}
