use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use chrono::NaiveDateTime;
use clap::ArgMatches;
use walkdir::WalkDir;

use crate::conf::{Format, TemplateType};
use crate::out::ARROW_CHARACTERS;

#[derive(Debug)]
struct Branch {
	pub format: Format,
	pub name: String,
	pub path: PathBuf,
	pub created: NaiveDateTime,
	pub root: PathBuf,
	pub class_name: String,
}

pub fn run(matches: &ArgMatches) {
	let subcommand_matches = matches.subcommand_matches("build").unwrap();
	let branch_path =
		branch_to_build(subcommand_matches).expect("Failed to get what file should be built");
	let branch_contents =
		fs::read_to_string(&branch_path).expect("Failed to read from branch file");
	let branch_data = extract_branch_data(&branch_contents, &branch_path)
		.expect("Failed to extract data from branch file");
	println!("{:?}", branch_data);
}

/// Get the file that should be built
fn branch_to_build(matches: &ArgMatches) -> Result<PathBuf> {
	// Return path provided via args if it is provided
	if matches.value_of("path").is_some() {
		return Ok(PathBuf::from(matches.value_of("path").unwrap()));
	}

	// Find and return file that was most recently updated
	let mut min_time = None;
	let mut file = None;
	for entry in WalkDir::new("docs") {
		let entry = entry?;
		let extension = entry.path().extension().unwrap_or_default();
		if entry.file_type().is_file() && extension == OsStr::new("tex")
			|| extension == OsStr::new("md")
		{
			let modtime = entry.metadata()?.modified()?.elapsed()?.as_secs();
			if min_time.is_none() || file.is_none() || min_time.unwrap() > modtime {
				min_time = Some(modtime);
				file = Some(entry);
			}
		}
	}
	Ok(file.unwrap().path().to_path_buf())
}

fn extract_branch_data(content: &str, branch_path: &PathBuf) -> Result<Branch> {
	/// Extract variable value. Example:
	/// "2021-08-18" from "create ―→ 2021-08-18"
	fn extract_variable(name: &str, lines: &Vec<&str>) -> Option<String> {
		for line in lines {
			let trimmed_line = line.trim();
			let prefix = format!("{} {} ", name, ARROW_CHARACTERS);
			if trimmed_line.starts_with(&prefix) {
				return Some(trimmed_line.trim_start_matches(&prefix).to_string());
			}
		}
		None
	}

	let lines: Vec<&str> = content.split("\n").collect();
	Ok(Branch {
		name: branch_path
			.to_str()
			.unwrap()
			.trim_end_matches(branch_path.extension().unwrap().to_str().unwrap())
			.to_string(),
		format: match branch_path.extension().unwrap().to_str().unwrap() {
			"md" => Format::Markdown,
			_ => Format::LaTeX,
		},
		path: branch_path.clone(),
		created: NaiveDateTime::parse_from_str(
			&format!(
				"{} {}",
				extract_variable("created", &lines)
					.expect("Failed to extract \"created\" field from preamble"),
				"0:0:0"
			),
			"%F %H:%M:%S", // We must include an time of the day so we add 0:0:0 here manually
		)?,
		root: Path::new("templates")
			.join(TemplateType::Root.to_string())
			.join(
				extract_variable("root", &lines)
					.expect("Failed to extract \"root\" field from preamble"),
			),
		class_name: extract_variable("class", &lines)
			.expect("Failed to extract \"class\" field from preamble"),
	})
}
