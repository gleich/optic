use cmds::{build, check, new, setup};
use out::custom_dialoguer_theme;

mod branches;
mod cli;
mod cmds;
mod conf;
mod inject;
mod out;

fn main() {
	let prompt_theme = custom_dialoguer_theme();
	let matches = cli::setup().expect("Failed to setup CLI");
	if matches.is_present("setup") {
		setup::run(&prompt_theme);
	} else if matches.is_present("new") {
		new::run(matches.subcommand_matches("new").unwrap(), &prompt_theme);
	} else if matches.is_present("build") {
		build::run(matches.subcommand_matches("build").unwrap());
	} else if matches.is_present("check") {
		check::run(&prompt_theme);
	}
}
