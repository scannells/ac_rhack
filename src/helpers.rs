use std::process::Command;
use std::fs::{File, remove_file};
use std::io::prelude::*;
use std::path::Path;

extern crate nix;
use nix::sys::mman::{mmap, ProtFlags, MapFlags};

use core::ffi::c_void;

extern crate lpmanipulator;
use lpmanipulator::ProcessErrors;

pub fn get_executable_map(size: usize) -> Result<*mut c_void, ProcessErrors> {
    let mut prot_flags = ProtFlags::empty();
    prot_flags.insert(ProtFlags::PROT_READ);
    prot_flags.insert(ProtFlags::PROT_EXEC);

    let mut map_flags = MapFlags::empty();
    map_flags.insert(MapFlags::MAP_PRIVATE);
    map_flags.insert(MapFlags::MAP_ANON);
    let rw_page = unsafe {
          mmap(0 as *mut c_void,  // 0 = NULL means allocate at any address
               size,                // 4096 is the smallest possible page
               prot_flags,          // R/W
               map_flags,           // Not backed by a file
               -1,                 // -1 explicit for no FD
               0                   // no offset required
                ).unwrap()
    };

    Ok(rw_page)
}


/// Takes in a string of x86 intel assembly and uses nasm to return a vec of bytes
/// of raw instructions corresponding to the shellcode
pub fn gen_shellcode(shellcode: String) -> Vec<u8>{

    // write the shellcode to a file
    let mut asm_file = File::create("/tmp/ac_hack_asm.S").expect("Failed to write to /tmp");
    asm_file.write_all(shellcode.as_bytes());

    // assemble it
    Command::new("nasm")
        .arg("-f")
        .arg("bin")
        .arg("/tmp/ac_hack_asm.S")
        .arg("-o")
        .arg("/tmp/ac_hack_asm")
        .status()
        .expect("This hack requires NASM to gen shellcode dynamically. Please install it");


    // delete the assembly file
    remove_file(Path::new("/tmp/ac_hack_asm.S")).expect("Could not clean shellcode file");


    // read the resulting opcodes into a u8 vec - start with a 4096byte buffer
    let mut asm_file = File::open("/tmp/ac_hack_asm").expect("Something went wrong when assembling shellcode\n");
    let size = asm_file.metadata().unwrap().len();

    let mut res: Vec<u8> = vec![0; size as usize];
    asm_file.read(&mut res).expect("Failed to read assembly dump");

    // delete the assembled code
    remove_file(Path::new("/tmp/ac_hack_asm")).expect("Could not clean assembled file");
    res
}