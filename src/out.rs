use colorful::Colorful;

pub struct Job {
	pub task: String,
}

impl Job {
	pub fn new(task: &str) -> Self {
		Self {
			task: String::from(task),
		}
	}

	pub fn set_task(&mut self, new_task: &str) { self.task = String::from(new_task); }

	pub fn start(&self) { println!("  {}  | {}", "RUNNING".yellow(), self.task) }

	pub fn done(&self) {
		println!("\x1b[A\x1b[A");
		println!("  {}     | {}", "DONE".green(), self.task);
	}
}
