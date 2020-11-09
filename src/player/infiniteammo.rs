use crate::{Process, MemoryManipulator, ProcMem};

const AMMO_PATCH_OFF: usize = 0xbf50b;

pub struct InfiniteAmmo {
    patch_addr: usize,
    enabled: bool,
    saved_instr: Option<u32>,

    // the reason we use procmem here is that memory writes via /proc/mem
    // bypass write protection on executable pages
    mem: ProcMem
}

impl InfiniteAmmo {
    pub fn new(process: &Process) -> Self {
        InfiniteAmmo {
            patch_addr: process.module("linux_64_client").unwrap().base + AMMO_PATCH_OFF,
            enabled: false,
            saved_instr: None,
            mem: process.get_mem_access().expect("Failed to get access to /proc/self/mem")
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
            self.saved_instr = Some({
                self.mem.read(self.patch_addr)
            });
        }

        // patch the instruction with 3 bytes of NOPs (1x 16 bytes and 1x 8 byte write)
        self.mem.write(self.patch_addr, 0x90_90 as u16);
        self.mem.write(self.patch_addr + 2 as usize, 0x90 as u8);

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
        self.mem.write(self.patch_addr, self.saved_instr.unwrap());

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
