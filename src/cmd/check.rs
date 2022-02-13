use anyhow::Result;

use crate::branch::Branch;
use crate::conf::Config;

pub fn run() {
	let config = Config::read().expect("Failed to read from configuration file");
	let branches = Branch::get_all(&config).expect("Failed to get all branches");
	needs_building(&branches).expect("Failed to get branches that need building");
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
