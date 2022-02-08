use std::path::PathBuf;

use crate::branch::Branch;
use crate::conf::Config;

pub fn run() {
	let config = Config::read().expect("Failed to read from configuration file");
	println!(
		"{:?}",
		Branch::parse(
			PathBuf::from("./docs/AP Physics II/January/Note/Fluids Midterm Review.tex"),
			&config
		)
		.expect("Failed to parse branch"),
	);
}
