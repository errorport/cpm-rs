
#[cfg(test)]
mod tests {
	use std::process::exit;

	use crate::Scheduler;
	use crate::CustomTask;

	#[test]
	fn cpm_case_1() {
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
		let endpoints = scheduler.get_endpoints();
		assert_eq!(endpoints[0].get_id(), "Finish");
		assert_eq!(endpoints[0].get_early_start(), Some(4));
		assert_eq!(endpoints[0].get_early_finish(), Some(5));
		assert_eq!(endpoints[0].get_total_float(), Ok(0));
		assert_eq!(scheduler.get_parallelism(), Ok(2));
	}
}

