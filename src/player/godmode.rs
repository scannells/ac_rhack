use core::ffi::c_void;

use crate::{ProcMem, Player};

use crate::{get_executable_map, gen_shellcode};
use crate::util::game_base;

/* Enabling gode mode works by patching the instruction that writes to the health of a
 * player. We can't just add a NOP here, as that would mean no one can die anymore.
 * For this reason we must allocate an executable page where we write code that will
 * check if the health address that is supposed to be written to is the health address
 * of the current player. If it is, we just NOP
 * If it isn't, we set the damage to be 100 and proceed to subtract the damage
 * the instruction that subtracts the damage is
 * sub dword [rbx+0x110], ebp
 * We will patch this instruction and some instructions around it to jump to that page and then
 * restore registers
 */
const DAMAGE_PATCH_OFF: usize = 0x1c2e6;
const HOOK_SIZE: usize = 16;

pub struct GodMode {
    patch_addr: usize,
    enabled: bool,
    saved_instr: Option<[u8; HOOK_SIZE]>,

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

impl GodMode {
    pub fn new() -> Self {
        GodMode {
            patch_addr: game_base() + DAMAGE_PATCH_OFF,
            enabled: false,
            saved_instr: None,
            mem: ProcMem::init(),
            page: None,
            player_base: Player::player1().base,
            patch_shellcode: None,
        }
    }

    pub fn enable(&mut self) {
        // nothing to do
        if self.enabled {
            return
        }

        // If this is the first time patching, make sure to have saved the instruction before
        // so that we can restore the code
        if !self.saved_instr.is_some() {
            /* we need to allocate a rwx page for our payload hook
             * 1. allocate a r-x map at any address
             * 2. patch the damage function so that instead of doing damage jump to the page
             *    where our shellcode will be located
             * 3. we will replace 4 instructions to make space for our hook:
             *        sub     eax, ecx                 # needed for space
             *        mov     dword [rbx+0x114], eax   # this instruction has nothing to do with our code but is needed for space
             *        sub     ebp, edx                 # ebp will contain the damage
             *        sub     dword [rbx+0x110], ebp   # this is the instruction that does damage
             *    this means the shellcode we will jump to has to include these instructions and
             *    set and return the registers cleanly so that no difference is made
            */
            self.page = Some(get_executable_map(4096));

            // TODO: Check if the bot is an enemy
            let shellcode = format!(
              "BITS 64;             ; NASM stuff\n\
              \
              ; restore rax after we used it to jump\n\
              pop rax\n\
              ; this instruction was patched out by the hook so include it here\n\
              sub eax, ecx\n\
              ; this instruction was patched out by the hook so include it here\n\
              mov DWORD [rbx+0x114],eax\n\
              ; move the pointer to the player base into rax\n\
              mov QWORD rax, QWORD 0x{:x}\n\
              ; rbx contains the pointer to the player struct this damage should bedded on\n\
              ; compare the pointer to the base of the player to see if the player is supposed\n\
              ; to take damage\n\
              cmp rax, rbx\n\
              ; if they are equal, turn this into a NOP by jumping to the ret\n\
              jz exit\n\
              ; if this is another player, enable 1-hit kills by increasing the damage to over 9000\n\
              ; the damage will be contained in ebp after ebp - edx\n\
              sub ebp, edx\n\
              ; save rbp in case the value is needed later for something else\n\
              push rbp\n\
              mov ebp, 9001\n\
              ; do the damage\n\
              sub    DWORD [rbx+0x110],ebp\n\
              pop rbp\n\
              \
              exit:\n\
              ; return by pushing to the address of the instructions after the patch (patch_addr + 14 bytes)\n\
              ; onto the stack. This way all registers are preserved. Rax will be restored by the hook\n\
              mov QWORD rax, QWORD 0x{:x}\n\
              push rax\n\
              ret\n"
            , self.player_base, self.patch_addr + HOOK_SIZE);
            println!("patch add = 0x{:x}", self.patch_addr + HOOK_SIZE);

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
                pop rax                  ; restore rax\n\
                NOP                      ; NOP for padding\n\
                NOP                      ; NOP for padding\n"
            , self.page.unwrap() as usize);



            // assemble the patchcode
            self.patch_shellcode = Some(gen_shellcode(patchcode));

            // before overwriting the patch address, we need to save it
            let mut saved: [u8; HOOK_SIZE] = [0; HOOK_SIZE];
            for i in 0..saved.len() {
                saved[i] = self.mem.read(self.patch_addr);
            }

            self.saved_instr = Some(saved);

        }

        // patch the instruction with with the shellcode that jumps to the function hook
        self.mem.write_n(self.patch_addr, &self.patch_shellcode.as_ref().unwrap());
        println!("patching address at 0x{:x} with len {}", self.patch_addr, &self.patch_shellcode.as_ref().unwrap().len());

        // keep a record that this hook is enabled
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        // nothing to do if this patch is already enabled
        if !self.enabled {
            return
        }

        // make sure the code can't accidentally disable without having
        // read the original instructions before
        if !self.saved_instr.is_some() {
            panic!("Tried to disable infinite ammo without ever having enabled it");
        }

        // simply write back the original bytes
        self.mem.write_n(self.patch_addr, &self.saved_instr.unwrap());

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
