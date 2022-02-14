use std::time::UNIX_EPOCH;

use anyhow::Result;
use chrono::Duration;
use chrono_humanize::{Accuracy, HumanTime, Tense};
use dialoguer::theme::ColorfulTheme;
use dialoguer::Confirm;

use crate::branch::Branch;
use crate::conf::Config;
use crate::out::task;

pub fn run() {
	let config = Config::read().expect("Failed to read from configuration file");
	let branches = Branch::get_all(&config).expect("Failed to get all branches");
	let (missing_pdfs, old_pdfs) =
		needs_building(&branches).expect("Failed to get branches that need building");
	let (build_missing, build_old) =
		ask(&missing_pdfs, &old_pdfs).expect("Failed to ask user about old and missing pdfs");

	if build_missing || build_old {
		println!();
	}
	if build_missing {
		build_all(&config, missing_pdfs);
	} else {
		println!("0 branches with missing PDF files");
	}
	if build_old {
		build_all(&config, old_pdfs);
	} else {
		println!("0 branches with old PDF files");
	}
}

pub fn needs_building(branches: &Vec<Branch>) -> Result<(Vec<&Branch>, Vec<&Branch>)> {
	let mut missing_pdf = Vec::new();
	let mut old_pdfs = Vec::new();
	for branch in branches {
		if branch.pdf_path.exists() {
			if branch.pdf_path.metadata()?.modified()? < branch.mod_time {
				old_pdfs.push(branch);
			}
		} else {
			missing_pdf.push(branch);
		}
	}

	Ok((missing_pdf, old_pdfs))
}

pub fn ask(missing_pdfs: &Vec<&Branch>, old_pdfs: &Vec<&Branch>) -> Result<(bool, bool)> {
	let mut build_missing = false;
	let mut build_old = false;
	let theme = ColorfulTheme::default();

	if !missing_pdfs.is_empty() {
		println!(
			"The following {}:\n",
			if missing_pdfs.len() == 1 {
				"branch is missing a PDF"
			} else {
				"branches are missing PDFs"
			}
		);

		for branch in missing_pdfs {
			println!("\t{} ({})", branch.name, branch.path.display());
		}
		println!();

		build_missing = Confirm::with_theme(&theme)
			.with_prompt("Do you want to build them?")
			.interact()?;
	}

	if !old_pdfs.is_empty() {
		println!(
			"\nThe following {}:\n",
			if old_pdfs.len() == 1 {
				"branch has an old a PDF"
			} else {
				"branches have old PDFs"
			}
		);

		for branch in old_pdfs {
			println!(
				"\t{} ({})\n\t\tΔ age: {}",
				branch.name,
				branch.path.display(),
				HumanTime::from(Duration::seconds(
					(branch
						.pdf_path
						.metadata()?
						.modified()?
						.duration_since(UNIX_EPOCH)
						.unwrap()
						.as_secs() - branch
						.mod_time
						.duration_since(UNIX_EPOCH)
						.unwrap()
						.as_secs()) as i64
				))
				.to_text_en(Accuracy::Precise, Tense::Present)
			)
		}
		println!();

		build_old = Confirm::with_theme(&theme)
			.with_prompt("Do you want to build them?")
			.interact()?;
	}

	Ok((build_missing, build_old))
}

pub fn build_all(config: &Config, branches: Vec<&Branch>) {
	for branch in branches {
		task(&format!("Building {}", branch.path.display()), || {
			branch.build(&config).expect("Failed to build branch");
		})
	}
}
