use std::collections::HashMap;
use crate::customtask::CustomTask;
use crate::path::Path;

/// Different state indicators of Scheduler.
#[derive(Debug, PartialEq)]
enum SchedulerState {
	/// Uninitialized.
	Unknown,
	/// Something has been changed, it should be recalculated.
	Edited,
	/// All calculations are finished, results can be extracted.
	Ready,
}

/// The scheduler implements the basic functionality to
/// calculate critical paths plus the number of
/// maximum parallel jobs at a time.
#[derive(Debug)]
pub struct Scheduler {
	tasks: HashMap<String, CustomTask>,
	state: SchedulerState,
}

impl Scheduler {
	pub fn new() -> Self {
		Scheduler {
			tasks: HashMap::new(),
			state: SchedulerState::Unknown,
		}
	}

	/// Ignites all the calculations.
	pub fn schedule(&mut self) -> Result<(), String>{
		self.calculate();
		self.print_output();
		return Ok(());
	}

	/// Recalculate all parameters without providing new tasks.
	pub fn calculate(&mut self) {
		self.calculate_es_ef();
		self.calculate_ls_lf();
		self.state = SchedulerState::Ready;
	}

	pub fn add_task(&mut self, task: CustomTask) -> Result<(), String> {
		match self.check_task_duplication(&task) {
			Ok(_) => { self.tasks.insert(task.get_id(), task); return Ok(()); },
			Err(e) => { return Err(format!("Failed to add task: {}", e)); }
		}
	}

	/// Sets up a list of tasks, overwriting the already listed ones.
	pub fn fill_tasklist(&mut self, task_list: Vec<CustomTask>) -> Result<(), String> {
		self.state = SchedulerState::Edited;
		let mut new_tasks: HashMap<String, CustomTask> = HashMap::new();
		for task in &task_list {
			if new_tasks.contains_key(&task.get_id()) {
				return Err(format!("task ID duplication: {}", task.get_id()));
			} else {
				new_tasks.insert(task.get_id(), task.clone());
			}
		}
		self.tasks = new_tasks;
		Ok(())
	}

	fn check_task_duplication(&self, ref_task: &CustomTask) -> Result<(), String> {
		if self.tasks.contains_key(&ref_task.get_id()) {
			return Err(format!("task ID is already added: {}", ref_task.get_id()));
		} else {
			return Ok(());
		}
	}

	/// Gets a task by it's name.
	pub fn get_task_by_name(&self, task_name: &String) -> Option<&CustomTask> {
		self.tasks.get(task_name)
	}

	/// This one makes the scheduler get the Edited state if the task is found.
	pub fn get_mut_task_by_name(&mut self, task_name: &String) -> Option<&mut CustomTask> {
		self.tasks.get_mut(task_name)
	}

	/// Gets dependencies of a task.
	pub fn get_task_dependencies(&self, task_ref: &CustomTask) -> Vec<&CustomTask> {
		let mut dependencies: Vec<&CustomTask> = vec!{};
		for dep_name in &task_ref.get_dependencies() {
			match self.get_task_by_name(dep_name) {
				Some(dep_ref) => dependencies.push(dep_ref),
				None => {},
			}
		}
		dependencies
	}

	/// Gets successors of a task.
	pub fn get_task_successors(&self, task_ref: &CustomTask) -> Vec<&CustomTask> {
		let mut successors: Vec<&CustomTask> = vec!{};
		for (_, task) in &self.tasks {
			if task.get_dependencies().contains(&task_ref.get_id()) {
				successors.push(&task);
			}
		}
		successors
	}

	// TODO: optimize
	fn calculate_es_ef(&mut self) {
		let mut sorting_list = self.tasks.clone();
		loop {
			for (id, task) in sorting_list.clone() {
				//println!("Task taken: {}", id);
				let deps = self.get_task_dependencies(&task);
				let successor_count = self.get_task_successors(&task).len();

				if deps.len() == 0 {
					let original_task = self.get_mut_task_by_name(&id).unwrap();
					original_task.set_early_start(0);
					original_task.set_early_finish(
						original_task.get_duration() as i64
					);
					//println!("ESEF SP calculated: \n{:?}", self.get_task_by_name(&id));
				} else {
					let mut max_dep_ef = 0;
					let mut invalid_deps = 0;
					for dep in deps {
						if dep.get_early_finish() < 0 {
							invalid_deps += 1;
							break;
						}
						if dep.get_early_finish() > max_dep_ef {
							max_dep_ef = dep.get_early_finish();
						}
					}
					//println!("Invalid deps: {}", invalid_deps);
					if invalid_deps == 0 {
						let original_task = self.get_mut_task_by_name(&id).unwrap();
						original_task.set_early_start(max_dep_ef);
						original_task.set_early_finish(
							max_dep_ef + original_task.get_duration() as i64
						);
						//println!("ESEF calculated: \n{:?}", self.get_task_by_name(&id));
					} else {
						continue;
					}
				}
				if successor_count == 0 {
					let original_task = self.get_mut_task_by_name(&id).unwrap();
					original_task.set_late_finish(original_task.get_early_finish());
					original_task.set_late_start(original_task.get_early_start());
					//println!("ESEF EP calculated: \n{:?}", self.get_task_by_name(&id));
				}
				sorting_list.remove(&id);
				break;
			}
			if sorting_list.len() == 0 { break; }
		}
	}

