use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Result;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{FuzzySelect, Input};
use strum::VariantNames;

use crate::branch::Branch;
use crate::conf::{Config, DocumentType, Format};
use crate::template::BranchTemplate;

pub fn run() {
	let branches = BranchTemplate::get_all().expect("Failed to get all branches");
	println!("{:?}", branches);
	let config = Config::read().expect("Failed to read from config file");
	let branch = ask(&config).expect("Failed to ask user about branch");
	println!("{:?}", branch);
}

fn ask(config: &Config) -> Result<Branch> {
	let theme = ColorfulTheme::default();
	let name: String = Input::with_theme(&theme)
		.with_prompt("Name")
		.interact_text()?;
	let format = Format::from_str(
		Format::VARIANTS
			.get(
				FuzzySelect::with_theme(&theme)
					.with_prompt("Format")
					.items(Format::VARIANTS)
					.default(0)
					.interact()?,
			)
			.unwrap(),
	)?;
	let doc_type = DocumentType::from_str(
		DocumentType::VARIANTS
			.get(
				FuzzySelect::with_theme(&theme)
					.with_prompt("Type")
					.items(DocumentType::VARIANTS)
					.default(0)
					.interact()?,
			)
			.unwrap(),
	)?;

	let class = config
		.classes
		.get(
			FuzzySelect::with_theme(&theme)
				.with_prompt("Class")
				.items(config.classes.as_slice())
				.interact()?,
		)
		.unwrap();

	Branch::new(name, format, doc_type, class.clone())
}
