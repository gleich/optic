use task_log::task;

use crate::branch::Branch;
use crate::conf::Config;

pub fn run() {
	let config = Config::read().expect("Failed to read configuration file");
	let branches = Branch::get_all(&config).expect("Failed to get all branches");
	let branch = branches.get(0).unwrap();

	task(format!("Revealing {}", branch.name), || {
		branch
			.reveal(&config, true)
			.expect("Failed to reveal branch");
	})
}
