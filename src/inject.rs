use anyhow::Result;
use chrono::{Datelike, Local};
use handlebars::Handlebars;
use ordinal::Ordinal;
use serde_json::json;

use crate::conf::{Config, Format};

pub fn base(
	branch_filename: String,
	root_filename: &str,
	class_name: String,
	format: &Format,
	config: &Config,
	template_string: String,
) -> Result<String> {
	let now = Local::now();
	Ok(Handlebars::new().render_template(
		&template_string,
		&json!({
			"time": {
				"simple_date": now.format("%F").to_string(),
				"day": now.day(),
				"year": now.year(),
				"date": match format {
					Format::Markdown => now.format(&format!("%A, %B ^{}^, %Y", Ordinal(now.day()))).to_string(),
					Format::LaTeX => now.format(&format!("%A, %B \\textsuperscript{{{}}}, %Y", Ordinal(now.day()))).to_string()
				}
			},
			"branch_filename": branch_filename,
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
		}),
	)?)
}
