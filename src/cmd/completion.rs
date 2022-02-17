use std::io;

use clap::ArgMatches;
use clap_complete::{generate, Shell};

use crate::cli;

pub fn run(args: &ArgMatches) {
	let mut command = cli::setup();
	generate(
		args.value_of_t::<Shell>("shell").expect("Invalid shell"),
		&mut command,
		cli::setup().get_name().to_string(),
		&mut io::stdout(),
	);
}
