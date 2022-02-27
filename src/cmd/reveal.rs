use std::env::consts;
use std::process::Command;

use task_log::task;

use crate::branch::Branch;
use crate::conf::Config;

pub fn run() {
	let (cmd, args) = match consts::OS {
		"macos" => ("open", vec!["-R"]),
		"windows" => ("explorer", vec![]),
		"linux" => ("xdg-open", vec![]),
		_ => panic!("OS ({}) doesn't have support for this command", consts::OS),
	};
	let config = Config::read().expect("Failed to read configuration file");
	let branches = Branch::get_all(&config).expect("Failed to get all branches");
	let branch = branches.get(0).unwrap();

	task(format!("Opening \"{}\" with {}", branch.name, cmd), || {
		Command::new(cmd)
			.args(&args)
			.arg(&branch.pdf_path)
			.output()
			.expect("Failed to run terminal command ot open the PDF");
	})
}
