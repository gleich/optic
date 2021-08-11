use dialoguer::theme::ColorfulTheme;

mod cli;
mod cmds;
mod conf;
mod out;

fn main() {
	let prompt_theme = ColorfulTheme::default();
	let matches = cli::setup().expect("Failed to setup CLI");
	if matches.is_present("setup") {
		cmds::setup::run(&prompt_theme);
	} else if matches.is_present("new") {
		cmds::new::run(&matches, &prompt_theme).expect("Failed to run new command");
	}
}
