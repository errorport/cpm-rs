use std::env;
use std::process::exit;
use std::fs::File;
use std::io::Write;

pub mod input_parser;
pub mod customtask;
pub mod path;
pub mod scheduler;

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

pub fn print_output(output: &String) {
	let filename: &String = &env::args().collect::<Vec<String>>()[1];
	let output_filename
		= format!("{}.sched.out", filename.split(".").collect::<Vec<&str>>()[0]);
	let mut file
		= File::create(output_filename).expect("Could not create output file.");
	file.write_all(output.as_bytes()).expect("Could not write output.");
}

