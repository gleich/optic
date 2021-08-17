use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use anyhow::Result;
use clap::ArgMatches;
use dialoguer::theme::Theme;

use crate::conf::{Config, DocType, Format};
use crate::{cli, conf, inject};

#[derive(Debug)]
struct File {
	name: String,
	class: String,
	doc_type: DocType,
	format: Format,
	branch_template_path: PathBuf,
	root_template_path: PathBuf,
}

pub fn run(matches: &ArgMatches, prompt_theme: &dyn Theme) {
	let subcommand_matches = matches.subcommand_matches("new").unwrap();
	let config = conf::read(false).expect("Failed to read from the configuration file");
	let file = ask(&config, &subcommand_matches, prompt_theme)
		.expect("Failed to ask user for info about the branch");
	create(&file, &config).expect("Failed to create the file");
}

/// Ask the user information
fn ask(config: &Config, matches: &ArgMatches, prompt_theme: &dyn Theme) -> Result<File> {
	let format = conf::Format::from_str(&cli::flag_or_ask_select(
		matches,
		prompt_theme,
		"format",
		"Format",
		conf::Format::to_vec(),
	)?)?;

	Ok(File {
		name: cli::flag_or_ask_input(matches, prompt_theme, "name", "Name")?,
		class: cli::flag_or_ask_select(
			matches,
			prompt_theme,
			"class",
			"Class",
			config.classes.iter().map(|c| c.name.clone()).collect(),
		)?,
		doc_type: conf::DocType::from_str(&cli::flag_or_ask_select(
			matches,
			prompt_theme,
			"type",
			"Type",
			conf::DocType::to_vec(),
		)?)
		.unwrap(),
		branch_template_path: Path::new(&cli::flag_or_ask_select(
			matches,
			prompt_theme,
			"branch",
			"Branch template",
			conf::list_templates(&format, &conf::TemplateType::Branch)?,
		)?)
		.to_path_buf(),
	})
}

fn create(file: &File, config: &Config) -> Result<()> {
	// Create file contents
	let path = Path::new("templates")
		.join(conf::TemplateType::Branch.to_string())
		.join(&file.branch_template_path);
	let contents = inject::BaseData {
		doc_name: &file.name,
		class_name: &file.class,
		format: &file.format,
		config,
	}
	.inject_into(&fs::read_to_string(path)?)?;
	println!("{}", contents);
	Ok(())
}
