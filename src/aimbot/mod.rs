mod norecoil;
use norecoil::NoRecoilSpread;

mod autoshoot;
use autoshoot::AutoShoot;

use crate::player::Player;
use crate::process::InternalMemory;
use crate::util::game_base;
use crate::util::Vec3;

use std::f32::consts::PI;

// offset from the game's base to a pointer that points at camera1
const CAMERA1_OFF: usize = 0x1371b0;
const YAW_OFF: usize = 0x44;
const PITCH_OFF: usize = 0x48;

const IS_VISIBLE_OFF: usize = 0xda520;


pub struct AimBot {
    player: Player,
    pub norecoil_spread: NoRecoilSpread,
    pub autoshoot: AutoShoot,
    enabled: bool,
}

#[link(name="bot_trampoline", kind="static")]
extern "C" {
    fn bot_isvisible(func_addr: usize, from: *const Vec3, to: *const Vec3) -> u8;
}

impl AimBot {
    pub fn new() -> AimBot {
        let player = Player::player1();
        AimBot {
            autoshoot: AutoShoot::new(player.base),
            player,
            norecoil_spread: NoRecoilSpread::new(),
            enabled: false,
        }
    }


    // returns the address of the camera1 object
    fn camera1() -> usize {
        InternalMemory::read::<u64>(game_base() + CAMERA1_OFF) as usize
    }

    // calculates the position the player will be in the next frame when it moves


    fn enemy_to_angle(&self, enemy: &Player) -> (f32, f32) {
        let target_pos = enemy.get_pos();
        let self_pos = self.player.get_pos();
        let dx = target_pos.x - self_pos.x;
        let dy = target_pos.y - self_pos.y;
        let dz = target_pos.z - self_pos.z;

        // horizontal angle to player
        let yaw = dy.atan2(dx) * 180.0 / PI;

        let distance = self.player.distance_to(enemy);
        let pitch = dz.atan2(distance) * 180.0 / PI;

        (yaw + 90.0, pitch)
    }

    // returns true when an enemy can be shot at
    // to do this, we call the function "IsVisible" of AssaultCube and
    // let the game do the math :)
    // The problem with this function is that it requires C++ calling
    // conentions. For this reason, there is a trampoline written in C++
    // that we are calling here
    fn is_visible(&self, enemy: &Player) -> bool {
        let res = unsafe {
            let is_visible_addr = game_base() + IS_VISIBLE_OFF;
            let from = &self.player.get_pos() as *const Vec3;
            let to = &enemy.get_pos() as *const Vec3;
            bot_isvisible(is_visible_addr, from, to)
        };
        res == 1
    }


    // todo: implement locking on enemy
    /// Called after each frame by the main SwapBuffer hook. Handles findings a target
    /// to aim at and updating camera perspective
    pub fn logic(&self) {
        // don't to anything if the aimbot is disabled
        if !self.enabled {
            return
        }

        // TODO: Team check
        // obtain a list of all enemies which are alive
        let players: Vec<Player> = Player::players()
            .into_iter()
            .filter(|p| p.is_alive())
            .collect();


        // no need to do anything if no enemies are alive
        if players.len() == 0 {
            return
        }

        let mut best_dist = f32::INFINITY;
        let mut target = None;
        for p in players.iter() {
            let pdist = self.player.distance_to(p);
            if pdist < best_dist && self.is_visible(p) {
                best_dist = pdist;
                target = Some(p);
            }
        }

        // verify that a target was found to point at
        if target.is_none() {
            return
        }

        let target = target.unwrap();

        // update the camera position to point at the enemy
        let (yaw, pitch) = self.enemy_to_angle(target);

        // verify camera1 is valid!
        if Self::camera1() == 0x0 {
            return;
        }

        InternalMemory::write(Self::camera1() + YAW_OFF, yaw);
        InternalMemory::write(Self::camera1() + PITCH_OFF, pitch);
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn toggle(&mut self) {
        if self.enabled {
            self.disable();
        } else {
            self.enable();
        }
    }
}