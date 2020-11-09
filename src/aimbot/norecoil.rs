extern crate lpmanipulator;
use lpmanipulator::{Process, MemoryManipulator, ProcMem};

const RECOIL_PATCH_OFF: usize = 0xbd220;

pub struct NoRecoilSpread {
    patch_addr: usize,
    enabled: bool,
    saved_instr: Option<u32>,

    // the reason we use procmem here is that memory writes via /proc/mem
    // bypass write protection on executable pages
    mem: ProcMem
}

impl NoRecoilSpread {
    pub fn new(process: &Process) -> Self {
        NoRecoilSpread {
            patch_addr: process.module("linux_64_client").unwrap().base + RECOIL_PATCH_OFF,
            enabled: false,
            saved_instr: None,
            mem: process.get_mem_access().unwrap(),
        }
    }

    pub fn enable(&mut self) {
        println!("recoil addr=0x{:x}", self.patch_addr);
        // nothing to do
        if self.enabled {
            return
        }

        // If this is the first time patching, make sure to have saved the instruction before
        // so that we can restore the code
        if !self.saved_instr.is_some() {
            self.saved_instr = Some({
                self.mem.read(self.patch_addr).unwrap()
            });
        }

        // patch the instruction with a simple ret
        self.mem.write(self.patch_addr, 0xc3 as u8).unwrap();

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
            panic!("Tried to disable Recoil / Spread without ever having enabled it");
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
