use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use anyhow::Result;
use clap::ArgMatches;
use dialoguer::theme::Theme;
use strum::VariantNames;

use crate::conf::{Config, DocType, Format};
use crate::{cli, conf, inject};

pub fn run(matches: &ArgMatches, prompt_theme: &dyn Theme) {
	let mut steps = Steps {
		branch: None,
		config: conf::read(false).expect("Failed to read from configuration file"),
		matches: matches.subcommand_matches("new").unwrap(),
	};
	steps
		.ask(prompt_theme)
		.expect("Failed to ask user for info about the branch");
	steps.create().expect("Failed to create the file");
}

struct Branch {
	name: String,
	class: String,
	doc_type: DocType,
	format: Format,
	branch_template_path: PathBuf,
	root_template_path: PathBuf,
}

struct Steps<'a> {
	branch: Option<Branch>,
	config: Config,
	matches: &'a ArgMatches,
}

impl Steps<'_> {
	/// Ask the user information
	pub fn ask(&mut self, prompt_theme: &dyn Theme) -> Result<()> {
		let format = conf::Format::from_str(&cli::flag_or_ask_select(
			&self.matches,
			prompt_theme,
			"format",
			"Format",
			Format::VARIANTS.to_vec(),
		)?)?;

		self.branch = Some(Branch {
			name: cli::flag_or_ask_input(&self.matches, prompt_theme, "name", "Name")?,
			class: cli::flag_or_ask_select(
				&self.matches,
				prompt_theme,
				"class",
				"Class",
				self.config
					.classes
					.iter()
					.map(|c| c.name.as_str())
					.collect(),
			)?,
			doc_type: conf::DocType::from_str(&cli::flag_or_ask_select(
				&self.matches,
				prompt_theme,
				"type",
				"Type",
				DocType::VARIANTS.to_vec(),
			)?)
			.unwrap(),
			branch_template_path: Path::new(&cli::flag_or_ask_select(
				&self.matches,
				prompt_theme,
				"branch",
				"Branch template",
				conf::list_templates(&format, &conf::TemplateType::Branch)?
					.iter()
					.map(AsRef::as_ref)
					.collect(),
			)?)
			.to_path_buf(),
			root_template_path: Path::new(&cli::flag_or_ask_select(
				&self.matches,
				prompt_theme,
				"root",
				"Root template",
				conf::list_templates(&format, &conf::TemplateType::Root)?
					.iter()
					.map(AsRef::as_ref)
					.collect(),
			)?)
			.to_path_buf(),
			format,
		});
		Ok(())
	}

	pub fn create(&self) -> Result<()> {
		// Create file contents
		let branch = self.branch.as_ref().unwrap();
		let path = Path::new("templates")
			.join(conf::TemplateType::Branch.to_string())
			.join(&branch.branch_template_path);
		let contents = inject::BaseData {
			doc_name: &branch.name,
			class_name: &branch.class,
			format: &branch.format,
			config: &self.config,
		}
		.inject_into(&fs::read_to_string(path)?)?;
		println!("{}", contents);
		Ok(())
	}
}
