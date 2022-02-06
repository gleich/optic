use std::fmt::Display;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Result};

use crate::conf::Format;
use crate::locations::folders;

#[derive(Debug, PartialEq)]
pub struct BranchTemplate {
	pub path: PathBuf,
	pub name: String,
	pub format: Format,
}

#[derive(Debug, PartialEq)]
pub struct RootTemplate {
	pub path: PathBuf,
	pub name: String,
}

impl BranchTemplate {
	pub fn new(path: PathBuf) -> Result<Self> {
		let latex_extension = ".tex.hbs";
		let markdown_extension = ".md.hbs";
		let filename = path.file_name().unwrap().to_str().unwrap();
		let (format, extension) = if filename.ends_with(latex_extension) {
			(Format::LaTeX, latex_extension)
		} else if filename.ends_with(markdown_extension) {
			(Format::Markdown, markdown_extension)
		} else {
			bail!("Improper file format for {}", path.display())
		};
		Ok(Self {
			name: filename.strip_suffix(extension).unwrap().to_string(),
			format,
			path,
		})
	}

	pub fn get_all() -> Result<Vec<Self>> {
		let files: Vec<PathBuf> =
			fs::read_dir(Path::new(folders::TEMPLATES).join(folders::BRANCH_TEMPLATES))?
				.into_iter()
				.filter(|r| r.is_ok())
				.map(|r| r.unwrap().path())
				.filter(|r| r.is_file())
				.collect();
		let mut templates = Vec::new();
		for file in files {
			templates.push(Self::new(file)?);
		}
		Ok(templates)
	}
}

impl RootTemplate {
	pub fn new(path: PathBuf) -> Self {
		Self {
			name: path
				.file_name()
				.unwrap()
				.to_str()
				.unwrap()
				.strip_suffix(".hbs")
				.unwrap()
				.to_string(),
			path,
		}
	}

	pub fn get_all() -> Result<Vec<Self>> {
		let files: Vec<PathBuf> =
			fs::read_dir(Path::new(folders::TEMPLATES).join(folders::ROOT_TEMPLATES))?
				.into_iter()
				.filter(|r| r.is_ok())
				.map(|r| r.unwrap().path())
				.filter(|r| r.is_file())
				.collect();
		let mut templates = Vec::new();
		for file in files {
			templates.push(Self::new(file));
		}
		Ok(templates)
	}
}

impl Display for BranchTemplate {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.name)
	}
}

impl Display for RootTemplate {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.name)
	}
}
