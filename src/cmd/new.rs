use std::str::FromStr;

use anyhow::Result;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{FuzzySelect, Input, Select};
use strum::VariantNames;

use crate::branch::Branch;
use crate::conf::{Config, DocumentType, Format};
use crate::template::{BranchTemplate, RootTemplate};

pub fn run() {
	let branch = ask().expect("Failed to ask user about branch");
	println!("{:?}", branch);
}

fn ask() -> Result<Branch> {
	let theme = ColorfulTheme::default();
	let mut config = Config::read()?;
	let mut branch_templates = BranchTemplate::get_all()?;
	let mut root_templates = RootTemplate::get_all()?;

	let name: String = Input::with_theme(&theme)
		.with_prompt("Name")
		.interact_text()?;

	let format = Format::from_str(
		Format::VARIANTS
			.get(
				Select::with_theme(&theme)
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

	let class = config.classes.swap_remove(
		FuzzySelect::with_theme(&theme)
			.with_prompt("Class")
			.items(config.classes.as_slice())
			.interact()?,
	);

	branch_templates = branch_templates
		.into_iter()
		.filter(|b| b.format == format)
		.collect();
	let branch_template = branch_templates.swap_remove(
		FuzzySelect::with_theme(&theme)
			.with_prompt("Branch Template")
			.items(branch_templates.as_slice())
			.interact()?,
	);

	let root_template = root_templates.swap_remove(
		FuzzySelect::with_theme(&theme)
			.with_prompt("Root Template")
			.items(root_templates.as_slice())
			.interact()?,
	);

	Branch::new(
		name,
		format,
		doc_type,
		class,
		branch_template,
		root_template,
	)
}
