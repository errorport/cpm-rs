use crate::customtask::CustomTask;

/// Represents a path of tasks.
/// It is a copy of an original path in the graph.
#[derive(Debug, Clone)]
pub struct Path<T>
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
	/// Vector of tasks.
	tasks: Vec<CustomTask<T>>,
}

impl <T> Path<T>
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
		Path {
			tasks: vec!{},
		}
	}

	pub fn new_from_vec(_tasks: Vec<CustomTask<T>>) -> Self {
		Path {
			tasks: _tasks.clone(),
		}
	}

	pub fn add_task(&mut self, task: &CustomTask<T>) {
		self.tasks.push(task.clone());
	}

	pub fn join_path(&mut self, path: &Path<T>) {
		for task in &path.tasks {
			self.add_task(&task);
		}
	}

	/// Gets the total float of the path.
	/// If the total float is zero, than the path is probably a
	/// critical path.
	pub fn get_total_float(&self) -> T {
		let mut total_float: T = 0.into();
		for task in &self.tasks {
			total_float += task.get_total_float().unwrap(); // TODO check this unwrap
		}
		total_float
	}

	/// Reverse the order of the tasks in the path.
	/// It does not change the order of the tasks in the Scheduler!
	pub fn reverse_tasks(&mut self) {
		self.tasks.reverse();
	}

	/// Returns a text that shows the order of tasks in the path.
	pub fn get_path_string(&self) -> String {
		let mut path_string: String = "".to_string();
		for task_idx in 0..self.tasks.len() {
			path_string = format!(
				"{}{}({})->"
				, path_string
				, self.tasks[task_idx].get_id()
				, self.tasks[task_idx].get_duration()
			);
		}
		path_string
	}

	/// Gets the duration of the path.
	pub fn get_dur(&self) -> T {
		let mut dur = 0.into();
		for task in &self.tasks {
			dur += task.get_duration()
		}
		dur
	}

}

