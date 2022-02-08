use crate::branch::Branch;
use crate::conf::Config;
use crate::out::Job;

pub fn run() {
	let config = Config::read().expect("Failed to read from configuration file");

	let mut job = Job::new("Collecting branches");
	job.start();
	let branches = Branch::get_all(&config).expect("Failed to get all branches");
	job.done();

	let branch = branches.get(0).unwrap();
	job.set_task("Building branch");
	job.start();
	branch.build(&config).expect("Failed to build branch");
	job.done();
}