	// TODO: optimize
	fn calculate_ls_lf(&mut self) {
		let mut sorting_list = self.tasks.clone();
		loop {
			for (id, task) in sorting_list.clone() {
				let successors = self.get_task_successors(&task);
				let mut invalid_successors = 0;
				if successors.len() > 0 {
					let mut min_successor_ls = 1 << 32;
					for successor in successors {
						if successor.get_late_start() == -1 {
							invalid_successors += 1;
							break;
						}
						if successor.get_late_start() < min_successor_ls {
							min_successor_ls = successor.get_late_start();
						}
					}
					if invalid_successors == 0 {
						let original_task
							= self.get_mut_task_by_name(&task.get_id()).unwrap();
						original_task.set_late_finish(min_successor_ls);
						original_task.set_late_start(
							min_successor_ls - original_task.get_duration() as i64
						);
						//println!("LSLF calculated: \n{:?}", original_task);
						sorting_list.remove(&id);
						break;
					}
				} else {
					sorting_list.remove(&id);
					break;
				}
			}
			if sorting_list.len() == 0 { break; }
		}
	}

	/// Get all the entry points of the graph.
	pub fn get_startpoints(&self) -> Vec<&CustomTask> {
		let mut startpoints: Vec<&CustomTask> = vec!{};
		for (_, task) in &self.tasks {
			if self.get_task_dependencies(&task).len() == 0 {
				startpoints.push(&task);
			}
		}
		startpoints
	}

	/// Get all the end points of the graph.
	pub fn get_endpoints(&self) -> Vec<&CustomTask> {
		let mut endpoints: Vec<&CustomTask> = vec!{};
		for (_, task) in &self.tasks {
			if self.get_task_successors(&task).len() == 0 {
				endpoints.push(&task);
			}
		}
		endpoints
	}

	/// Returns all paths that are able to trace from the given task.
	pub fn get_paths_from_task(&self, start_point: &CustomTask, level: u32) -> Vec<Path> {
		let mut head = start_point;
		let mut base_path = Path::new();
		let mut found_paths: Vec<Path> = vec!{};
		base_path.add_task(start_point);
		loop {
			let deps = self.get_task_dependencies(&head);
			if deps.len() > 1 {
				for dep in &deps {
					let sub_paths = self.get_paths_from_task(&dep, level + 1);
					for path in sub_paths {
						let mut concatenated_path = base_path.clone();
						concatenated_path.join_path(&path);
						found_paths.push(concatenated_path);
					}
				}
				break;
			}
			if self.get_task_dependencies(&head).len() == 0 {
				found_paths.push(base_path);
				break;
			}
			if deps.len() == 1 {
				base_path.add_task(deps[0]);
				head = deps[0];
			}
		}
		found_paths
	}

	/// Gets all the paths in the graph.
	/// Attention! Does not check the possible cycles in dependencies!
	/// TODO: make it parallel.
	pub fn get_all_paths(&self) -> Vec<Path> {
		let mut paths: Vec<Path> = vec!{};
		let endpoints = self.get_endpoints();
		for task in endpoints {
			paths.append(&mut self.get_paths_from_task(&task, 0));
		}
		paths
	}

	pub fn get_critical_paths(&self) -> Vec<Path> {
		let mut paths = self.get_all_paths();
		let mut candidates: Vec<Path> = vec!{};
		let mut critical_paths: Vec<Path> = vec!{};
		let mut max_length: u32 = 0;
		for path in &mut paths {
			// reverse paths
			path.reverse_tasks();
			if path.get_total_float() == 0 {
				candidates.push(path.clone());
				if path.get_dur() > max_length {
					max_length = path.get_dur();
				}
			}
		}
		for path in candidates {
			if path.get_dur() == max_length {
				critical_paths.push(path);
			}
		}
		critical_paths
		//println!("Critical paths: {:?}", self.critical_paths.len());
	}

	/// Calculates the maximum number of parallel jobs at a time.
	/// Scheduler has to be in ready state.
	pub fn get_parallelism(&self) -> Result<u32, String> {
		if self.state == SchedulerState::Ready {
			let mut ef_list: Vec<i64> = vec!{0};
			let mut max_parallel = 0;
			for (_, task) in &self.tasks {
				ef_list.push(task.get_early_finish());
			}
			ef_list.dedup();
			ef_list.sort();
			for ef_idx in 0..ef_list.len() - 1 {
				let section_start = ef_list[ef_idx];
				let section_end = ef_list[ef_idx + 1];
				let section_parallel = self.tasks.iter().filter(
					| (_, task) |
					task.get_early_start() <= section_start
					&& section_end <= task.get_early_finish()
					).collect::<Vec<(&String, &CustomTask)>>();
				if section_parallel.len() > max_parallel {
					max_parallel = section_parallel.len();
					//println!("section: {} .. {}", section_start, section_end);
					//println!("section tasks: {:?}", section_parallel);
				}
			}
			return Ok(max_parallel.try_into().unwrap());
		} else {
			return Err(
				format!("Scheduler is in state {:?} instead of being ready.", self.state)
			);
		}
	}

	fn print_output(&self) {
		let critical_paths = self.get_critical_paths();
		println!("Critical paths: {}", critical_paths.len());
		for path in &critical_paths {
			println!("\tCritical path: {}", path.get_path_string());
			println!("\tPath duration: {}", path.get_dur());
		}
		println!("Number of maximum parallel jobs: {}", self.get_parallelism().unwrap());
	}

}

