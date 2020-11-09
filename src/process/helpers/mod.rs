use std::path::{Path, PathBuf};
use std::fs::{DirEntry, read_link};
use std::os::linux::fs::MetadataExt;

extern crate users;
use users::get_current_uid;

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

pub fn is_proc_dir(dir: &DirEntry) -> bool {
	// a proc dir contains only digits
	for &c in path_basename(dir).as_bytes() {
		if c < '0' as u8 || c > '9' as u8 {
			return false;
		}
	}

	// the second criteria is that it must be a directory
	if let Ok(file_type) = dir.file_type() {
		return file_type.is_dir();
	} else {
		return false;
	}
}


pub fn owned_by_user(dir: &DirEntry) -> bool {
	let meta = dir.metadata();

	if let Err(_) = meta {
		return false;
	}

	let meta = meta.unwrap();

	meta.st_uid() == get_current_uid() || get_current_uid() == 0
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

pub fn cmp_strpaths(path1: &str, path2: &str) -> bool {
	let path1 = PathBuf::from(path1);
	let path2 = PathBuf::from(path2);

	if let Err(_) = std::fs::canonicalize(&path1) {
		return false;
	}

	if let Err(_) = std::fs::canonicalize(&path2) {
		return false;
	}

	return path1 == path2;

}

pub fn cmp_basenames(path1: &str, path2: &str) -> bool {
	let path1 = filename_basename(path1);
	let path2 = filename_basename(path2);

	path1 == path2
}