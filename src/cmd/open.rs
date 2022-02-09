use crate::branch::Branch;
use crate::conf::Config;
use crate::out::task;

pub fn run() {
	let config = Config::read().expect("Failed to read from configuration file");
	let branches = Branch::get_all(&config).expect("Failed to get all branches");

	task("Opening with editor", || {
		branches
			.get(0)
			.unwrap()
			.open(&config)
			.expect("Failed to open branch with editor");
	});
}
