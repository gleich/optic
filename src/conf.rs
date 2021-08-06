use serde::{Deserialize, Serialize};

pub const FNAME: &'static str = "kiwi.toml";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
	pub name: String,
	pub school_level: String,
	pub school_type: String,
	pub classes: Vec<Class>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Class {
	pub name: String,
	pub teacher: String,
}
