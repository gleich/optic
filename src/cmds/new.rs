use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use anyhow::{Context, Result};
use chrono::{Datelike, Local, Month};
use clap::ArgMatches;
use dialoguer::theme::Theme;
use dialoguer::Select;
use num_traits::FromPrimitive;

use crate::conf::{Config, Format};
use crate::{conf, out, util};

#[derive(Debug)]
pub struct File {
	pub name: String,
	pub class: String,
	pub doc_type: String,
	pub format: String,
	pub template: PathBuf,
}

pub fn run(matches: &ArgMatches, prompt_theme: &dyn Theme) -> Result<()> {
	let subcommand_matches = matches.subcommand_matches("new").unwrap();
	let now = Local::now();
	let folder = Path::new(".")
		.join("Pre-Calculus Honors")
		.join(format!("{:?}", Month::from_u32(now.month()).unwrap()))
		.join("Worksheet")
		.join("Test Paper.md");
	println!("{:?}", folder);
	let config = conf::read(false)?;

	let file = ask(&config, &subcommand_matches, prompt_theme)?;
	println!("{:?}", file);
	// create(&subcommand_matches, &file, &folder, prompt_theme)?;

	Ok(())
}

/// Ask the user information
fn ask(config: &Config, matches: &ArgMatches, prompt_theme: &dyn Theme) -> Result<File> {
	// Getting the template that should be used.
	let template_files = fs::read_dir("templates").context("Failed to get templates")?;
	let mut file_names = Vec::new();
	let format = Format::from_str(matches.value_of("format").unwrap())?;
	for raw_fs_object in template_files {
		let fs_object = raw_fs_object?;
		if fs_object.file_type()?.is_file() {
			let file_name = fs_object.file_name().to_str().unwrap().to_string();
			if file_name.ends_with("tex.hbs") && format == Format::LaTeX {
				file_names.push(file_name);
			} else if file_name.ends_with("md.hbs") && format == Format::Markdown {
				file_names.push(file_name)
			}
		}
	}

	if file_names.is_empty() {
		out::problem(&format!(
			"No {} templates found",
			matches.value_of("format").unwrap()
		))
	}

	let selected_template: &String = &file_names
		.get(
			Select::with_theme(prompt_theme)
				.with_prompt("Template")
				.items(&file_names)
				.interact()
				.context("Failed to ask the user for the template file to use")?,
		)
		.unwrap()
		.to_owned();

	Ok(File {
		name: util::flag_or_ask_input(matches, prompt_theme, "name", "Name")?,
		class: util::flag_or_ask_select(
			matches,
			prompt_theme,
			"class",
			"Class",
			config.classes.iter().map(|c| c.name.clone()).collect(),
		)?,
		doc_type: util::flag_or_ask_select(
			matches,
			prompt_theme,
			"type",
			"Type",
			conf::DocType::to_vec(),
		)?,
		// format: matches.value_of("format").unwrap().to_string(),
		format: util::flag_or_ask_select(
			matches,
			prompt_theme,
			"format",
			"Format",
			conf::Format::to_vec(),
		)?,
		template: Path::new(".").join("templates").join(selected_template),
	})
}
