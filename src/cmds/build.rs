use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::{env, fs};

use anyhow::{bail, Context, Result};
use chrono::{Datelike, Month, NaiveDateTime};
use clap::ArgMatches;
use num_traits::FromPrimitive;
use walkdir::WalkDir;

use crate::conf::{self, Format, TemplateType};
use crate::inject::inject;
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
	let config = conf::read(false).expect("Failed to read from the configuration file");
	let branch_path =
		branch_to_build(subcommand_matches).expect("Failed to get what file should be built");
	let branch_contents =
		fs::read_to_string(&branch_path).expect("Failed to read from branch file");
	let branch_data = extract_branch_data(&branch_contents, &branch_path)
		.expect("Failed to extract data from branch file");
	let latex = inject(
		branch_data
			.path
			.file_name()
			.unwrap()
			.to_str()
			.unwrap()
			.to_string(),
		branch_data.root.file_name().unwrap().to_str().unwrap(),
		&branch_data.class_name,
		&Format::LaTeX,
		&config,
		fs::read_to_string(&branch_data.root).expect("Failed to read from root file"),
		Some(match branch_data.format {
			Format::LaTeX => branch_contents,
			Format::Markdown => {
				convert_to_latex(&branch_path).expect("Failed to convert branch file to latex")
			}
		}),
	)
	.expect("Failed to inject variables into root file");
	generate_pdf(
		&latex,
		&branch_data.name,
		&branch_data.class_name,
		&branch_data.created,
		branch_path.parent().unwrap().file_name().unwrap(),
	)
	.expect("Failed to generate PDF file");
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
	let branch_extension = branch_path.extension().unwrap().to_str().unwrap();
	Ok(Branch {
		name: branch_path
			.file_name()
			.unwrap()
			.to_str()
			.unwrap()
			.trim_end_matches(branch_extension)
			.to_string(),
		format: match branch_extension {
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

fn convert_to_latex(branch_path: &PathBuf) -> Result<String> {
	let output = Command::new("pandoc")
		.arg("-r")
		.arg("markdown-auto_identifiers")
		.arg("-w")
		.arg("latex")
		.arg("--pdf-engine")
		.arg("pdflatex")
		.arg(branch_path.to_str().unwrap())
		.stdout(Stdio::piped())
		.output()?;
	Ok(String::from_utf8(output.stdout)?)
}

fn generate_pdf(
	latex: &str,
	doc_name: &str,
	class_name: &str,
	created_time: &NaiveDateTime,
	doc_type_name: &OsStr,
) -> Result<()> {
	let build_dir = Path::new("assembler");
	if build_dir.exists() {
		fs::remove_dir_all(build_dir)?;
	}
	fs::create_dir(build_dir)?;
	env::set_current_dir(build_dir)?;

	// Write to LaTeX file and building PDF
	let latex_fname = "build.tex";
	fs::write(latex_fname, latex)?;
	let output = Command::new("pdflatex")
		.arg(latex_fname)
		.stdout(Stdio::piped())
		.output()?;
	if !output.status.success() {
		let failed_log_fname = "failure.log";
		fs::write(failed_log_fname, output.stdout)?;
		bail!(
			"Failed to generate PDF. Please check {} in {}",
			failed_log_fname,
			build_dir.to_str().unwrap()
		);
	}
	env::set_current_dir("..")?;

	// Moving generated PDF to it's home
	let pdf_fname = format!("{}pdf", doc_name);
	let pdf_dir = Path::new("pdfs")
		.join(class_name)
		.join(Month::from_u32(created_time.month()).unwrap().name())
		.join(doc_type_name);
	fs::create_dir_all(&pdf_dir)?;
	fs::rename(
		Path::new(build_dir).join("build.pdf"),
		pdf_dir.join(&pdf_fname),
	)
	.context("Failed to move generated PDF from build directory to pdfs folder")?;

	fs::remove_dir_all(build_dir)?;
	Ok(())
}
