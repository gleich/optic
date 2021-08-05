use std::{env, process};

use anyhow::{Context, Result};
use dialoguer::Confirm;

pub fn run() -> Result<()> {
	confirm()?;
	Ok(())
}

fn confirm() -> Result<()> {
	let cwd = env::current_dir().context("Failed to get current directory")?;
	if !Confirm::new()
		.with_prompt(format!("Create kiwi project in {:?}?", &cwd))
		.interact()
		.context("Failed to confirm setup with user")?
	{
		process::exit(0);
	}
	Ok(())
}
