/**
 * the code in this file deals with reading the global variable "players", which is a
 * vector of player pointers where each player is a bot and potentially an enemy.
 * AC is written in C++ and dealing with a C++ Vector with raw pointers is tiresome, so
 * the file enemies.cpp is a C wrapper that fills an array of pointers to enemies for us
 */

extern crate lpmanipulator;
use lpmanipulator::{Process, Internal, MemoryManipulator};


const MAX_PLAYERS: usize = 32;

// offset to the offset of the position of an enemy relative to its base
const PLAYER_POS_OFF: usize = 0x8;


// AssaultCube has a custom vector for the enemies. We have a pointer to this
// struct so we can just deref it
#[repr(C)]
#[derive(Clone, Copy)]
struct AcVector {
    enemy_addresses: usize, // pointer to the buffer of pointers to the enemies
    capacity: i32,          // max size of the buffer
    elements: i32           // how many elements there actually are
}

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

        let mut enems = Vec::with_capacity(MAX_PLAYERS);

        // fill the vector of enemies
        for i in 0..vec_of_enems.elements {
            let enem_addr: u64 = mem.read(vec_of_enems.enemy_addresses + (i * 8) as usize).unwrap();

            // sometimes pointers are NULL
            if enem_addr == 0x0 {
                continue;
            }
            enems.push(Enemy {
                base: enem_addr as usize
            });
        }

        enems
    }

    pub fn get_pos(&self, mem: &mut Internal) -> [f32; 3] {
        let mut head: [f32; 3] = [0.0; 3];
        if self.base == 0 {
            return head;
        }
		for i in 0..3 {
			head[i] = mem.read(self.base + PLAYER_POS_OFF + i * 4).unwrap();
		}
		head
    }
}