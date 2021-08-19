use cmds::{build, new, setup};
use out::custom_dialoguer_theme;

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
		new::run(&matches, &prompt_theme);
	} else if matches.is_present("build") {
		build::run(&matches)
	}
}
