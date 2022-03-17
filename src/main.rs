use std::env;
use std::process::exit;

extern crate cpm_rs;

use cpm_rs::*;

fn main() {
	let mut scheduler = scheduler::Scheduler::new();
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		eprintln!("Please provide an input file path!");
		exit(1);
	}
	match input_parser::parse_input_file(&args[1]) {
		Ok(task_list) => { scheduler.schedule(task_list); },
		Err(e) => {eprintln!("Error: {}", e); exit(1);},
	}
}

