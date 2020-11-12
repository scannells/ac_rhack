use crate::process::{Process, Internal, MemoryManipulator};

mod godmode;
use godmode::GodMode;

mod infiniteammo;
use infiniteammo::InfiniteAmmo;

/// offset to the player1 pointer from the base of the loaded game
const PLAYER1_OFF: usize = 0x128328;

// there can be only 32 players in a match
const MAX_OTHER_PLAYER: usize = 32;

/// offsets from the playerent to fields we want to read / write
const HEALTH_OFF: usize = 0x110;
const AMMO_OFF: usize = 0x150;
const GUNSELECT_OFF: usize = 0x120;
const TEAM_OFF: usize = 0x344;
const STATE_OFF: usize = 0x86;
const PLAYER_POS_OFF: usize = 0x8;
const PLAYER_Y_OFF: usize = 0x8 + 0x8;

const PLAYER_VIEW_OFF: usize = 0x13745c;

const PLAYER_ATTACKING_OFF: usize = 0x23c;

pub struct Player {
	pub base: usize,
	worldpos: usize,
	mem: Internal,

	pub god_mode: GodMode,
	pub infinite_ammo: InfiniteAmmo,

}


impl Player {
	fn new_at_addr(process: &Process, addr: usize, worldpos: usize, mem: Internal) -> Self
	{
		Player {
			base: addr.clone(),
			worldpos: worldpos,
			mem: mem,
			god_mode: GodMode::new(process, addr),
			infinite_ammo: InfiniteAmmo::new(process),
		}
	}

	pub fn new(process: &Process) -> Self  {
		// There is a global variable called "player1", which is a pointer
		// to the actual, dynamically allocated player struct.
		// In order to obtain the address of the player, just dereference the global pointer
		let ac_base  = process.module("linux_64_client").unwrap().base;
		let player1_ptr = ac_base + PLAYER1_OFF;
		let mut mem: Internal = process.get_mem_access().expect("Could not create memory manager");
		let mut player1_base: u64 = mem.read(player1_ptr);

		// worldpos is another global variable. It contains the view XYZ coordinates of the
		// current user and can thus be used to create an aimbot
		let worldpos = ac_base + PLAYER_VIEW_OFF;

		Player::new_at_addr(process, player1_base as usize, worldpos, mem)
	}


	/// sets the health of the player to an arbitrary value
	pub fn set_health(&mut self, health: u32)  {
		self.mem.write(self.base + HEALTH_OFF, health);
		println!("health address: 0x{:x}", self.base + HEALTH_OFF);
	}


	/// sets the ammo off the current weapon
	pub fn set_ammo(&mut self, ammo: u32)  {
		// the playerstate keeps an index of the current weapon in the ammo array. It is an
		// int so multiply by 4
		let gun: u32 = self.mem.read(self.base + GUNSELECT_OFF);
		self.mem.write(self.base + AMMO_OFF + (gun * 4) as usize, ammo);
		println!("ammo addr: 0x{:x}", self.base + AMMO_OFF + (gun * 4) as usize);
	}

	pub fn get_xyz(&mut self) -> [f32; 3]  {
		let mut head: [f32; 3] = [0.0; 3];
		for i in 0..3 {
			head[i] = self.mem.read(self.base + PLAYER_POS_OFF + i * 4);
		}
		head
	}

	pub fn get_view(&mut self) -> [f32; 3]  {
		let mut head: [f32; 3] = [0.0; 3];
		for i in 0..3 {
			head[i] = self.mem.read(self.worldpos + i * 4);
		}
		head
	}

	pub fn aim(&mut self)  {
		self.mem.write(self.worldpos + 8, 5.0 as f32);
		self.mem.write(self.worldpos, 260.0 as f32);
	}

	pub fn team(&mut self) -> i32 {
        self.mem.read(self.base + TEAM_OFF)
    }

	pub fn shoot(&mut self) {
		self.mem.write(self.base + PLAYER_ATTACKING_OFF, 1 as u8);
	}

	pub fn stop_shoot(&mut self) {
		self.mem.write(self.base + PLAYER_ATTACKING_OFF, 0 as u8);
	}
}
