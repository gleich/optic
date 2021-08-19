use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use anyhow::{bail, Result};
use chrono::{Datelike, Local, Month};
use clap::ArgMatches;
use dialoguer::theme::Theme;
use num_traits::FromPrimitive;
use strum::VariantNames;

use crate::conf::{Config, DocType, Format};
use crate::out::success;
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
		let format = conf::Format::from_str(&cli::flag_or_select(
			&self.matches,
			prompt_theme,
			"format",
			"Format",
			Format::VARIANTS.to_vec(),
		)?)?;

		self.branch = Some(Branch {
			name: cli::flag_or_input(&self.matches, prompt_theme, "name", "Name")?,
			class: cli::flag_or_select(
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
			doc_type: conf::DocType::from_str(&cli::flag_or_select(
				&self.matches,
				prompt_theme,
				"type",
				"Type",
				DocType::VARIANTS.to_vec(),
			)?)
			.unwrap(),
			branch_template_path: Path::new(&cli::flag_or_select(
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
			root_template_path: Path::new(&cli::flag_or_select(
				&self.matches,
				prompt_theme,
				"root",
				"Root template",
				conf::list_templates(&Format::LaTeX, &conf::TemplateType::Root)?
					.iter()
					.map(AsRef::as_ref)
					.collect(),
			)?)
			.to_path_buf(),
			format,
		});
		println!();
		success("Obtained information about file");
		Ok(())
	}

	pub fn create(&self) -> Result<()> {
		let branch = self.branch.as_ref().unwrap();
		let content = inject::inject(
			branch.doc_type.to_string(),
			branch.root_template_path.to_str().unwrap(),
			branch.class.to_string(),
			&branch.format,
			&self.config,
			fs::read_to_string(
				Path::new("templates")
					.join(conf::TemplateType::Branch.to_string())
					.join(&branch.branch_template_path),
			)?,
		)?;
		success("Injected variables into template");
		let path = Path::new("docs")
			.join(&branch.class)
			.join(Month::from_u32(Local::now().month()).unwrap().name())
			.join(&branch.doc_type.to_string())
			.join(format!(
				"{}.{}",
				&branch.name,
				match branch.format {
					Format::LaTeX => "tex",
					Format::Markdown => "md",
				}
			));
		fs::create_dir_all(path.parent().unwrap())?;
		if path.exists() {
			bail!("{} already exists", &path.to_str().unwrap());
		}
		fs::write(&path, content)?;
		success(&format!("Created {}", &path.to_str().unwrap()));
		Ok(())
	}
}
