use super::{MemoryManipulator, MemData, MemoryError};
use crate::Process;

pub struct Internal {}


impl MemoryManipulator for Internal {
	fn init(process: &Process) -> Result<Self, MemoryError> where Self: Sized {
		if process.is_internal {
			return Ok(Internal{});
		} else {
			return Err(MemoryError::InvalidTechnique);
		}
	}


	fn write<T: MemData>(&mut self, addr: usize, data: T) {
		let ptr: *mut T = addr as *mut T;
		unsafe { *ptr = data };
	}

	fn read<T: MemData + Copy>(&mut self, addr: usize)  -> T {
		let ptr: *const T = addr as *const T;
		let ret: T = unsafe { *ptr };
		ret
	}
}