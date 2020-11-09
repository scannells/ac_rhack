use std::mem::transmute;

fn fill_array(arr: &mut [u8], vec: &Vec<u8>) {
	let mut idx: usize = 0;
	for elem in vec.iter() {
		arr[idx] = *elem;
		idx += 1;
	}
}

pub trait MemData {
	fn get_vec(self) -> Vec<u8>;
	fn from_vec(vec: &Vec<u8>) -> Self;
	fn make_buf() -> Vec<u8>;
}

impl MemData for u8 {
	fn get_vec(self) -> Vec<u8> {
		Vec::from([self])
	}

	fn from_vec(vec: &Vec<u8>) -> Self {
		vec[0]
	}

	fn make_buf() -> Vec<u8> {
		vec![0 as u8; 1]
	}
}

impl MemData for i8 {
	fn get_vec(self) -> Vec<u8> {
		Vec::from([self as u8])
	}

	fn from_vec(vec: &Vec<u8>) -> Self {
		vec[0] as i8
	}

	fn make_buf() -> Vec<u8> {
		vec![0 as u8; 1]
	}
}

impl MemData for u16 {
	fn get_vec(self) -> Vec<u8> {
		let data = unsafe {
			transmute::<u16, [u8; 2]>(self)
		};
		Vec::from(data)
	}

	fn from_vec(vec: &Vec<u8>) -> Self {
		let mut arr: [u8; 2] = [0; 2];
		fill_array(&mut arr, vec);
		let discrete = unsafe {
			transmute::<[u8; 2], u16>(arr)
		};
		discrete
	}

	fn make_buf() -> Vec<u8> {
		vec![0 as u8; 2]
	}
}

impl MemData for i16 {
	fn get_vec(self) -> Vec<u8> {
		let data = unsafe {
			transmute::<i16, [u8; 2]>(self)
		};
		Vec::from(data)
	}

	fn from_vec(vec: &Vec<u8>) -> Self {
		let mut arr: [u8; 2] = [0; 2];
		fill_array(&mut arr, vec);
		let discrete = unsafe {
			transmute::<[u8; 2], i16>(arr)
		};
		discrete
	}

	fn make_buf() -> Vec<u8> {
		vec![0 as u8; 2]
	}
}

impl MemData for u32 {
	fn get_vec(self) -> Vec<u8> {
		let data = unsafe {
			transmute::<u32, [u8; 4]>(self)
		};
		Vec::from(data)
	}

	fn from_vec(vec: &Vec<u8>) -> Self {
		let mut arr: [u8; 4] = [0; 4];
		fill_array(&mut arr, vec);
		let discrete = unsafe {
			transmute::<[u8; 4], u32>(arr)
		};
		discrete
	}

	fn make_buf() -> Vec<u8> {
		vec![0 as u8; 4]
	}
}

impl MemData for i32 {
	fn get_vec(self) -> Vec<u8> {
		let data = unsafe {
			transmute::<i32, [u8; 4]>(self)
		};
		Vec::from(data)
	}

	fn from_vec(vec: &Vec<u8>) -> Self {
		let mut arr: [u8; 4] = [0; 4];
		fill_array(&mut arr, vec);
		let discrete = unsafe {
			transmute::<[u8; 4], i32>(arr)
		};
		discrete
	}

	fn make_buf() -> Vec<u8> {
		vec![0 as u8; 4]
	}
}

impl MemData for u64 {
	fn get_vec(self) -> Vec<u8> {
		let data = unsafe {
			transmute::<u64, [u8; 8]>(self)
		};
		Vec::from(data)
	}

	fn from_vec(vec: &Vec<u8>) -> Self {
		let mut arr: [u8; 8] = [0; 8];
		fill_array(&mut arr, vec);
		let discrete = unsafe {
			transmute::<[u8; 8], u64>(arr)
		};
		discrete
	}

	fn make_buf() -> Vec<u8> {
		vec![0 as u8; 8]
	}
}

impl MemData for i64 {
	fn get_vec(self) -> Vec<u8> {
		let data = unsafe {
			transmute::<i64, [u8; 8]>(self)
		};
		Vec::from(data)
	}

	fn from_vec(vec: &Vec<u8>) -> Self {
		let mut arr: [u8; 8] = [0; 8];
		fill_array(&mut arr, vec);
		let discrete = unsafe {
			transmute::<[u8; 8], i64>(arr)
		};
		discrete
	}

	fn make_buf() -> Vec<u8> {
		vec![0 as u8; 8]
	}
}

impl MemData for f32 {
	fn get_vec(self) -> Vec<u8> {
		let data = unsafe {
			transmute::<f32, [u8; 4]>(self)
		};
		Vec::from(data)
	}

	fn from_vec(vec: &Vec<u8>) -> Self {
		let mut arr: [u8; 4] = [0; 4];
		fill_array(&mut arr, vec);
		let discrete = unsafe {
			transmute::<[u8; 4], f32>(arr)
		};
		discrete
	}

	fn make_buf() -> Vec<u8> {
		vec![0 as u8; 4]
	}
}

impl MemData for f64 {
	fn get_vec(self) -> Vec<u8> {
		let data = unsafe {
			transmute::<f64, [u8; 8]>(self)
		};
		Vec::from(data)
	}

	fn from_vec(vec: &Vec<u8>) -> Self {
		let mut arr: [u8; 8] = [0; 8];
		fill_array(&mut arr, vec);
		let discrete = unsafe {
			transmute::<[u8; 8], f64>(arr)
		};
		discrete
	}

	fn make_buf() -> Vec<u8> {
		vec![0 as u8; 8]
	}
}


