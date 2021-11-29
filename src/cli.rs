use std::path::Path;

use anyhow::{Context, Result};
use clap::{App, AppSettings, Arg, ArgMatches};
use dialoguer::theme::Theme;
use dialoguer::{FuzzySelect, Input};
use strum::VariantNames;

use crate::conf::{self, DocType, Format};
use crate::out::got_value;

pub fn setup() -> Result<ArgMatches> {
	let config = conf::read(true).context("Failed to read from configuration file")?;
	let classes: &Vec<String> = &config.classes.into_iter().map(|c| c.name).collect();

	let default_format = config.default_format.to_string();
	let root_files =
		conf::list_templates(&Format::LaTeX, &conf::TemplateType::Root).unwrap_or_default();

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
						.possible_values(DocType::VARIANTS),
				)
				.arg(
					Arg::new("format")
						.long("format")
						.short('f')
						.value_name("FORMAT")
						.about("Format that the file should be created in")
						.takes_value(true)
						.possible_values(Format::VARIANTS)
						.default_value(&default_format),
				)
				.arg(
					Arg::new("branch")
						.long("branch")
						.short('b')
						.value_name("PATH")
						.about("Filename of the branch template file")
						.takes_value(true),
				)
				.arg(
					Arg::new("root")
						.long("root")
						.short('r')
						.value_name("PATH")
						.about("Filename of the root file")
						.takes_value(true)
						.possible_values(
							&root_files.iter().map(|s| s as &str).collect::<Vec<&str>>(),
						),
				),
		)
		.subcommand(
			App::new("build").about("Build a branch").arg(
				Arg::new("path")
					.about("Path to the file. Defaults to the most recent branch to be updated")
					.value_name("PATH")
					.index(1)
					.required(false),
			),
		)
		.subcommand(
			App::new("check")
				.about("Check to see if any branches have PDFs that haven't been created/updated"),
		)
		.subcommand(App::new("commit").about("Commit all modified/untracked branches"));
	if !Path::new(conf::FNAME).exists() {
		app = app.subcommand(App::new("setup").about("Setup a kiwi project"));
	}
	Ok(app.get_matches())
}

pub fn flag_or_input(
	matches: &ArgMatches,
	prompt_theme: &dyn Theme,
	flag_name: &str,
	prompt: &str,
) -> Result<String> {
	let flag = matches.value_of(flag_name);
	if flag.is_none() {
		return Ok(Input::<String>::with_theme(prompt_theme)
			.with_prompt(prompt)
			.interact()
			.context(format!("Failed to get {}", prompt))?);
	}
	let value = flag.unwrap();
	got_value(prompt, value);
	Ok(value.to_string())
}

pub fn flag_or_select(
	matches: &ArgMatches,
	prompt_theme: &dyn Theme,
	flag_name: &str,
	prompt: &str,
	options: Vec<&str>,
) -> Result<String> {
	let flag = matches.value_of(flag_name);
	if flag.is_none() {
		return Ok(options
			.get(
				FuzzySelect::with_theme(prompt_theme)
					.with_prompt(prompt)
					.default(0)
					.items(&options)
					.interact()
					.context(format!("Failed to ask for {}", prompt))?,
			)
			.unwrap()
			.to_string());
	}
	let value = flag.unwrap();
	got_value(prompt, value);
	Ok(value.to_string())
}
