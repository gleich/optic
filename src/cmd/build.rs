use clap::ArgMatches;

use crate::branch::Branch;
use crate::conf::Config;
use crate::out::task;

pub fn run(args: &ArgMatches) {
	let config = Config::read().expect("Failed to read from configuration file");

	let mut branches = Vec::new();
	task("Collecting branches", || {
		branches = Branch::get_all(&config).expect("Failed to get all branches");
	});

	task("Building branch", || {
		branches
			.get(0)
			.unwrap()
			.build(&config, &(config.latexmk || args.is_present("latexmk")))
			.expect("Failed to build");
	});
}
