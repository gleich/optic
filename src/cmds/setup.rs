use std::{env, process};

use anyhow::{Context, Result};
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Confirm, Input, Select};

use crate::conf::{Class, Config};

pub fn run() -> Result<()> {
	let theme = ColorfulTheme::default();
	confirm(&theme)?;
	let toml = ask_config(&theme)?;
	println!("{}", toml);
	Ok(())
}

/// Confirm with the user that they want to create a kiwi project in the current working directory
fn confirm(theme: &ColorfulTheme) -> Result<()> {
	let cwd = env::current_dir().context("Failed to get current directory")?;
	if !Confirm::with_theme(theme)
		.with_prompt(format!("Create kiwi project in {:?}?", &cwd))
		.interact()
		.context("Failed to confirm setup with user")?
	{
		process::exit(0);
	}
	Ok(())
}

/// Ask the user a number of questions to generate a toml configuration file
fn ask_config(theme: &ColorfulTheme) -> Result<String> {
	println!(
		"\nYou're now going to input classes. Input \"Done\" as the class name once you've \
		 inputted all classes.\n"
	);
	let mut classes: Vec<Class> = Vec::new();
	loop {
		println!("  Class #{}", classes.len() + 1);
		let name: String = Input::with_theme(theme)
			.with_prompt("Name")
			.interact()
			.context("Failed to ask the name of the class")?;
		if name.to_lowercase() == "done" {
			break;
		}
		classes.push(Class {
			name,
			teacher: Input::with_theme(theme)
				.with_prompt("Teacher Name")
				.interact()
				.context("Failed to ask the name of the teacher")?,
		});
		println!("");
	}

	println!("\nYou're now going to input some general information.\n");
	let school_levels = ["Freshman", "Sophomore", "Junior", "Senior"];
	let school_types = ["High School", "College"];
	let config = Config {
		name: Input::with_theme(theme)
			.with_prompt("First and last name?")
			.interact()
			.context("Failed to ask for first and last name")?,
		school_level: school_levels
			.get(
				Select::with_theme(theme)
					.with_prompt("School level")
					.default(0)
					.items(&school_levels)
					.interact()
					.context("Failed to ask for school level")?,
			)
			.unwrap()
			.to_string(),
		school_type: school_types
			.get(
				Select::with_theme(theme)
					.with_prompt("School type")
					.default(0)
					.items(&school_types)
					.interact()
					.context("Failed to ask for school type")?,
			)
			.unwrap()
			.to_string(),
		classes,
	};
	Ok(toml::to_string(&config)?)
}
