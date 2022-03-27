use std::fs;
use regex::Regex;

use crate::customtask::CustomTask;

/**
 * Pattern for task names.
 */
static TASK_NAME_PATTERN: &str
	= r"a-zA-Z-_.0-9";

/**
 * Pattern for dependency names.
 * NAME_PAT should be replaced by TASK_NAME_PATTERN.
 */
static DEPENDENCY_NAME_PATTERN: &str
	= r"([NAME_PAT]+)";

/**
 * Pattern for task definition.
 * LIST_PAT should be replaced by TASK_LIST_PATTERN.
 * NAME_PAT should be replaced by TASK_NAME_PATTERN.
 */
static TASK_DEFINITION_PATTERN: &str
	= r"([NAME_PAT]+)\s*\(([-+]?\d+)\)(\s+after\s+\[([NAME_PAT\s,]*)\])?";

pub fn parse_input_file(filename: &String) -> Result<Vec<CustomTask<i64>>, String> {
	let task_def_str = TASK_DEFINITION_PATTERN.to_string()
		.replace("NAME_PAT", TASK_NAME_PATTERN);
	let dependency_def_str = DEPENDENCY_NAME_PATTERN.to_string()
		.replace("NAME_PAT", TASK_NAME_PATTERN);

	let task_definition: Regex = Regex::new(task_def_str.as_str()).unwrap();
	let dependency_definition: Regex = Regex::new(dependency_def_str.as_str()).unwrap();

	let mut task_list: Vec<CustomTask<i64>> = vec!{};

	//println!("Input file: {}", filename);
	let contents: String;
	match fs::read_to_string(filename) {
		Ok(file_content) => {contents = file_content;},
		Err(e) => { return Err(format!("Could not read file: {}\n\r{}", filename, e)); },
	};

	let mut leftover = contents.clone();

	for cap in task_definition.captures_iter(&contents) {
		let id = cap[1].to_string();
		let duration: u32;
		match cap[2].parse::<u32>() {
			Ok(dur) => duration = dur,
			Err(_) => {
				return Err(find_pattern_error(&cap[2], contents.clone()));
			},
		}

		let dependency_str = cap.get(4).map_or("", |m| m.as_str());
		let mut dependencies: Vec<String> = vec!{};
		for dep_name in dependency_definition.captures_iter(&dependency_str) {
			dependencies.push(dep_name[1].to_string());
		}

		let task = CustomTask::new(id, duration.into(), dependencies);
		//println!("Task: {:?}", task);
		leftover = leftover.replacen(&cap[0], "", 1);
		task_list.push(task);
	}
	if leftover.replace("\n", "").len() > 0 {
		return Err(find_pattern_error(&leftover, contents.clone()));
	}
	Ok(task_list)
}

pub fn find_pattern_error(error_str: &str, contents: String) -> String {
	let output: String;
	let error_lines: Vec<&str> = error_str.split('\n').collect();
	for error_line in error_lines {
		if error_line.len() > 0 {
			//println!("Error line: {}", error_line);
			let pos = contents.find(error_line).unwrap();
			let (left_split, _) = contents.split_at(pos);
			let lines = left_split.split('\n').collect::<Vec<&str>>();
			let line_num = lines.len();
			let column = lines.last().unwrap().len() + 1;
			output = format!("line {}, column {}\n", line_num, column);
			return output;
		}
	}
	return format!("failed to parse errorous lines.");
}

