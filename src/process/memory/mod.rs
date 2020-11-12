
use std::path::Path;
use std::io::prelude::*;
use std::fs::{OpenOptions, File};
use std::io::SeekFrom;

mod memdata;
pub use memdata::MemData;

use crate::Process;


/// A wrapper for writing / reading memory through /proc/mem. This is needed when
/// trying to write to a r-x page for example, as this method bypasses rwx protections.
pub struct ProcMem {
	handle: File,
}

impl ProcMem {
	pub fn init() -> Self {
		let process = Process::current().unwrap();
		let mempath = format!("{}/mem", &process.proc_dir);
		let mempath = Path::new(&mempath);
		let memhandle = OpenOptions::new()
			.read(true)
			.write(true)
			.open(mempath)
			.expect("Could not open /proc/self/mem for memory operations");

		ProcMem {
			handle: memhandle
		}
	}

	pub fn write<T: MemData>(&mut self, addr: usize, data: T) {
		self.handle.seek(SeekFrom::Start(addr as u64))
			.expect("Could not seek /proc/self/mem file");

		self.handle.write(&data.get_vec())
			.expect("Could not write to /proc/self/mem file");
	}

	pub fn read<T: MemData + Copy>(&mut self, addr: usize) -> T {
		self.handle.seek(SeekFrom::Start(addr as u64))
			.expect("Could not seek /proc/self/mem file");

		let mut _buf = T::make_buf();
		let bread = self.handle.read(&mut _buf)
			.expect("Could not read from /proc/self/mem file");

		T::from_vec(&_buf)
	}

	// a basic memcpy() for larger data buffers. This is called when copying shellcode
	pub fn write_n(&mut self, addr: usize, data: &[u8]) {
		let mut rest = data.len();
		let mut curr = 0;
		while rest != 0 {
			let mut size = 0;
			if rest % 8 == 0 {
				let bytes = u64::from_vec(&Vec::from(&data[curr..curr + 8]));
				self.write(addr + curr, bytes);
				size = 8;
			}
			else if rest % 4 == 0 {
				let bytes = u32::from_vec(&Vec::from(&data[curr..curr + 4]));
				self.write(addr + curr, bytes);
				size = 4;
			}
			else if rest % 2 == 0 {
				let bytes = u16::from_vec(&Vec::from(&data[curr..curr + 2]));
				self.write(addr + curr, bytes);
				size = 2;
			} else {
				let bytes = data[curr];
				self.write(addr + curr, bytes);
				size = 1;
			}

			rest -= size;
			curr += size;
		}
	}
}

/// a wrapper for reading and writing dynamic data through pointers
#[derive(Clone)]
pub struct InternalMemory {}


impl InternalMemory {
	pub fn write<T: MemData>(addr: usize, data: T) {
		let ptr: *mut T = addr as *mut T;
		unsafe { *ptr = data };
	}

	pub fn read<T: MemData + Copy>(addr: usize)  -> T {
		let ptr: *const T = addr as *const T;
		let ret: T = unsafe { *ptr };
		ret
	}
}

#[derive(Debug)]
pub enum MemoryError {
	ProcInvalid,
	InvalidTechnique,
}


