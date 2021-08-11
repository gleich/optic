use std::path::Path;

use anyhow::{Context, Result};
use chrono::{Datelike, Local, Month};
use clap::ArgMatches;
use dialoguer::theme::Theme;
use dialoguer::Input;
use num_traits::FromPrimitive;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, ToString};

#[derive(ToString, EnumIter)]
pub enum DocType {
	Worksheet,
	Note,
	Assessment,
	Paper,
	Lab,
	Other,
}

impl DocType {
	pub fn to_vec() -> Vec<String> { DocType::iter().map(|t| t.to_string()).collect() }
}

#[derive(Debug)]
pub struct File {
	pub name: String,
	pub class: String,
	pub doc_type: String,
	pub format: String,
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

	let file = ask(&subcommand_matches, prompt_theme)?;
	println!("{:?}", file);

	Ok(())
}

/// Ask the user information
fn ask(matches: &ArgMatches, prompt_theme: &dyn Theme) -> Result<File> {
	Ok(File {
		name: flag_or_ask(matches, prompt_theme, "name", "Name")?,
		class: flag_or_ask(matches, prompt_theme, "class", "Class")?,
		doc_type: flag_or_ask(matches, prompt_theme, "type", "Document Type")?,
		format: matches.value_of("format").unwrap().to_string(),
	})
}

/// Ask the user for a value if it isn't provided via a flag.
fn flag_or_ask(
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
	Ok(flag.unwrap().to_string())
}
