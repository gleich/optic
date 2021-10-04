use std::path::{Path, PathBuf};

use anyhow::Result;
use walkdir::WalkDir;

pub struct Branch {
	pub path: PathBuf,
	pub pdf_path: Option<PathBuf>,
	pub imgs_dir: Option<PathBuf>,
}

pub fn get_all() -> Result<Vec<Branch>> {
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
						.unwrap(),
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
			})
		}
	}
	Ok(branches)
}
