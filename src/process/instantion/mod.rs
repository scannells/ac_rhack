use std::fs::{DirEntry, read_dir};
use std::path::Path;
use std::process;

use crate::{Process, ProcessErrors};
use crate::process::helpers::*;

fn from_proc_dir(dir: &DirEntry, is_internal: bool) -> Result<Process, ProcessErrors> {
	let exe = read_exe(dir);
	if !exe.0 {
		return Err(ProcessErrors::ProcInvalid);
	}

	let exe = exe.1;

	let pid = path_basename(dir).parse();
	if let Err(_) = pid {
		return Err(ProcessErrors::ProcInvalid);
	}

	let process = Process {
		pid: pid.unwrap(),
		proc_dir: dir.path().into_os_string().into_string().unwrap(),
		exe: exe,
		is_internal: is_internal,
	};

	Ok(process)
}


pub fn from_current() -> Result<Process, ProcessErrors> {
	let curr_pid = process::id();
	let proc_dir = std::format!("/proc/{}", curr_pid);
	let proc_root = Path::new("/proc");
	for entry in read_dir(proc_root).expect("Failed to read /proc dir") {
		let entry = entry.expect("Failed to get next entry in /proc dir");
		if entry.path().into_os_string().into_string().unwrap() == proc_dir {
			return from_proc_dir(&entry, true);
		}
	}

	Err(ProcessErrors::NotFound)
}
