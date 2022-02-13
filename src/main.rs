mod branch;
mod cli;
mod cmd;
mod conf;
mod locations;
mod out;
mod template;

fn main() {
	let matches = cli::setup();
	match matches.subcommand() {
		Some(("new", _)) => cmd::new::run(),
		Some(("build", _)) => cmd::build::run(),
		Some(("watch", _)) => cmd::watch::run(),
		Some(("open", _)) => cmd::open::run(),
		Some(("reveal", _)) => cmd::reveal::run(),
		Some(("check", _)) => cmd::check::run(),
		_ => unreachable!(),
	}
}
