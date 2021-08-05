use clap::{App, ArgMatches, SubCommand};

pub fn setup<'a>() -> ArgMatches<'a> {
	App::new("kiwi")
		.version("v1.0.0")
		.author("Matt Gleich <email@mattglei.ch>")
		.about("🏫 Schoolwork as code")
		.subcommand(SubCommand::with_name("setup").about("Setup a kiwi project"))
		.get_matches()
}
