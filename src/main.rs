mod branch;
mod cli;
mod cmd;
mod conf;
mod locations;
mod template;

fn main() {
	let matches = cli::setup().get_matches();
	match matches.subcommand() {
		Some(("new", _)) => cmd::new::run(),
		Some(("build", args)) => cmd::build::run(args),
		Some(("watch", args)) => cmd::watch::run(args),
		Some(("open", _)) => cmd::open::run(),
		Some(("reveal", _)) => cmd::reveal::run(),
		Some(("check", _)) => cmd::check::run(),
		Some(("trash", _)) => cmd::trash::run(),
		Some(("completion", args)) => cmd::completion::run(args),
		Some(("commit", _)) => cmd::commit::run(),
		Some(("search", _)) => cmd::search::run(),
		_ => unreachable!(),
	}
}
