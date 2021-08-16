use anyhow::{Context, Result};
use clap::ArgMatches;
use dialoguer::theme::Theme;
use dialoguer::{Input, Select};

pub fn flag_or_ask_input(
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

pub fn flag_or_ask_select(
	matches: &ArgMatches,
	prompt_theme: &dyn Theme,
	flag_name: &str,
	prompt: &str,
	options: Vec<String>,
) -> Result<String> {
	let flag = matches.value_of(flag_name);
	if flag.is_none() {
		return Ok(options
			.get(
				Select::with_theme(prompt_theme)
					.with_prompt(prompt)
					.items(&options)
					.interact()
					.context(format!("Failed to ask for {}", prompt))?,
			)
			.unwrap()
			.to_owned());
	}
	Ok(flag.unwrap().to_string())
}
