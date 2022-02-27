use clap::ArgMatches;
use task_log::task;

use crate::branch::Branch;
use crate::conf::Config;

pub fn run(args: &ArgMatches) {
	let config = Config::read().expect("Failed to read from configuration file");

	let branches = task("Collecting branches", || -> Vec<Branch> {
		Branch::get_all(&config).expect("Failed to get all branches")
	});

	task("Building branch", || {
		branches
			.get(0)
			.unwrap()
			.build(&config, &(config.latexmk || args.is_present("latexmk")))
			.expect("Failed to build");
	});
}
