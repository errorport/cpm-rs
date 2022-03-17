use crate::customtask::CustomTask;

#[derive(Debug, Clone)]
pub struct Path {
	total_dur: u32,
	tasks: Vec<CustomTask>,
}

impl Path {
	pub fn new() -> Self {
		Path {
			total_dur: 0,
			tasks: vec!{},
		}
	}

	pub fn add_task(&mut self, task: &CustomTask) {
		self.total_dur += task.duration;
		self.tasks.push(task.clone());
	}

	pub fn join_path(&mut self, path: &Path) {
		for task in &path.tasks {
			self.add_task(&task);
		}
	}

	pub fn get_total_float(&self) -> i64 {
		let mut total_float: i64 = 0;
		for task in &self.tasks {
			total_float += task.get_total_float();
		}
		total_float
	}

	pub fn reverse_tasks(&mut self) {
		self.tasks.reverse();
	}

	pub fn get_path_string(&self) -> String {
		let mut path_string: String = "".to_string();
		for task_idx in 0..self.tasks.len() - 1 {
			path_string = format!("{}{}->", path_string, self.tasks[task_idx].id);
		}
		path_string = format!("{}{}", path_string, self.tasks.last().unwrap().id);
		path_string
	}

	pub fn get_dur(&self) -> u32 {
		self.total_dur
	}

}

