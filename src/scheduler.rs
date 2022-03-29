use std::collections::HashMap;
use std::cmp::Ordering::Equal;

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
pub struct Scheduler<T>
where T: From<i8>
	+ std::clone::Clone
	+ std::marker::Copy
	+ std::ops::Sub::<Output = T>
	+ std::ops::Add<Output = T>
	+ std::fmt::Display
	+ std::fmt::Debug
	+ std::cmp::PartialOrd
	+ std::ops::AddAssign
{
	tasks: HashMap<String, CustomTask<T>>,
	state: SchedulerState,
}

impl <T> Scheduler<T>
where T: From<i8>
	+ std::clone::Clone
	+ std::marker::Copy
	+ std::ops::Sub::<Output = T>
	+ std::ops::Add<Output = T>
	+ std::fmt::Display
	+ std::fmt::Debug
	+ std::cmp::PartialOrd
	+ std::ops::AddAssign
{
	pub fn new() -> Self {
		env_logger::init();
		Scheduler {
			tasks: HashMap::new(),
			state: SchedulerState::Unknown,
		}
	}

	/// Ignites all the calculations.
	pub fn schedule(&mut self) -> Result<(), String>{
		self.calculate()?;
		self.print_output();
		Ok(())
	}

	/// Recalculate all parameters without providing new tasks.
	pub fn calculate(&mut self) -> Result<(), String> {
		self.calculate_es_ef()?;
		self.calculate_ls_lf()?;
		self.state = SchedulerState::Ready;
		Ok(())
	}

	pub fn add_task(&mut self, task: CustomTask<T>) -> Result<(), String> {
		match self.check_task_duplication(&task) {
			Ok(_) => { self.tasks.insert(task.get_id(), task); return Ok(()); },
			Err(e) => { return Err(format!("Failed to add task: {}", e)); }
		}
	}

	/// Sets up a list of tasks, overwriting the already listed ones.
	pub fn fill_tasklist(&mut self, task_list: Vec<CustomTask<T>>) -> Result<(), String> {
		self.state = SchedulerState::Edited;
		let mut new_tasks: HashMap<String, CustomTask<T>> = HashMap::new();
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

	fn check_task_duplication(&self, ref_task: &CustomTask<T>) -> Result<(), String> {
		if self.tasks.contains_key(&ref_task.get_id()) {
			return Err(format!("task ID is already added: {}", ref_task.get_id()));
		} else {
			return Ok(());
		}
	}

	/// Gets a task by it's name.
	pub fn get_task_by_name(&self, task_name: &String) -> Option<&CustomTask<T>> {
		self.tasks.get(task_name)
	}

	/// This one makes the scheduler get the Edited state if the task is found.
	pub fn get_mut_task_by_name(&mut self, task_name: &String)
	-> Option<&mut CustomTask<T>> {
		self.tasks.get_mut(task_name)
	}

	/// Gets dependencies of a task.
	pub fn get_task_dependencies(&self, task_ref: &CustomTask<T>) -> Vec<&CustomTask<T>> {
		let mut dependencies: Vec<&CustomTask<T>> = vec!{};
		for dep_name in &task_ref.get_dependencies() {
			match self.get_task_by_name(dep_name) {
				Some(dep_ref) => dependencies.push(dep_ref),
				None => {},
			}
		}
		dependencies
	}

	/// Gets successors of a task.
	pub fn get_task_successors(&self, task_ref: &CustomTask<T>) -> Vec<&CustomTask<T>> {
		let mut successors: Vec<&CustomTask<T>> = vec!{};
		for (_, task) in &self.tasks {
			if task.get_dependencies().contains(&task_ref.get_id()) {
				successors.push(&task);
			}
		}
		successors
	}

	// TODO: optimize
	fn calculate_es_ef(&mut self) -> Result<(), String> {
		debug!("Calculating ES-EF *************************");
		let mut sorting_list = self.tasks.clone();
		loop {
			for (id, task) in sorting_list.clone() {
				debug!("Task taken: {}", id);
				let deps = self.get_task_dependencies(&task);
				let successor_count = self.get_task_successors(&task).len();

				if deps.len() == 0 {
					match self.get_mut_task_by_name(&id) {
						None => {}, // TODO
						Some(original_task) => {
							original_task.set_early_start(0.into())
								.expect("Could not set early start.");
							original_task.set_early_finish(
								original_task.get_duration()
							).expect("Could not set early finish.");
						}
					}
					debug!("ESEF SP calculated: \n{:?}", self.get_task_by_name(&id));
				} else {
					let mut max_dep_ef: T = 0.into();
					let mut invalid_deps = 0;
					for dep in deps {
						if dep.get_early_finish() == None {
							invalid_deps += 1;
							break;
						}
						match dep.get_early_finish() {
							None => {
								return Err("Uncalculated early finish found.".to_string());
							},
							Some(ef) => {
								//max_dep_ef = max_dep_ef.max(ef);
								if ef > max_dep_ef {
									max_dep_ef = ef;
								}
							},
						}
					}
					debug!("Invalid deps: {}", invalid_deps);
					if invalid_deps == 0 {
						match self.get_mut_task_by_name(&id) {
							None => {},
							Some(original_task) => {
								original_task.set_early_start(max_dep_ef)
									.expect("Could not set early start.");
								original_task.set_early_finish(
									max_dep_ef + original_task.get_duration()
								).expect("Could not set early finish.");
							}
						}
						debug!("ESEF calculated: \n{:?}", self.get_task_by_name(&id));
					} else {
						continue;
					}
				}
				if successor_count == 0 {
					debug!("No successors found for {}", id);
					match self.get_mut_task_by_name(&id) {
						None => { },
						Some(original_task) => {
							original_task.set_late_finish(
								original_task.get_early_finish().unwrap()
							).expect("Could not set late finish.");
							original_task.set_late_start(
								original_task.get_early_start().unwrap()
							).expect("Could not set late start.");
						}
					}
					debug!("ESEF EP calculated: \n{:?}", self.get_task_by_name(&id));
				}
				sorting_list.remove(&id);
				break;
			}
			if sorting_list.len() == 0 { break; }
		}
		Ok(())
	}

	// TODO: optimize
	fn calculate_ls_lf(&mut self) -> Result<(), String> {
		debug!("Calculating LS-LF *************************");
		let mut sorting_list = self.tasks.clone();
		loop {
			for (id, task) in sorting_list.clone() {
				debug!("Task taken: {}", id);
				let successors = self.get_task_successors(&task);
				if successors.len() > 0 {
					let mut min_successor_ls: Option<T> = None;
					for successor in successors {
						match successor.get_late_start() {
							None => {
								min_successor_ls = None;
								break;
							},
							Some(ls) => {
								if ls < 0.into() {
									min_successor_ls = None;
									break;
								}
								match min_successor_ls {
									None => {
										min_successor_ls = Some(ls);
									},
									Some(current_min) => {
										if current_min < ls {
											min_successor_ls = Some(current_min);
										} else {
											min_successor_ls = Some(ls);
										}
									}
								}
							},
						}
						debug!("min ls: {:?}", min_successor_ls);
					}
					match min_successor_ls {
						Some(min) => {
							match self.get_mut_task_by_name(&task.get_id()) {
								None => {}, // TODO
								Some(original_task) => {
									original_task.set_late_finish(min)
										.expect("Could not set late finish.");
									original_task.set_late_start(
										min - original_task.get_duration()
									).expect("Could not set late start.");
								}
							}
							debug!("LSLF calculated: \n{:?}", id);
							sorting_list.remove(&id);
							break;
						},
						None => {},
					}
				} else {
					sorting_list.remove(&id);
					break;
				}
			}
			if sorting_list.len() == 0 { break; }
		}
		Ok(())
	}

	/// Get all the entry points of the graph.
	pub fn get_startpoints(&self) -> Vec<&CustomTask<T>> {
		let mut startpoints: Vec<&CustomTask<T>> = vec!{};
		for (_, task) in &self.tasks {
			if self.get_task_dependencies(&task).len() == 0 {
				startpoints.push(&task);
			}
		}
		startpoints
	}

	/// Get all the end points of the graph.
	pub fn get_endpoints(&self) -> Vec<&CustomTask<T>> {
		let mut endpoints: Vec<&CustomTask<T>> = vec!{};
		for (_, task) in &self.tasks {
			if self.get_task_successors(&task).len() == 0 {
				endpoints.push(&task);
			}
		}
		endpoints
	}

	/// Returns all paths that are able to trace from the given task.
	pub fn get_paths_from_task(&self, start_point: &CustomTask<T>, level: u32)
	-> Vec<Path<T>> {
		let mut head = start_point;
		let mut base_path = Path::new();
		let mut found_paths: Vec<Path<T>> = vec!{};
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
	pub fn get_all_paths(&self) -> Vec<Path<T>> {
		let mut paths: Vec<Path<T>> = vec!{};
		let endpoints = self.get_endpoints();
		for task in endpoints {
			paths.append(&mut self.get_paths_from_task(&task, 0));
		}
		paths
	}

	pub fn get_critical_paths(&self) -> Vec<Path<T>> {
		let mut paths = self.get_all_paths();
		let mut candidates: Vec<Path<T>> = vec!{};
		let mut critical_paths: Vec<Path<T>> = vec!{};
		let mut max_length: T = 0.into();
		for path in &mut paths {
			// reverse paths
			path.reverse_tasks();
			if path.get_total_float() == 0.into() {
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
			let mut ef_list: Vec<Option<T>> = vec!{Some(0.into())};
			let mut max_parallel = 0;
			for (_, task) in &self.tasks {
				match task.get_early_finish() {
					Some(_) => {
						ef_list.push(task.get_early_finish());
					},
					None => {
						return Err(
							format!(
								"Some of the early finish values have not been calculated! Task: {}"
								, task.get_id()
							)
						);
					},
				}
			}
			ef_list.dedup();
			//ef_list.sort();
			ef_list.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
			for ef_idx in 0..ef_list.len() - 1 {
				let section_start = ef_list[ef_idx].unwrap();
				let section_end = ef_list[ef_idx + 1].unwrap();
				let section_parallel = self.tasks.iter().filter(
					| (_, task) |
					task.get_early_start().unwrap() <= section_start
					&& section_end <= task.get_early_finish().unwrap()
					).collect::<Vec<(&String, &CustomTask<T>)>>();
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

