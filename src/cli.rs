use clap::{App, AppSettings, ArgMatches};

pub fn setup() -> ArgMatches {
	App::new("kiwi")
		.version("1.0.0")
		.author("Matt Gleich <email@mattglei.ch>")
		.about("🥝 Schoolwork as code")
		.setting(AppSettings::ArgRequiredElseHelp)
		.subcommand(App::new("build").about("Build a branch"))
		.get_matches()
}
