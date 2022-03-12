use clap::{Arg, Command};
use clap_complete::Shell;

pub fn setup() -> Command<'static> {
	Command::new("optic")
		.version("1.0.0")
		.author("Matt Gleich <email@mattglei.ch>")
		.about("Schoolwork as code")
		.arg_required_else_help(true)
		.subcommand(Command::new("new").about("Create a new branch"))
		.subcommand(
			Command::new("build").about("Build a branch").arg(
				Arg::new("latexmk")
					.long("latexmk")
					.help("Use latexmk instead of pdflatex to build the PDF")
					.takes_value(false),
			),
		)
		.subcommand(Command::new("watch").about("View a branch and build it on change"))
		.subcommand(Command::new("open").about("Open a branch in an editor"))
		.subcommand(Command::new("reveal").about("Open a branch PDF in finder"))
		.subcommand(
			Command::new("check").about("Check to see if any branches don't have up-to-date PDFs"),
		)
		.subcommand(Command::new("trash").about("Move branch to trash can"))
		.subcommand(
			Command::new("completion")
				.about("Generate shell completion for optic")
				.arg(Arg::new("shell").possible_values(Shell::possible_values())),
		)
		.subcommand(Command::new("commit").about("Commit uncommitted branches"))
		.subcommand(Command::new("search").about("Search for a branch"))
}
