use std::path::{Path, PathBuf};
use std::str::FromStr;

use anyhow::{Context, Result};
use walkdir::WalkDir;

use crate::conf::DocType;

#[derive(Debug)]
pub struct Branch {
	pub path: PathBuf,
	pub pdf_path: Option<PathBuf>,
	pub imgs_dir: Option<PathBuf>,
	pub name: String,
	pub doc_type: DocType,
}

pub fn get() -> Result<Vec<Branch>> {
	let mut branches: Vec<Branch> = Vec::new();
	for entry in WalkDir::new("docs") {
		let entry = entry?;
		let mut extension = entry
			.path()
			.extension()
			.unwrap_or_default()
			.to_str()
			.unwrap()
			.to_string();
		if entry.file_type().is_file() && extension == String::from("tex")
			|| extension == String::from("md")
		{
			extension = format!(".{}", extension);
			let path = entry.path();
			let mut path_comps = path.components();
			let pdf_path = PathBuf::from(format!(
				"{}.pdf",
				Path::new("pdfs")
					.join(path.strip_prefix("docs").unwrap())
					.to_str()
					.unwrap()
					.strip_suffix(extension.as_str())
					.unwrap()
			));
			let img_path = PathBuf::from("imgs")
				.join(path_comps.nth(1).unwrap())
				.join(path_comps.nth(0).unwrap())
				.join(
					path_comps
						.nth(1)
						.unwrap()
						.as_os_str()
						.to_str()
						.unwrap()
						.strip_suffix(extension.as_str())
						.unwrap_or_default(),
				);
			branches.push(Branch {
				path: path.to_path_buf(),
				pdf_path: if pdf_path.exists() {
					Some(pdf_path)
				} else {
					None
				},
				imgs_dir: if img_path.exists() && img_path.is_dir() {
					Some(img_path)
				} else {
					None
				},
				name: path
					.file_name()
					.unwrap()
					.to_str()
					.unwrap()
					.trim_end_matches(".tex")
					.trim_end_matches(".md")
					.to_string(),
				doc_type: DocType::from_str(
					path.parent()
						.unwrap()
						.file_name()
						.unwrap()
						.to_str()
						.unwrap(),
				)
				.context(format!(
					"Failed to fetch document type for {}",
					path.to_str().unwrap()
				))?,
			})
		}
	}
	Ok(branches)
}