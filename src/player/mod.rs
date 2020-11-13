
mod godmode;
pub use godmode::GodMode;

mod infiniteammo;
pub use infiniteammo::InfiniteAmmo;
use crate::{InternalMemory, ESP};
use crate::util::{game_base, Vec3, ViewMatrix};

/// offset to the player1 pointer from the base of the loaded game
const PLAYER1_OFF: usize = 0x128328;

/// offset to the vector of player pointers from the base of the loaded game
const PLAYERS_OFF: usize = 0x128330;

// there can be only 32 players in a match
const MAX_PLAYERS: usize = 32;


/// offset from the player base to the team field (int)
const TEAM_OFF: usize = 0x344;

/// offset from the player base to the state (alive, dead etc.) a player has (u8)
const STATE_OFF: usize = 0x86;

/// offset to the player position (an array of 3 32bit floats)
const PLAYER_POS_OFF: usize = 0x8;

const PLAYER_NEWPOS_OFF: usize = 0x38;
const PLAYER_EYEHEIGHT_OFF: usize = 0x60;

const PLAYER_ATTACKING_OFF: usize = 0x23c;

const GAMEMODE_OFF: usize = 0x128294;

// this value represents a living player (used for aimbot / ESP)
const ALIVE_STATE: u8 = 0;


enum GameModes {
	GmodeBotTeamdeathMatch = 7,
	GmodeBotDeathMatch = 8,
	GmodeBotOneShotOneKill = 12,
	GmodeBotPistolFrenzy = 18,
	GmodeBotlss = 19,
	GmodeBotSurvivor = 20,
	GmodeBotTeamOneShotOneKill = 21,
}

impl GameModes {
	fn from_i32(int: i32) -> GameModes {
		match int {
			7 => GameModes::GmodeBotTeamdeathMatch,
			8 => GameModes::GmodeBotDeathMatch,
			12 => GameModes::GmodeBotOneShotOneKill,
			18 => GameModes::GmodeBotPistolFrenzy,
			19 => GameModes::GmodeBotlss,
			20 => GameModes::GmodeBotSurvivor,
			21 => GameModes::GmodeBotTeamOneShotOneKill,
			_ => panic!("Unsupported Game mode")
		}
	}
}

pub struct Player {
	pub base: usize,
}

// AssaultCube has a custom vector that holds pointers to enemies,
// which are also Players. We use this struct to read the enemy player's positions
#[repr(C)]
#[derive(Clone, Copy)]
struct AcVector {
    player_addresses: usize, // pointer to the buffer of pointers to the enemies
    capacity: i32,           // max size of the buffer
    elements: i32            // how many elements there actually are
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

	fn get_eyeheight(&self) -> f32 {
		InternalMemory::read(self.base + PLAYER_EYEHEIGHT_OFF)
	}

	fn get_gamemode() -> GameModes {
		GameModes::from_i32(
			InternalMemory::read(game_base() + GAMEMODE_OFF)
		)
	}

	fn is_free_for_all() -> bool {
		let gamemode = Self::get_gamemode();
		match gamemode {
			GameModes::GmodeBotDeathMatch => true,
			GameModes::GmodeBotOneShotOneKill => true,
			GameModes::GmodeBotSurvivor => true,
			GameModes::GmodeBotlss => true,
			_ => false
		}
	}

	/// returns true if the two players are enemies
	pub fn enemy_of(&self, other: &Player) -> bool {
		// first, check the game mode against a list of game modes where
		// the team does not matter
		if Self::is_free_for_all() {
			return true;
		}

		self.get_team() != other.get_team()
	}

	/// returns the position a player will be located at in the next frame.
	/// This is needed for a reliable aimbot
	pub fn get_new_pos(&self) -> Vec3 {
		let mut foot: [f32; 3] = [0.0; 3];
		for i in 0..3 {
			foot[i] = InternalMemory::read(self.base + PLAYER_NEWPOS_OFF + i * 4);
		}
		let mut vec = Vec3::from(foot);
		vec.z += self.get_eyeheight();
		vec
	}

	/// Calculates the distance between to players in a 3D space
	pub fn distance_to(&self, other: &Player) -> f32 {
		let self_pos = self.get_pos();
		let other_pos = other.get_pos();

		Vec3::distance(self_pos, other_pos)
	}

	/// retuns the team the player is in
	fn get_team(&self) -> i32 {
        InternalMemory::read(self.base + TEAM_OFF)
    }


	/// returns true if the player is alive
	pub fn is_alive(&self) -> bool {
		InternalMemory::read::<u8>(self.base + STATE_OFF) == ALIVE_STATE
	}


	/// returns true if a player is infront of the player on the 2D screen
	pub fn is_in_view(&self) -> bool {
		let pos = self.get_pos();
		let (window_width, window_height) = ESP::window_dimensions();
		ViewMatrix::new().world_to_screen(pos, window_width, window_height).0
	}

	/// triggers the ->attacking state of the player to start shooting
	pub fn shoot(&mut self) {
		InternalMemory::write(self.base + PLAYER_ATTACKING_OFF, 1 as u8)
	}

	/// stops shooting after having started through autoshoot
	pub fn stop_shoot(&mut self) {
		InternalMemory::write(self.base + PLAYER_ATTACKING_OFF, 0 as u8)
	}

}
