extern crate lpmanipulator;
use lpmanipulator::{Process, Internal, MemoryManipulator};

// offset to the player1 pointer 
const PLAYER1_OFF: usize = 0x3252e8;

// offset to the vector of pointer to all other players
const PLAYERS_OFF: usize = 0x3252f0;

// there can be only 32 players in a match
const MAX_OTHER_PLAYER: usize = 32;

// offsets from the playernt to fields we want to read / write
const HEALTH_OFF: usize = 0x110;
const AMMO_OFF: usize = 0x128 + (4 * 10) + 24;
const PLAYER_POS_OFF: usize = 0x8;
const PLAYER_Y_OFF: usize = 0x8 + 0x8;

const PLAYER_VIEW_OFF: usize = 0x33440c;

pub struct Player {
	base: usize,
	worldpos: usize,
	mem: Internal,
}

use std::process::Command;
use std::str;

impl Player {
	fn new_at_addr(addr: usize, worldpos: usize, mem: Internal) -> Self
	{
		Player {
			base: addr,
			worldpos: worldpos,
			mem: mem
		}
	}

	pub fn player1(process: &Process) -> Self  {
		let player1_ptr = process.module("linux_64_client").unwrap().base + PLAYER1_OFF;


		let worldpos = process.module("linux_64_client").unwrap().base + PLAYER_VIEW_OFF;
		let mut mem: Internal = process.get_mem_access().unwrap();
		let mut player1_base: u64 = mem.read(player1_ptr).unwrap();
		Player::new_at_addr(player1_base as usize, worldpos, mem)
	}

	pub fn get_health(&mut self) -> u32  {
		let health: u32 = self.mem.read(self.base + HEALTH_OFF).unwrap();
		health
	}

	pub fn set_health(&mut self, health: u32)  {
		self.mem.write(self.base + HEALTH_OFF, health).unwrap();
	}

	pub fn get_ammo(&mut self) -> u32  {
		let ammo: u32 = self.mem.read(self.base + AMMO_OFF).unwrap();
		ammo
	}

	pub fn set_ammo(&mut self, ammo: u32)  {
		self.mem.write(self.base + AMMO_OFF, ammo).unwrap();
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
