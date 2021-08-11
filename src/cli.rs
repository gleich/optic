use std::path::Path;

use anyhow::Result;
use clap::{App, AppSettings, Arg, ArgMatches};

use crate::cmds::new::DocType;
use crate::conf;

pub fn setup() -> Result<ArgMatches> {
	// Getting a list of the classes
	let config = conf::read(true)?;
	let classes: Vec<String> = config.classes.into_iter().map(|c| c.name).collect();

	let doc_types = DocType::to_vec();

	let mut app = App::new("kiwi")
		.version("1.0.0")
		.author("Matt Gleich <email@mattglei.ch>")
		.about("🥝 Schoolwork as code")
		.setting(AppSettings::ArgRequiredElseHelp)
		.subcommand(
			App::new("new")
				.about("Create a new document")
				.arg(
					Arg::new("name")
						.long("name")
						.short('n')
						.about("Name of the file")
						.takes_value(true)
						.value_name("NAME"),
				)
				.arg(
					Arg::new("class")
						.long("class")
						.short('c')
						.about("Name of the class")
						.takes_value(true)
						.value_name("CLASS")
						.possible_values(&classes.iter().map(|s| s as &str).collect::<Vec<&str>>()),
				)
				.arg(
					Arg::new("type")
						.long("type")
						.short('t')
						.about("Document type")
						.takes_value(true)
						.value_name("TYPE")
						.possible_values(
							&doc_types.iter().map(|s| s as &str).collect::<Vec<&str>>(),
						),
				),
		);
	if !Path::new(conf::FNAME).exists() {
		app = app.subcommand(App::new("setup").about("Setup a kiwi project"));
	}
	Ok(app.get_matches())
}
