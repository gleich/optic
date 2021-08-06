mod cli;
mod cmds;
mod conf;
mod out;

fn main() {
	let matches = cli::setup();
	if matches.is_present("setup") {
		cmds::setup::run();
	}
}
