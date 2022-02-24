use crate::branch::Branch;
use crate::conf::Config;
use crate::out::task;

pub fn run() {
	let config = Config::read().expect("Failed to read from configuration file");
	let branches = Branch::get_all(&config).expect("Failed to get all branches");
	let branch = branches.get(0).unwrap();

	task(&format!("Moving {} to trash", branch.name), || {
		trash::delete_all([&branch.path, &branch.pdf_path, &branch.imgs_dir])
			.expect("Failed to move branch file, PDF, or images directory to trash");
	})
}
