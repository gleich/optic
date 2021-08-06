use colored::Colorize;

/// Output a success message
pub fn success(message: &str) {
	println!("{} {}", "✔".green(), message.bold());
}
