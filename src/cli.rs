use clap::{App, AppSettings, ArgMatches};

pub fn setup() -> ArgMatches {
	App::new("optic")
		.version("1.0.0")
		.author("Matt Gleich <email@mattglei.ch>")
		.about("Schoolwork as code")
		.setting(AppSettings::ArgRequiredElseHelp)
		.subcommand(App::new("new").about("Create a new branch"))
		.subcommand(App::new("build").about("Build a branch"))
		.subcommand(App::new("watch").about("View a branch and build it on change"))
		.subcommand(App::new("open").about("Open a branch in an editor"))
		.subcommand(App::new("reveal").about("Open a branch PDF in finder"))
		.get_matches()
}
