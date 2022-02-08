use std::path::PathBuf;
use std::{fmt, fs};

use anyhow::Result;
use serde::Deserialize;
use strum_macros::{Display, EnumString, EnumVariantNames};

use crate::locations;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Config {
	pub author: String,
	#[serde(default = "defaults::config_delimiter")]
	pub delimiter: String,
	pub open_with: Option<Vec<String>>,
	#[serde(default = "defaults::config_default_format")]
	pub default_format: Format,
	pub classes: Vec<Class>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Class {
	pub name: String,
	pub teacher: String,
	#[serde(default = "defaults::class_active")]
	pub active: bool,
}

#[derive(PartialEq, Debug, Display, Deserialize, EnumVariantNames, EnumString, Clone)]
pub enum Format {
	LaTeX,
	Markdown,
}

#[derive(PartialEq, Debug, Display, Deserialize, EnumVariantNames, EnumString)]
pub enum DocumentType {
	Worksheet,
	Note,
	Assessment,
	Paper,
	Lab,
	Other,
}

mod defaults {
	use super::Format;

	pub fn config_delimiter() -> String { String::from(">") }
	pub fn config_default_format() -> Format { Format::Markdown }

	pub fn class_active() -> bool { true }
}

impl Config {
	pub fn read() -> Result<Config> {
		let content = fs::read_to_string(locations::files::CONFIG)?;
		Ok(toml::from_str::<Config>(&content)?)
	}
}

impl Format {
	pub fn extension(&self) -> &'static str {
		match *self {
			Format::LaTeX => ".tex",
			Format::Markdown => ".md",
		}
	}

	pub fn from_path(path: &PathBuf) -> Option<Self> {
		match path.extension().unwrap_or_default().to_str().unwrap() {
			"tex" => Some(Format::LaTeX),
			"md" => Some(Format::Markdown),
			_ => None,
		}
	}
}

impl Default for Format {
	fn default() -> Self { Self::LaTeX }
}

impl fmt::Display for Class {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} ({})", self.name, self.teacher)
	}
}

#[cfg(test)]
mod test {
	use toml::de::Error;

	use super::Format;
	use crate::conf::{Class, Config};

	#[test]
	fn read_config() -> Result<(), Error> {
		// Super basic config file
		assert_eq!(
			toml::from_str::<Config>(
				"
        author = \"Matt Gleich\"

        [[classes]]
        name = \"AP Physics 2\"
        teacher = \"Mr. Feynman\"
    "
			)?,
			Config {
				author: String::from("Matt Gleich"),
				delimiter: String::from(">"),
				open_with: None,
				default_format: Format::Markdown,
				classes: vec![Class {
					name: String::from("AP Physics 2"),
					teacher: String::from("Mr. Feynman"),
					active: true
				}],
			}
		);
		// Custom default_format
		assert_eq!(
			toml::from_str::<Config>(
				"
        author = \"Matt Gleich\"
        open_with = [\"code\"]
        default_format = \"LaTeX\"

        [[classes]]
        name = \"AP Physics 2\"
        teacher = \"Mr. Feynman\"
    "
			)?,
			Config {
				author: String::from("Matt Gleich"),
				delimiter: String::from(">"),
				open_with: Some(vec![String::from("code")]),
				default_format: Format::LaTeX,
				classes: vec![Class {
					name: String::from("AP Physics 2"),
					teacher: String::from("Mr. Feynman"),
					active: true
				}],
			}
		);
		// Multiple classes
		assert_eq!(
			toml::from_str::<Config>(
				"
        author = \"Matt Gleich\"
        open_with = [\"code\"]
        default_format = \"LaTeX\"

        [[classes]]
        name = \"AP Physics 2\"
        teacher = \"Mr. Feynman\"

        [[classes]]
        name = \"AP Chemistry 2\"
        teacher = \"Mr. White\"
		active = false
    "
			)?,
			Config {
				author: String::from("Matt Gleich"),
				delimiter: String::from(">"),
				open_with: Some(vec![String::from("code")]),
				default_format: Format::LaTeX,
				classes: vec![
					Class {
						name: String::from("AP Physics 2"),
						teacher: String::from("Mr. Feynman"),
						active: true
					},
					Class {
						name: String::from("AP Chemistry 2"),
						teacher: String::from("Mr. White"),
						active: false
					}
				],
			}
		);
		Ok(())
	}
}
