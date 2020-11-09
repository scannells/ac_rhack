use std::path::Path;
use std::io::prelude::*;
use std::fs::{OpenOptions, File};
use std::io::SeekFrom;


use crate::{MemoryManipulator, MemData, MemoryError};
use crate::Process;

pub struct ProcMem {
	handle: File,
}

impl MemoryManipulator for ProcMem {
	fn init(process: &Process) -> Result<Self, MemoryError> {
		let mempath = format!("{}/mem", &process.proc_dir);
		let mempath = Path::new(&mempath);
		let memhandle = OpenOptions::new().read(true).write(true).open(mempath);

		if let Ok(handle) = memhandle {
			return Ok(ProcMem{
				handle: handle
			});
		} else {
			return Err(MemoryError::ProcInvalid);
		}
	}
	
	fn write<T: MemData>(&mut self, addr: usize, data: T) {
		if let Err(_) = self.handle.seek(SeekFrom::Start(addr as u64)) {
			panic!("Can't seek /proc/self/mem file");
		}
		if let Err(_) = self.handle.write(&data.get_vec()) {
			panic!("Something went wrong when trying to write to /proc/self/mem");
		}
	}
	
	fn read<T: MemData + Copy>(&mut self, addr: usize) -> T {
		// seek the file to the address we want to read from
		if let Err(_) = self.handle.seek(SeekFrom::Start(addr as u64)) {
			panic!("Can't seek /proc/self/mem file");
		}
		let mut _buf = T::make_buf();
		let bread = self.handle.read(&mut _buf);
		if let Err(_) = bread {
			panic!("Something went wrong when trying to read from /proc/self/mem");
		}	
		let ret = T::from_vec(&_buf);
		ret
	}
}

