use std::path::{Path};
use std::fs::{DirEntry, read_link};


pub fn path_basename(dir: &DirEntry) -> String {
	let name = dir.file_name();
	let name = name.into_string();
	match name {
		Ok(basename) => basename,
		Err(_) => panic!("Can't convert OsString to String"),
	}
} 

pub fn filename_basename(file_name: &str) -> String {
	let path = Path::new(file_name);
	let file_name = path.file_name();
	if let Some(file_name) = file_name {
		return file_name.to_os_string().into_string().unwrap();
	} else {
		return String::from("");
	}
}




pub fn read_exe(proc_dir: &DirEntry) -> (bool, String) {
	let mut path = proc_dir.path();
	path.push("exe");
	let path = path.as_path();

	let exe = read_link(path);
	if let Ok(exe) = exe {
		return (true, exe.into_os_string().into_string().unwrap());
	} else {
		return (false, String::from(""));
	}
}

