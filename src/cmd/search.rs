use anyhow::Result;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{FuzzySelect, Select};
use strum::VariantNames;
use strum_macros::{Display, EnumVariantNames, FromRepr};
use task_log::task;

use crate::branch::Branch;
use crate::conf::Config;

pub fn run() {
	let config = Config::read().expect("Failed to read from configuration file");
	let branches = Branch::get_all(&config).expect("Failed to get branches");
	let (branch, action) = ask(&branches).expect("Failed to ask user for branch");
	println!();
	match action {
		Action::Build => {
			task(format!("Building {}", branch.name), || {
				branch
					.build(&config, &config.latexmk)
					.expect("Failed to build PDF")
			})
		}
		Action::Open => {
			task(format!("Opening {}", branch.name), || {
				branch.open(&config).expect("Failed to open branch")
			})
		}
		Action::Reveal => {
			task(format!("Revealing {}", branch.name), || {
				branch.reveal(&config, true).expect("Failed to reveal PDF")
			})
		}
	};
}

#[derive(Display, FromRepr, EnumVariantNames, Debug)]
enum Action {
	Build,
	Open,
	Reveal,
}

fn ask(branches: &[Branch]) -> Result<(&Branch, Action)> {
	let theme = ColorfulTheme::default();
	let branch = branches
		.get(
			FuzzySelect::with_theme(&theme)
				.with_prompt("Branch")
				.items(branches)
				.interact()?,
		)
		.unwrap();

	let action = Action::from_repr(
		Select::with_theme(&theme)
			.with_prompt("Action")
			.items(Action::VARIANTS)
			.default(0)
			.interact()?,
	)
	.unwrap();

	Ok((branch, action))
}
