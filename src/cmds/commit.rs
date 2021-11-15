use std::ops::Index;
use std::process::Command;

use anyhow::{Context, Result};
use git2::Status;

use crate::branches::{self, Branch};

pub fn run() {
	let branches = branches::get().expect("Failed to get branches");
	working_tree(&branches).expect("Failed to get working tree");
}

fn commit_branches(branches: &Vec<Branch>) -> Result<()> {
	let repo = git2::Repository::open(".").context("Failed to open repo")?;
	let new_file_states = [Status::INDEX_NEW, Status::WT_NEW];
	let modified_file_states = [
		Status::INDEX_MODIFIED,
		Status::WT_MODIFIED,
		Status::INDEX_RENAMED,
		Status::WT_RENAMED,
	];
	for branch in branches {
		let mut state_change_msg = "";
		// Stage branch
		let branch_status = repo.status_file(branch.path.as_path()).context(&format!(
			"Failed to get status of branch at filepath {}",
			branch.path
		))?;
		if new_file_states.contains(&branch_status) || modified_file_states.contains(&branch_status)
		{
			state_change_msg = "branch";
			Command::new("git")
				.arg("add")
				.arg(branch.path)
				.status()
				.context(format!(
					"Failed to stage branch at filepath {}",
					branch.path.to_str().unwrap()
				))?;

			if branch.pdf_path.is_some() {
				let pdf_status = repo
					.status_file(branch.pdf_path.unwrap().as_path())
					.context(&format!(
						"Failed to get status of pdf at filepath {}",
						branch.pdf_path().unwrap()
					))?;
				if new_file_states.contains(&pdf_status)
					|| modified_file_states.contains(&pdf_status)
				{
					if new_file_states.contains(&branch_status)
						&& new_file_states.contains(&pdf_status)
					{
						state_change_msg = "new"
					} else {
						state_change_msg = "modified"
					}
					Command::new("git")
						.arg("add")
						.arg(branch.pdf_path.unwrap())
						.status()
						.context(format!(
							"Failed to stage pdf found at filepath {}",
							branch.pdf_path.unwrap().to_str().unwrap()
						))?;
				};
			}
		}
	}
	Ok(())
}
