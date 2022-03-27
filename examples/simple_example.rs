use std::process::exit;

extern crate cpm_rs;

use cpm_rs::{scheduler::Scheduler, customtask::CustomTask};

#[allow(unused_must_use)]

fn main() {
	let mut scheduler = Scheduler::<i32>::new();
	scheduler.add_task(CustomTask::new(
		"Task_A".to_string()
		, 1
		, vec!{}
	));
	scheduler.add_task(CustomTask::new(
		"Sidetask_B".to_string()
		, 3
		, vec!{"Task_A".to_string()}
	));
	scheduler.add_task(CustomTask::new(
		"Sidetask_C".to_string()
		, 2
		, vec!{"Task_B".to_string()}
	));
	scheduler.add_task(CustomTask::new(
		"Finish".to_string()
		, 1
		, vec!{"Sidetask_B".to_string(), "Sidetask_C".to_string()}
	));
	match scheduler.schedule() {
		Ok(()) => {},
		Err(e) => {eprintln!("Error: {}", e); exit(1);},
	}

}

