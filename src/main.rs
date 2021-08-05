mod cli;
mod cmds;
mod conf;

fn main() {
	let matches = cli::setup();
	if matches.is_present("setup") {
		cmds::setup::run().expect("Failed to run setup command");
	}
}
