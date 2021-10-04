use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, EnumVariantNames, ToString};

pub const FNAME: &str = "kiwi.toml";
pub const TEMPLATES_DIR: &str = "templates";

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct Config {
	pub name: String,
	pub school: School,
	#[serde(skip_serializing)]
	#[serde(default = "default_delimiter")]
	pub delimiter: String,
	pub open_with: Option<Open>,
	#[serde(skip_serializing)]
	#[serde(default)]
	pub default_format: Format,
	pub classes: Vec<Class>,
}

// Defaults
fn default_delimiter() -> String { String::from(">") }
fn default_active() -> bool { true }
impl Default for Format {
	fn default() -> Self { Format::Markdown }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Open {
	pub command: String,
	pub args: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Class {
	pub name: String,
	pub teacher: String,
	#[serde(default = "default_active", skip_serializing)]
	pub active: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct School {
	pub level: String,
	// type is a keyword in rust so it can't be the name of the variant
	#[serde(rename = "type")]
	pub type_name: String,
}

#[derive(EnumVariantNames, ToString, EnumString, PartialEq, Debug)]
pub enum DocType {
	Worksheet,
	Note,
	Assessment,
	Paper,
	Lab,
	Other,
}

#[derive(EnumVariantNames, ToString, Debug, Serialize, Deserialize, EnumString, PartialEq)]
pub enum Format {
	LaTeX,
	Markdown,
}

/// Types of templates that the user can write
#[derive(EnumVariantNames, ToString, PartialEq, Debug, EnumString)]
pub enum TemplateType {
	#[strum(serialize = "root")]
	Root,
	#[strum(serialize = "branch")]
	Branch,
}

/// Read from the config file.
/// Default parameter is to have the function return a blank config struct if the file doesn't exist.
pub fn read(default: bool) -> Result<Config> {
	let loc = Path::new(FNAME);
	if !loc.exists() && default {
		return Ok(Default::default());
	}
	let contents = fs::read_to_string(loc).context("Failed to read from configuration file")?;
	Ok(toml::from_str(&contents).context("Failed to parse toml")?)
}

/// Get a list of all the template files for a certain template type
pub fn list_templates(format: &Format, group: &TemplateType) -> Result<Vec<String>> {
	let template_fs_objects = fs::read_dir(Path::new(TEMPLATES_DIR).join(group.to_string()))
		.context("Failed to get templates")?;

	let mut file_names = Vec::new();
	for entry in template_fs_objects {
		let fs_object =
			entry.context("Failed to get file system object from read directory call")?;
		if fs_object
			.file_type()
			.context("Failed to get file type")?
			.is_file()
		{
			let file_name = fs_object.file_name().to_str().unwrap().to_string();
			if file_name.ends_with("tex.hbs") && format == &Format::LaTeX {
				file_names.push(file_name);
			} else if file_name.ends_with("md.hbs") && format == &Format::Markdown {
				file_names.push(file_name);
			}
		}
	}

	Ok(file_names)
}
