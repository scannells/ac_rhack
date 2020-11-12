/**
 * the code in this file deals with reading the global variable "players", which is a
 * vector of player pointers where each player is a bot and potentially an enemy.
 * AC is written in C++ and dealing with a C++ Vector with raw pointers is tiresome, so
 * the file enemies.cpp is a C wrapper that fills an array of pointers to enemies for us
 */

use crate::{Process, InternalMemory};
use crate::internal::InternalMemory;


const MAX_PLAYERS: usize = 32;

// offset to the offset of the position of an enemy relative to its base
const PLAYER_POS_OFF: usize = 0x8;
const TEAM_OFF: usize = 0x344;
const STATE_OFF: usize = 0x86;

const CS_ALIVE: u8 = 0;



#[derive(Clone, Copy)]
pub struct Enemy {
    base: usize,
}

impl Enemy {
    pub fn all(enemy_base: usize, mem: &mut Internal) -> Vec<Enemy> {
        let vec_of_enems = unsafe {
            let vec_ptr = enemy_base as *const AcVector;
            (*vec_ptr)
        };



        enems
    }

    pub fn get_pos(&self, mem: &mut Internal) -> [f32; 3] {
        let mut head: [f32; 3] = [0.0; 3];
        if self.base == 0 {
            return head;
        }
		for i in 0..3 {
			head[i] = mem.read(self.base + PLAYER_POS_OFF + i * 4);
		}
		head
    }

    pub fn is_alive(&self, mem: &mut Internal) -> bool {
        let state: u8 = mem.read(self.base + STATE_OFF);
        state == CS_ALIVE
    }

    pub fn team(&self, mem: &mut Internal) -> i32 {
        mem.read(self.base + TEAM_OFF)
    }
}