use crate::branches;

pub fn run() {
	let branches = branches::get().expect("Failed to get branches");
	println!("{}", branches.len());
}
