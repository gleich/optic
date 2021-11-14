use colored::Colorize;
use console::{style, Style};
use dialoguer::theme::ColorfulTheme;

const CHECK_CHARACTER: &str = "✔";
pub const ARROW_CHARACTERS: &str = ">";

pub fn custom_dialoguer_theme() -> ColorfulTheme {
	let green_check = CHECK_CHARACTER.green();
	let mut theme = ColorfulTheme::default();
	theme.success_prefix = style(green_check.to_string()).for_stderr();
	theme.success_suffix = style(ARROW_CHARACTERS.to_string()).for_stderr();
	theme.values_style = theme.values_style.bold().underlined();
	theme.prompt_style = Style::new().for_stderr().bold();
	theme
}

pub fn success(message: &str) {
	println!("{} {}", CHECK_CHARACTER.green(), message.bold());
}

/// Output a message saying that a value was obtained
pub fn got_value(name: &str, value: &str) {
	success(&format!(
		"{} {} {}",
		name.bold(),
		ARROW_CHARACTERS,
		value.bold().underline().green()
	))
}
