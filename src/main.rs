use conf::Config;

mod branch;
mod cli;
mod cmd;
mod conf;
mod locations;

fn main() {
	let matches = cli::setup();
	match matches.subcommand() {
		Some(("build", _)) => cmd::build::run(),
		_ => unreachable!(),
	}
}
