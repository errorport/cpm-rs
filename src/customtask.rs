
#[derive(Clone, Debug)]
pub struct CustomTask {
	id: String,
	duration: u32,
	early_start: i64,
	early_finish: i64,
	late_start: i64,
	late_finish: i64,
	dependencies: Vec<String>
}

impl CustomTask {
	pub fn new(_id: String, _duration: u32, _dependencies: Vec<String>) -> Self {
		CustomTask {
			id: _id,
			duration: _duration,
			dependencies: _dependencies,
			early_start: -1,
			early_finish: -1,
			late_start: -1,
			late_finish: -1,
		}
	}

	pub fn get_id(&self) -> String {
		self.id.clone()
	}

	pub fn get_duration(&self) -> u32 {
		self.duration
	}

	pub fn set_duration(&mut self, dur: u32) {
		self.duration = dur;
	}

	pub fn get_dependencies(&self) -> Vec<String> {
		self.dependencies.clone()
	}

	pub fn get_early_start(&self) -> i64 {
		self.early_start
	}

	pub fn set_early_start(&mut self, es: i64) {
		self.early_start = es;
	}

	pub fn get_early_finish(&self) -> i64 {
		self.early_finish
	}

	pub fn set_early_finish(&mut self, ef: i64) {
		self.early_finish = ef;
	}

	pub fn get_late_start(&self) -> i64 {
		self.late_start
	}

	pub fn set_late_start(&mut self, ls: i64) {
		self.late_start = ls;
	}

	pub fn get_late_finish(&self) -> i64 {
		self.late_finish
	}

	pub fn set_late_finish(&mut self, lf: i64) {
		self.late_finish = lf;
	}

	pub fn get_total_float(&self) -> i64 {
		self.late_finish - self.early_finish
	}

}
