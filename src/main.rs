mod branch;
mod cli;
mod cmd;
mod conf;
mod locations;

fn main() {
	let matches = cli::setup();
	match matches.subcommand() {
		Some(("new", _)) => cmd::new::run(),
		_ => unreachable!(),
	}
}
