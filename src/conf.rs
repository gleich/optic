use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
	pub name: String,
	pub school_year: String,
	pub classes: Vec<Class>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Class {
	pub name: String,
	pub teacher: String,
}
