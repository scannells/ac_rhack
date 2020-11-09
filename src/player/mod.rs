extern crate lpmanipulator;
use lpmanipulator::{Process, Internal, MemoryManipulator};

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
const ARMOR_OFF: usize = 0x114;
const AMMO_OFF: usize = 0x150;
const GUNSELECT_OFF: usize = 0x120;
const PLAYER_POS_OFF: usize = 0x8;
const PLAYER_Y_OFF: usize = 0x8 + 0x8;

const PLAYER_VIEW_OFF: usize = 0x33440c;


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
		let player1_ptr = process.module("linux_64_client").unwrap().base + PLAYER1_OFF;
		let mut mem: Internal = process.get_mem_access().unwrap();
		let mut player1_base: u64 = mem.read(player1_ptr).unwrap();

		// worldpos is another global variable. It contains the view XYZ coordinates of the
		// current user and can thus be used to create an aimbot
		let worldpos = process.module("linux_64_client").unwrap().base + PLAYER_VIEW_OFF;

		Player::new_at_addr(process, player1_base as usize, worldpos, mem)
	}

	pub fn get_health(&mut self) -> u32  {
		let health: u32 = self.mem.read(self.base + HEALTH_OFF).unwrap();
		health
	}

	/// sets the health of the player to an arbitrary value
	pub fn set_health(&mut self, health: u32)  {
		self.mem.write(self.base + HEALTH_OFF, health).unwrap();
		println!("health address: 0x{:x}", self.base + HEALTH_OFF);
	}

	pub fn get_health_addr(&self) -> usize {
		self.base + HEALTH_OFF
	}

	/// sets the ammo off the current weapon
	pub fn get_ammo(&mut self) -> u32  {
		// the playerstate keeps an index of the current weapon in the ammo array. It is an
		// int so multiply by 4
		let gun: u32 = self.mem.read(self.base + GUNSELECT_OFF).unwrap();
		let ammo: u32 = self.mem.read(self.base + AMMO_OFF + (gun * 4) as usize).unwrap();
		ammo
	}

	/// sets the ammo off the current weapon
	pub fn set_ammo(&mut self, ammo: u32)  {
		// the playerstate keeps an index of the current weapon in the ammo array. It is an
		// int so multiply by 4
		let gun: u32 = self.mem.read(self.base + GUNSELECT_OFF).unwrap();
		self.mem.write(self.base + AMMO_OFF + (gun * 4) as usize, ammo).unwrap();
		println!("ammo addr: 0x{:x}", self.base + AMMO_OFF + (gun * 4) as usize);
	}

	pub fn get_xyz(&mut self) -> [f32; 3]  {
		let mut head: [f32; 3] = [0.0; 3];
		for i in 0..3 {
			head[i] = self.mem.read(self.base + PLAYER_POS_OFF + i * 4).unwrap();
		}
		head
	}

	pub fn get_view(&mut self) -> [f32; 3]  {
		let mut head: [f32; 3] = [0.0; 3];
		for i in 0..3 {
			head[i] = self.mem.read(self.worldpos + i * 4).unwrap();
		}
		head
	}

	pub fn fly(&mut self)  {
		self.mem.write(self.base + PLAYER_Y_OFF, 18.0 as f32).unwrap();
	}

	pub fn aim(&mut self)  {
		self.mem.write(self.worldpos + 8, 5.0 as f32).unwrap();
		self.mem.write(self.worldpos, 260.0 as f32).unwrap();
	}
}

pub struct Enemy {
	base: usize
}
