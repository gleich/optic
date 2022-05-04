use anyhow::Result;
use copypasta::{ClipboardContext, ClipboardProvider};
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
		Action::Path => {
			println!("{}", branch.path.display());
			ClipboardContext::new()
				.expect("Failed to setup clipboard context")
				.set_contents(branch.path.to_str().unwrap().to_string())
				.expect("Failed to set clipboard context");
			println!("Copied to clipboard");
		}
	};
}

#[derive(Display, FromRepr, EnumVariantNames, Debug)]
enum Action {
	Build,
	Open,
	Reveal,
	Path,
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
