use clap::{App, AppSettings, ArgMatches};

pub fn setup() -> ArgMatches {
	App::new("kiwi")
		.version("1.0.0")
		.author("Matt Gleich <email@mattglei.ch>")
		.about("🥝 Schoolwork as code")
		.setting(AppSettings::ArgRequiredElseHelp)
		.subcommand(App::new("new").about("Create a new branch"))
		.get_matches()
}
