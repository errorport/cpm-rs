
use crate::customtask::CustomTask;
use crate::path::Path;

#[derive(Debug)]
pub struct Scheduler {
	tasks: Vec<CustomTask>,
	critical_paths: Vec<Path>,
}

impl Scheduler {
	pub fn new() -> Self {
		Scheduler {
			tasks: vec!{},
			critical_paths: vec!{},
		}
	}

	pub fn schedule(&mut self, task_list: Vec<CustomTask>) {
		self.fill_tasklist(task_list);
		self.calculate_es_ef();
		self.calculate_ls_lf();
		self.find_critical_paths();
		self.compose_output();
	}

	fn fill_tasklist(&mut self, task_list: Vec<CustomTask>) {
		self.tasks = task_list;
	}

	fn get_task_by_name(&self, task_name: &String) -> Option<&CustomTask> {
		for task in &self.tasks {
			if task.id.eq(task_name) {
				return Some(&task);
			}
		}
		None
	}

	fn get_mut_task_by_name(&mut self, task_name: &String) -> Option<&mut CustomTask> {
		for task in &mut self.tasks {
			if task.id.eq(task_name) {
				return Some(task);
			}
		}
		None
	}

	fn get_task_dependencies(&self, task_ref: &CustomTask) -> Vec<&CustomTask> {
		let mut dependencies: Vec<&CustomTask> = vec!{};
		for dep_name in &task_ref.dependencies {
			match self.get_task_by_name(dep_name) {
				Some(dep_ref) => dependencies.push(dep_ref),
				None => {},
			}
		}
		dependencies
	}

	fn get_task_successors(&self, task_ref: &CustomTask) -> Vec<&CustomTask> {
		let mut successors: Vec<&CustomTask> = vec!{};
		for task in &self.tasks {
			if task.dependencies.contains(&task_ref.id) {
				successors.push(&task);
			}
		}
		successors
	}

	fn calculate_es_ef(&mut self) {
		let mut sorting_list = self.tasks.clone();
		loop {
			for task_idx in 0..sorting_list.len() {
				let task = &sorting_list[task_idx];
				let deps = self.get_task_dependencies(&task);
				let mut max_dep_ef = 0;
				for dep in deps {
					if dep.early_finish == -1 {
						continue;
					}
					if dep.early_finish > max_dep_ef {
						max_dep_ef = dep.early_finish;
					}
				}

				let successor_count = self.get_task_successors(&task).len();
				let mut original_task = self.get_mut_task_by_name(&task.id).unwrap();
				original_task.early_start = max_dep_ef;
				original_task.early_finish = max_dep_ef + original_task.duration as i64;

				if successor_count == 0 {
					original_task.late_finish = original_task.early_finish;
					original_task.late_start = original_task.early_start;
				}
				sorting_list.remove(task_idx);
				//println!("ESEF calculated: \n{:?}", original_task);
				break;
			}
			if sorting_list.len() == 0 { break; }
		}
	}

	fn calculate_ls_lf(&mut self) {
		let mut sorting_list = self.tasks.clone();
		loop {
			for task_idx in (0..sorting_list.len()).rev() {
				let task = &sorting_list[task_idx];
				let successors = self.get_task_successors(&task);
				if successors.len() > 0 {
					let mut min_successor_ls = 1 << 32;
					for successor in successors {
						if successor.late_start == -1 {
							continue;
						}
						if successor.late_start < min_successor_ls {
							min_successor_ls = successor.late_start;
						}
					}
					let mut original_task = self.get_mut_task_by_name(&task.id).unwrap();
					original_task.late_finish = min_successor_ls;
					original_task.late_start = min_successor_ls - original_task.duration as i64;
					//println!("LSLF calculated: \n{:?}", original_task);
				}
				sorting_list.remove(task_idx);
				break;
			}
			if sorting_list.len() == 0 { break; }
		}
	}

	fn get_startpoints(&self) -> Vec<&CustomTask> {
		let mut startpoints: Vec<&CustomTask> = vec!{};
		for task in &self.tasks {
			if self.get_task_dependencies(&task).len() == 0 {
				startpoints.push(&task);
			}
		}
		startpoints
	}

	fn get_endpoints(&self) -> Vec<&CustomTask> {
		let mut endpoints: Vec<&CustomTask> = vec!{};
		for task in &self.tasks {
			if self.get_task_successors(&task).len() == 0 {
				endpoints.push(&task);
			}
		}
		endpoints
	}

	fn get_paths_from_task(&self, start_point: &CustomTask, level: u32) -> Vec<Path> {
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

	fn get_all_paths(&self) -> Vec<Path> {
		let mut paths: Vec<Path> = vec!{};
		let endpoints = self.get_endpoints();
		for task in endpoints {
			paths.append(&mut self.get_paths_from_task(&task, 0));
		}
		paths
	}

	fn find_critical_paths(&mut self) {
		let mut paths = self.get_all_paths();
		let mut candidates: Vec<Path> = vec!{};
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
				self.critical_paths.push(path);
			}
		}
		//println!("Critical paths: {:?}", self.critical_paths.len());
	}

	fn get_parallelism(&self) -> u32 {
		let mut ef_list: Vec<i64> = vec!{0};
		let mut max_parallel = 0;
		for task in &self.tasks {
			ef_list.push(task.early_finish);
		}
		ef_list.dedup();
		ef_list.sort();
		for ef_idx in 0..ef_list.len() - 1 {
			let section_start = ef_list[ef_idx];
			let section_end = ef_list[ef_idx + 1];
			let section_parallel = self.tasks.iter().filter(
				|task|
				task.early_start <= section_start
				&& section_end <= task.early_finish
				).collect::<Vec<&CustomTask>>();
			if section_parallel.len() > max_parallel {
				max_parallel = section_parallel.len();
				//println!("section: {} .. {}", section_start, section_end);
				//println!("section tasks: {:?}", section_parallel);
			}
		}
		max_parallel.try_into().unwrap()
	}

	fn compose_output(&self) -> String {
		let output =
			format!("Critical: {}\nMinimum: {}\nParallelism: {}\n"
				, self.critical_paths[0].get_path_string()
				, self.critical_paths[0].get_dur()
				, self.get_parallelism()
			);
		println!("{}", output);
		output
	}

}

