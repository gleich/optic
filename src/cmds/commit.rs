use std::process::Command;

use anyhow::{Context, Result};
use git2::Status;

use crate::branches::{self, Branch};
use crate::out::success;

pub fn run() {
	let branches = branches::get().expect("Failed to get branches");
	commit_branches(&branches).expect("Failed to get working tree");
}

/// This function will commit branches according to the following steps:
/// 1. Check that the branch is modified/new
/// 2. Check if the branch's PDF is new/modified
/// 3. If only the branch is new/modified -> stage and commit branch
/// 4. If the branch and matching PDF are changed -> stage both and commit them
fn commit_branches(branches: &Vec<Branch>) -> Result<()> {
	let repo = git2::Repository::open(".").context("Failed to open repo")?;

	let new_file_states = [Status::INDEX_NEW, Status::WT_NEW];
	let modified_file_states = [
		Status::INDEX_MODIFIED,
		Status::WT_MODIFIED,
		Status::INDEX_RENAMED,
		Status::WT_RENAMED,
	];
	let deleted_file_states = [Status::INDEX_DELETED, Status::WT_DELETED];

	for branch in branches {
		let mut state_change_msg;

		let branch_status = repo.status_file(branch.path.as_path()).context(format!(
			"Failed to get status of branch at filepath {}",
			branch.path.to_str().unwrap()
		))?;
		if new_file_states.contains(&branch_status)
			|| modified_file_states.contains(&branch_status)
			|| deleted_file_states.contains(&branch_status)
		{
			state_change_msg = "branch";
			Command::new("git")
				.arg("add")
				.arg(&branch.path)
				.status()
				.context(format!(
					"Failed to stage branch at filepath {}",
					branch.path.to_str().unwrap()
				))?;

			if branch.pdf_path.is_some() {
				let pdf_status = repo
					.status_file(branch.pdf_path.as_ref().unwrap().as_path())
					.context(format!(
						"Failed to get status of pdf at filepath {}",
						branch.pdf_path.as_ref().unwrap().to_str().unwrap()
					))?;
				if new_file_states.contains(&pdf_status)
					|| modified_file_states.contains(&pdf_status)
					|| deleted_file_states.contains(&pdf_status)
				{
					if new_file_states.contains(&branch_status)
						&& new_file_states.contains(&pdf_status)
					{
						state_change_msg = "new"
					} else if deleted_file_states.contains(&branch_status)
						&& deleted_file_states.contains(&pdf_status)
					{
						state_change_msg = "deleted"
					} else {
						state_change_msg = "modified"
					}
					Command::new("git")
						.arg("add")
						.arg(branch.pdf_path.as_ref().unwrap())
						.status()
						.context(format!(
							"Failed to stage pdf found at filepath {}",
							branch.pdf_path.as_ref().unwrap().to_str().unwrap()
						))?;
				};
			}

			Command::new("git")
				.arg("commit")
				.arg("-m")
				.arg(format!(
					"feat({}): {} {}",
					state_change_msg,
					branch.name,
					branch.doc_type.to_string()
				))
				.status()
				.context(format!(
					"Failed to commit branch located at path {}",
					branch.path.to_str().unwrap()
				))?;
			success(&format!("Committed {}", branch.name));
		}
	}
	Ok(())
}
