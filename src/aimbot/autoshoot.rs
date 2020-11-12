use core::ffi::c_void;

use crate::{ProcMem};

use crate::{get_executable_map, gen_shellcode};
use crate::util::game_base;

/* Autoshoot works by hooking the AC function "playerincrosshair".
 * This function takes in the current view of the player and returns a pointer
 * to a player entity if the crosshair is on it.
 * Here, we will build a hook that checks if the pointer is of an enemy
 * by reading the team field and if so, we set the "attacking" property of our own player
 * to shoot. If we are not on an enemy, we set this property to false to stop shooting.
 * Lucky for us, following the 'ret' instruction of this function comes a 12 byte padding
 * This means , by patching the ret as well as the padding we can insert code such as
 * push rax
 * mov rax, SHELLCODE_LOCATION
 * jmp rax
 */

/// offset to the "playerincrosshair" function
const CROSSHAIR_OFF: usize = 0xbad63;

/// offset to the ->attacking field. When set, the player will shoot
const PLAYER_ATTACKING_OFF: usize = 0x23c;

pub struct AutoShoot {
    patch_addr: usize,
    enabled: bool,

    // the reason we use procmem here is that memory writes via /proc/mem
    // bypass write protection on executable pages
    mem: ProcMem,

    // a reference to the executable map containing the shellcode for this feature
    page: Option<*mut c_void>,

    // the address of the health of the player. It will be used in the shellcode
    player_base: usize,

    // the shellcode used to detour the damage taking function
    patch_shellcode: Option<Vec<u8>>,
}

impl AutoShoot {
    pub fn new(player_base: usize) -> Self {
        AutoShoot {
            patch_addr: game_base() + CROSSHAIR_OFF,
            enabled: false,
            mem: ProcMem::init(),
            page: None,
            player_base: player_base,
            patch_shellcode: None,
        }
    }

    pub fn enable(&mut self) {
        // nothing to do
        if self.enabled {
            return
        }

        // If this is the first time patching, make sure to prepare the shellcodes
        if !self.page.is_some() {
            /* we need to allocate a r-x page for our payload hook
             * 1. allocate a r-x map at any address
             * 2. patch the damage function so that instead of doing damage jump to the page
             *    where our shellcode will be located
             * 3. we will patch the last instruction of the function, which is a ret
             *    that is followed by some padding. If we overwrite the padding, we can jump
             *    to our shellcode
            */
            self.page = Some(get_executable_map(4096));

            // TODO: Add team check
            let shellcode = format!(
              "BITS 64;             ; NASM stuff\n\
              \
              ; restore rax after we used it to jump - it contains the resulting pointer\n\
              pop rax\n\
              ; save rbx, we will use it as a pointer to our player\n\
              push rbx\n\
              mov QWORD rbx, 0x{:x}\n\
              ; check if rax is NULL. If so there is no player in the crosshair. Stop shooting\n\
              test rax, rax\n\
              jnz attack\n\
              ; stop shooting by setting ->attacking to 0\n\
              mov BYTE [rbx + 0x{:x}], 0\n\
              jmp exit\n\
              attack:\n\
              ; if an enemy is in our crosshair, shoot by setting ->attacking to 1\n\
              mov BYTE [rbx + 0x{:x}], 1\n\
              exit:\n\
              pop rbx\n\
              ; this will also return to the main code as we jmped into this\n\
              ret\n\
              "
            , self.player_base, PLAYER_ATTACKING_OFF, PLAYER_ATTACKING_OFF);

            let shellcode = gen_shellcode(shellcode);

            // now copy the shellcode to the executable map
            self.mem.write_n(self.page.unwrap() as usize, &shellcode);


            // the patch we will write will jump to the page
            // these instructions are 13 bytes large, the instructions we are patching
            // are 14 bytes in size. So add a NOP for padding
            let patchcode = format!(
                "BITS 64;                ; NASM stuff\n\
                push rax                 ; save rax\n\
                mov rax, QWORD 0x{:x}    ; move the address to the page into rax\n\
                jmp  rax                 ; jump to the shellcode in our map\n\
                "
            , self.page.unwrap() as usize);



            // assemble the patchcode
            self.patch_shellcode = Some(gen_shellcode(patchcode));
        }

        // patch the instruction with with the shellcode that jumps to the function hook
        self.mem.write_n(self.patch_addr, &self.patch_shellcode.as_ref().unwrap());
        println!("patching address at 0x{:x} with len {} (for autoshoot)", self.patch_addr, &self.patch_shellcode.as_ref().unwrap().len());

        // keep a record that this hook is enabled
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        // nothing to do if this patch is already enabled
        if !self.enabled {
            return
        }

        // simply write back the original ret instruction
        self.mem.write(self.patch_addr, 0xc3 as u8);

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
