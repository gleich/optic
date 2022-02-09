use std::fs;
use std::str::FromStr;
use std::time::SystemTime;

use anyhow::Result;
use chrono::Local;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{FuzzySelect, Input, Select};
use strum::VariantNames;

use crate::branch::Branch;
use crate::conf::{Class, Config, DocumentType, Format};
use crate::out::task;
use crate::template::{BranchTemplate, RootTemplate};

pub fn run() {
	let mut config = Config::read().expect("Failed to read from config file");
	let branch = ask(&mut config).expect("Failed to ask user about branch");
	let formatted_branch = branch
		.inject(
			&config,
			fs::read_to_string(branch.branch_template.clone().unwrap().path)
				.expect("Failed to read from branch file"),
			None,
		)
		.expect("Failed to inject variables into branch");

	task("Creating branch", || {
		fs::create_dir_all(branch.path.parent().unwrap())
			.expect("Failed to create parent folder for new branch file");
		fs::create_dir_all(branch.imgs_dir).expect("Failed to create images directory for branch");
		fs::write(&branch.path, formatted_branch).expect("Failed to format branch");
	})
}

fn ask(config: &mut Config) -> Result<Branch> {
	let theme = ColorfulTheme::default();
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
			.items(
				config
					.classes
					.iter()
					.filter(|c| c.active)
					.collect::<Vec<&Class>>()
					.as_slice(),
			)
			.default(0)
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
			.default(0)
			.interact()?,
	);

	let root_template = root_templates.swap_remove(
		FuzzySelect::with_theme(&theme)
			.with_prompt("Root Template")
			.items(root_templates.as_slice())
			.default(0)
			.interact()?,
	);

	Branch::new(
		name,
		format,
		doc_type,
		class,
		Some(branch_template),
		root_template,
		Local::now().date(),
		SystemTime::now(),
	)
}
