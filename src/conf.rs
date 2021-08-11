use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

pub const FNAME: &'static str = "kiwi.toml";

fn default_format() -> String { String::from("markdown") }

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct Config {
	pub name: String,
	pub school_level: String,
	pub school_type: String,
	#[serde(default = "default_format")]
	#[serde(skip_serializing)]
	pub default_format: String,
	pub classes: Vec<Class>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Class {
	pub name: String,
	pub teacher: String,
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
