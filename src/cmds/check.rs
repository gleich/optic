use crate::branches::{self, Branch};
use crate::conf;

use anyhow::{Context, Result};
use dialoguer::theme::Theme;
use dialoguer::Confirm;

use super::build::build;

pub fn run(prompt_theme: &dyn Theme) {
	let config = conf::read(false).expect("Failed to read from the configuration file");
	let (missing_pdfs, unsynced_pdfs) = check_branches().expect("Failed to check branches");
	let branches_to_build = output_and_ask(missing_pdfs, unsynced_pdfs, prompt_theme)
		.expect("Failed to ask the use about the branches");
	for branch in branches_to_build {
		build(&config, &branch.path).expect("Failed to build branch file");
	}
}

fn check_branches() -> Result<(Vec<Branch>, Vec<Branch>)> {
	let mut missing_pdfs = Vec::new();
	let mut unsynced_branches = Vec::new();
	for branch in branches::get_all()? {
		if branch.pdf_path.is_none() {
			missing_pdfs.push(branch);
			continue;
		}
		let branch_modtime = &branch
			.path
			.metadata()
			.context("Failed to get metadata about branch")?
			.modified()
			.context("Failed to get modification information about branch")?
			.elapsed()
			.context("Failed to get elapsed time since modification for branch")?;
		let pdf_modtime = branch
			.pdf_path
			.as_ref()
			.unwrap()
			.metadata()
			.context("Failed to get metadata about pdf")?
			.modified()
			.context("Failed to get modification information about branch")?
			.elapsed()
			.context("Failed to get elapsed time since modification for pdf")?;
		if branch_modtime.as_millis() < pdf_modtime.as_millis() {
			unsynced_branches.push(branch);
		}
	}
	Ok((missing_pdfs, unsynced_branches))
}

fn output_and_ask(
	missing_pdfs: Vec<Branch>,
	unsynced_pdfs: Vec<Branch>,
	prompt_theme: &dyn Theme,
) -> Result<Vec<Branch>> {
	fn confirm_batch(
		batch: Vec<Branch>,
		problem: &str,
		prompt_theme: &dyn Theme,
	) -> Result<Vec<Branch>> {
		println!(
			"Found {} branches {}{}",
			batch.len(),
			problem,
			if !batch.is_empty() { ":" } else { "." }
		);
		if batch.is_empty() {
			return Ok(Vec::new());
		} else {
			for branch in &batch {
				println!("\t{}", branch.path.display());
			}
		}
		println!();
		if Confirm::with_theme(prompt_theme)
			.with_prompt(
				"Do you want to fix the issue by build/rebuilding the problematic branches?",
			)
			.interact()
			.context("Failed to confirm batch")?
		{
			Ok(batch)
		} else {
			Ok(Vec::new())
		}
	}

	let mut branches_to_build = Vec::new();
	branches_to_build.append(&mut confirm_batch(
		missing_pdfs,
		"missing PDF",
		prompt_theme,
	)?);
	branches_to_build.append(&mut confirm_batch(unsynced_pdfs, "unsynced", prompt_theme)?);

	Ok(branches_to_build)
}
