use colorful::Colorful;

pub fn task<F>(name: &str, runner: F)
where
	F: FnOnce(),
{
	println!("  {}  | {}", "RUNNING".yellow(), name);
	runner();
	println!("\x1b[A\x1b[A");
	println!("  {}     | {}", "DONE".green(), name);
}
