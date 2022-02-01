use std::fs;

use anyhow::Result;
use serde::Deserialize;

use crate::format::Format;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Config {
	pub name: String,
	#[serde(default = "defaults::config_delimiter")]
	pub delimiter: String,
	pub open_with: Vec<String>,
	#[serde(default = "defaults::config_default_format")]
	pub default_format: Format,
	pub classes: Vec<Class>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Class {
	pub name: String,
	pub teacher: String,
	#[serde(default = "defaults::class_active")]
	pub active: bool,
}

mod defaults {
	use crate::format::Format;

	pub fn config_delimiter() -> String { String::from(">") }
	pub fn config_default_format() -> Format { Format::Markdown }

	pub fn class_active() -> bool { false }
}

impl Config {
	pub fn read() -> Result<Config> {
		let content = fs::read_to_string("./kiwi.toml")?;
		Ok(toml::from_str::<Config>(&content)?)
	}
}

#[cfg(test)]
mod test {
	use toml::de::Error;

	use crate::conf::{Class, Config};
	use crate::format::Format;

	#[test]
	fn read_config() -> Result<(), Error> {
		// Super basic config file
		assert_eq!(
			toml::from_str::<Config>(
				"
        name = \"Matt Gleich\"
        open_with = [\"code\"]

        [[classes]]
        name = \"AP Physics 2\"
        teacher = \"Mr. Feynman\"
        active = true
    "
			)?,
			Config {
				name: String::from("Matt Gleich"),
				delimiter: String::from(">"),
				open_with: vec![String::from("code")],
				default_format: Format::Markdown,
				classes: vec![Class {
					name: String::from("AP Physics 2"),
					teacher: String::from("Mr. Feynman"),
					active: true,
				}],
			}
		);
		// Custom default_format
		assert_eq!(
			toml::from_str::<Config>(
				"
        name = \"Matt Gleich\"
        open_with = [\"code\"]
        default_format = \"LaTeX\"

        [[classes]]
        name = \"AP Physics 2\"
        teacher = \"Mr. Feynman\"
        active = true
    "
			)?,
			Config {
				name: String::from("Matt Gleich"),
				delimiter: String::from(">"),
				open_with: vec![String::from("code")],
				default_format: Format::LaTeX,
				classes: vec![Class {
					name: String::from("AP Physics 2"),
					teacher: String::from("Mr. Feynman"),
					active: true,
				}],
			}
		);
		// Multiple classes
		assert_eq!(
			toml::from_str::<Config>(
				"
        name = \"Matt Gleich\"
        open_with = [\"code\"]
        default_format = \"LaTeX\"

        [[classes]]
        name = \"AP Physics 2\"
        teacher = \"Mr. Feynman\"
        active = true

        [[classes]]
        name = \"AP Chemistry 2\"
        teacher = \"Mr. White\"
        active = true
    "
			)?,
			Config {
				name: String::from("Matt Gleich"),
				delimiter: String::from(">"),
				open_with: vec![String::from("code")],
				default_format: Format::LaTeX,
				classes: vec![
					Class {
						name: String::from("AP Physics 2"),
						teacher: String::from("Mr. Feynman"),
						active: true,
					},
					Class {
						name: String::from("AP Chemistry 2"),
						teacher: String::from("Mr. White"),
						active: true,
					}
				],
			}
		);
		Ok(())
	}
}
