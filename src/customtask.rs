
#[derive(Clone, Debug)]
pub struct CustomTask {
	pub id: String,
	pub duration: u32,
	pub early_start: i64,
	pub early_finish: i64,
	pub late_start: i64,
	pub late_finish: i64,
	pub dependencies: Vec<String>
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

	pub fn get_total_float(&self) -> i64 {
		self.late_finish - self.early_finish
	}
}
