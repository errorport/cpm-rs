
/// Represents a task a.k.a. a node in a batch graph.
#[derive(Clone, Debug)]
pub struct CustomTask<T>
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
	/// Identifier of a task. Should be unique.
	id: String,
	/// Duration of a task.
	duration: T,
	/// Earlyiest possible start for the task. (Calculated)
	early_start: Option<T>,
	/// Earlyiest possible finish for the task. (Calculated)
	early_finish: Option<T>,
	/// Latest possible start for the task. (Calculated)
	late_start: Option<T>,
	/// Latest possible finish for the task. (Calculated)
	late_finish: Option<T>,
	/// Task dependency IDs.
	dependencies: Vec<String>,
}

/*
impl <T> CustomTask<T>
where T: std::clone::Clone
	+ std::ops::Sub::<Output = T>
	+ std::fmt::Display
	+ std::fmt::Debug
	+ std::cmp::PartialOrd
{
	pub fn new(_id: String, _duration: T, _dependencies: Vec<String>) -> Self {
		CustomTask {
			id: _id,
			duration: _duration,
			dependencies: _dependencies,
			early_start: None,
			early_finish: None,
			late_start: None,
			late_finish: None,
		}
	}
}
*/

impl <T> CustomTask<T>
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
	pub fn new(_id: String, _duration: T, _dependencies: Vec<String>) -> Self {
		CustomTask {
			id: _id,
			duration: _duration,
			dependencies: _dependencies,
			early_start: None,
			early_finish: None,
			late_start: None,
			late_finish: None,
		}
	}
}

impl <T> CustomTask<T>
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
	pub fn get_id(&self) -> String {
		self.id.clone()
	}

	pub fn get_duration(&self) -> T {
		self.duration
	}

	pub fn set_duration(&mut self, dur: T) {
		self.duration = dur;
	}

	pub fn get_dependencies(&self) -> Vec<String> {
		self.dependencies.clone()
	}

	pub fn add_dependency(&mut self, _dependency: String) {
		self.dependencies.push(_dependency);
	}

	pub fn add_dependencies(&mut self, _dependencies: &mut Vec<String>) {
		self.dependencies.append(_dependencies);
	}

	pub fn set_dependencies(&mut self, _dependencies: Vec<String>) {
		self.dependencies = _dependencies;
	}

	pub fn get_early_start(&self) -> Option<T> {
		self.early_start
	}

	pub fn set_early_start(&mut self, es: T) -> Result<(), String> {
		if es >= 0.into() {
			self.early_start = Some(es);
		} else {
			return Err(
				format!("Early start have to be greater or equal 0! ES: {:?}", es)
			);
		}
		Ok(())
	}

	pub fn get_early_finish(&self) -> Option<T> {
		self.early_finish
	}

	pub fn set_early_finish(&mut self, ef: T) -> Result<(), String> {
		if ef >= 0.into() {
			self.early_finish = Some(ef);
		} else {
			return Err(
				format!("Early finish have to be greater or equal 0! EF: {:?}", ef)
			);
		}
		Ok(())
	}

	pub fn get_late_start(&self) -> Option<T> {
		self.late_start
	}

	pub fn set_late_start(&mut self, ls: T) -> Result<(), String> {
		if ls >= 0.into() {
			self.late_start = Some(ls);
		} else {
			return Err(
				format!("Late start have to be greater or equal 0! LS: {:?}", ls)
			);
		}
		Ok(())
	}

	pub fn get_late_finish(&self) -> Option<T> {
		self.late_finish
	}

	pub fn set_late_finish(&mut self, lf: T) -> Result<(), String> {
		if lf > 0.into() {
			self.late_finish = Some(lf);
		} else {
			return Err(
				format!("Late finish have to be greater or equal 0! LF: {:?}", lf)
			);
		}
		Ok(())
	}

	pub fn get_total_float(&self) -> Result<T, String> {
		let (lf, ef);
		match self.late_finish {
			None => {
				return Err(
					format!("Late finish has not been calculated in task: {}", self.id)
				);
			},
			Some(_lf) => {lf = _lf},
		};
		match self.early_finish {
			None => {
				return Err(
					format!("Early finish has not been calculated in task: {}", self.id)
				);
			},
			Some(_ef) => {ef = _ef},
		};
		let tf = lf - ef;
		if tf < 0.into() {
			return Err(format!("Total float is under 0 in task: {}", self.id));
		}
		Ok(tf)
	}

}
