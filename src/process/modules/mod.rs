use std::fs::read_to_string;
use std::collections::HashMap;


use crate::{Process, ProcessErrors, Module};
use crate::process::helpers::*;


fn parse_module(maps_line: &str) -> Result<Module, ()> {
	let columns: Vec<&str> = maps_line.split_whitespace().collect();
	
	// the format is the following: start_addr-end_addr perms pgoff major:minor inode binary (6 columns)
	// in some cases a memory segment exists without a module. We are not interested in these and ignore them by ignoring all modules that do not have 6 columns
	if columns.len() == 5 {
		return Err(());
	}

	// get the backing file of the module
	let mut file = String::from(columns[5]);

	// in case the file is a special one such as [stack] or [heap], create a module for that. Otherwise only get the base of the module (the segment that contains the executable bit).
	// we can obtain information about the other segments from the ELF file
	let is_special = file == "[stack]" || file == "[heap]";
	if is_special {
		file = String::from(file).replace("[", "").replace("]", "");
	}


	let mut executable = false;
	 if let Some(_) = columns[1].find('x') {
		 executable = true
	}

	// vsyscall meets our filter criteria but is uninteresting
	if (!is_special && !executable) || file == "[vsyscall]" {
		return Err(());
	}

	// parse the base address
	let range: Vec<&str> = columns[0].split('-').collect();
	let start = usize::from_str_radix(range[0], 16).unwrap();
	
	Ok(Module {
		name: filename_basename(&file),
		file: String::from(file),
		base: start,
		size: None,
	})
}

pub fn parse_modules(process: &Process) -> Result<HashMap<String, Module>, ProcessErrors> {

	let mut res: HashMap<String, Module> = HashMap::new();

	let maps_path = format!("{}/maps", &process.proc_dir);
	let modules = read_to_string(maps_path);
	if let Err(_) = modules {
		return Err(ProcessErrors::ProcInvalid);
	}
	let modules = modules.unwrap();

	for line in modules.lines() {
		let module = parse_module(line);
		if let Ok(module) = module {
			res.insert(module.name.clone(), module);
		} 
	}

	Ok(res)
}

pub fn get_module(process: &Process, module_name: &str) -> Result<Module, ProcessErrors> {
	let modules = parse_modules(process);

	if let Ok(modules) = modules {
		let module = modules.get(module_name);
	
		if let Some(module) = module {
			return Ok(module.clone());
		} else {
			return Err(ProcessErrors::NotFound);
		}
	} else {
		return Err(ProcessErrors::ProcInvalid);
	}
}
