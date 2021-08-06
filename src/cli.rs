use std::path::Path;

use clap::{App, AppSettings, ArgMatches, SubCommand};

use crate::conf;

pub fn setup<'a>() -> ArgMatches<'a> {
	let mut app = App::new("kiwi")
		.version("v1.0.0")
		.author("Matt Gleich <email@mattglei.ch>")
		.about("🥝 Schoolwork as code")
		.setting(AppSettings::ArgRequiredElseHelp);
	if !Path::new(conf::FNAME).exists() {
		app = app.subcommand(SubCommand::with_name("setup").about("Setup a kiwi project"));
	}
	app.get_matches()
}
