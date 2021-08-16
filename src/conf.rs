use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString, ToString};

pub const FNAME: &'static str = "kiwi.toml";

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct Config {
	pub name: String,
	pub school_level: String,
	pub school_type: String,
	#[serde(skip_serializing)]
	#[serde(default)]
	pub default_format: Format,
	pub classes: Vec<Class>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Class {
	pub name: String,
	pub teacher: String,
}

#[derive(ToString, EnumIter, EnumString, PartialEq)]
pub enum DocType {
	Worksheet,
	Note,
	Assessment,
	Paper,
	Lab,
	Other,
}

#[derive(ToString, EnumIter, Debug, Serialize, Deserialize, EnumString, PartialEq)]
pub enum Format {
	LaTeX,
	Markdown,
}

impl DocType {
	pub fn to_vec<'a>() -> Vec<String> { DocType::iter().map(|t| t.to_string()).collect() }
}

impl Format {
	pub fn to_vec<'a>() -> Vec<String> { Format::iter().map(|t| t.to_string()).collect() }
}

impl Default for Format {
	fn default() -> Self { Format::Markdown }
}

/// Read from the config file.
/// Default parameter is to have the function return a blank config struct if the file doesn't exist.
pub fn read(default: bool) -> Result<Config> {
	let loc = Path::new(FNAME);
	if !loc.exists() && default {
		return Ok(Default::default());
	}
	let contents = fs::read_to_string(loc).context("Failed to read from configuration file")?;
	Ok(toml::from_str(&contents)?)
}
