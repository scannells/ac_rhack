
mod memdata;
pub use memdata::MemData;

pub trait MemoryManipulator {
	fn init(process: &super::Process) -> Result<Self, MemoryError> where Self: Sized;
	fn write<T: MemData>(&mut self, addr: usize, data: T);
	fn read<T: MemData + Copy>(&mut self, addr: usize) -> T;

	fn write_n(&mut self, addr: usize, data: &[u8]) {
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

// export linux external process memory manipulation
mod procmem;
pub use procmem::ProcMem;

// 'internal' memory manipulation is just a wrapper for pointers and works on all OS
mod internal;
pub use internal::Internal;

#[derive(Debug)]
pub enum MemoryError {
	ProcInvalid,
	InvalidTechnique,
}


