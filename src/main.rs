use conf::Config;

mod cli;
mod cmd;
mod conf;
mod format;

fn main() {
	let matches = cli::setup();
	match matches.subcommand() {
		Some(("build", _)) => cmd::build::run(),
		_ => unreachable!(),
	}
}
