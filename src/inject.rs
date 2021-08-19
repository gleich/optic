use anyhow::Result;
use chrono::{Datelike, Local};
use handlebars::Handlebars;
use ordinal::Ordinal;
use serde_json::json;

use crate::conf::{Config, Format};

/// Inject a bunch of data into a template string using the handlebars template engine.
pub fn inject(
	branch_filename: String,
	root_filename: &str,
	class_name: &str,
	format: &Format,
	config: &Config,
	template_string: String,
	branch_content: Option<String>,
) -> Result<String> {
	let now = Local::now();

	// Getting ordinal numeral suffix (e.g. st or th)
	let raw_ordinal = Ordinal(now.day()).to_string();
	let ordinal_suffix = raw_ordinal.trim_start_matches(&now.day().to_string());

	Ok(Handlebars::new().render_template(
		&template_string,
		&json!({
			"time": {
				"simple_date": now.format("%F").to_string(),
				"day": now.day(),
				"year": now.year(),
				"date": match format {
					Format::Markdown => now.format(&format!("%A, %B %e^{}^, %Y", ordinal_suffix)).to_string(),
					Format::LaTeX => now.format(&format!("%A, %B %e\\textsuperscript{{{}}}, %Y", ordinal_suffix)).to_string()
				}
			},
			"name": branch_filename.replace("_", " ").replace("-", " ").trim_end_matches(match format {
					Format::Markdown => ".md",
					Format::LaTeX => ".tex",
				}),
			"root_filename": root_filename,
			"author": config.name,
			"class": {
				"name": class_name,
				"teacher": config.classes.iter().find(|c| c.name == class_name).unwrap().teacher,
			},
			"school": {
				"type": config.school.type_name,
				"level": config.school.level,
			},
			"branch": {
				"filename": branch_filename,
				"content": branch_content.unwrap_or_default()
			}
		}),
	)?)
}
