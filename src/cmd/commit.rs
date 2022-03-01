use std::collections::HashMap;
use std::process::Command;

use anyhow::{Context, Result};
use git2::Status;
use task_log::task;

use crate::branch::Branch;
use crate::conf::Config;

pub fn run() {
	let config = Config::read().expect("Failed to read configuration");
	let branches = Branch::get_all(&config).expect("Failed to get all branches");
	let branches_to_commit = working_branches(branches).expect("Failed to get working branches");
	for (msg, branch) in branches_to_commit {
		task(format!("Committing {}", branch.name), || {
			commit_branch(msg, &branch).expect(&format!("Failed to commit {}", branch.name));
		})
	}
}

fn working_branches(branches: Vec<Branch>) -> Result<HashMap<String, Branch>> {
	let repo = git2::Repository::open(".").context("Failed to open repo")?;

	let new_file_states = [Status::INDEX_NEW, Status::WT_NEW];
	let modified_file_states = [
		Status::INDEX_MODIFIED,
		Status::WT_MODIFIED,
		Status::INDEX_RENAMED,
		Status::WT_RENAMED,
	];
	let deleted_file_states = [Status::INDEX_DELETED, Status::WT_DELETED];

	let mut working = HashMap::new();
	for branch in branches {
		let branch_status = repo.status_file(branch.path.as_path())?;
		if new_file_states.contains(&branch_status) {
			working.insert(
				format!("new({}): {}", branch.doc_type.to_string(), branch.name),
				branch,
			);
		} else if modified_file_states.contains(&branch_status) {
			working.insert(
				format!("update({}): {}", branch.doc_type.to_string(), branch.name),
				branch,
			);
		} else if deleted_file_states.contains(&branch_status) {
			working.insert(
				format!("delete({}): {}", branch.doc_type.to_string(), branch.name),
				branch,
			);
		}
	}

	Ok(working)
}

fn commit_branch(msg: String, branch: &Branch) -> Result<()> {
	let git_binary = "git";

	let mut stage_cmd = Command::new(git_binary);
	stage_cmd.arg("add").arg(&branch.path);
	if branch.pdf_path.exists() {
		stage_cmd.arg(&branch.pdf_path);
	}
	if branch.imgs_dir.exists() {
		stage_cmd.arg(&branch.imgs_dir);
	}

	stage_cmd.output()?;

	Command::new(git_binary)
		.arg("commit")
		.arg("-m")
		.arg(msg)
		.output()?;

	Ok(())
}
