use std::process;

use colored::Colorize;

/// Output a success message
pub fn success(message: &str) {
	println!("{} {}", "✔".green(), message.bold());
}

/// Output a problem message
pub fn problem(message: &str) {
	println!("{} {}", "✗".red(), message.bold());
	process::exit(1);
}
