use std::ffi::OsStr;
use std::path::PathBuf;

use anyhow::Result;
use clap::ArgMatches;
use walkdir::WalkDir;

pub fn run(matches: &ArgMatches) {
	let subcommand_matches = matches.subcommand_matches("build").unwrap();
	let branch_path =
		branch_to_build(subcommand_matches).expect("Failed to get what file should be built");
	println!("{:?}", branch_path);
}

/// Get the file that should be built
pub fn branch_to_build(matches: &ArgMatches) -> Result<PathBuf> {
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
