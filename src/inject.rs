use anyhow::{Context, Result};
use chrono::{DateTime, Datelike, Local};
use handlebars::Handlebars;
use ordinal::Ordinal;
use serde_json::json;

use crate::conf::{Config, DocType, Format};

/// Inject a bunch of data into a template string using the handlebars template engine.
pub fn inject(
	branch_filename: String,
	root_filename: &str,
	class_name: &str,
	format: &Format,
	doc_type: &DocType,
	config: &Config,
	template_string: String,
	branch_content: Option<String>,
	time: DateTime<Local>,
) -> Result<String> {
	// Getting ordinal numeral suffix (e.g. st or th)
	let raw_ordinal = Ordinal(time.day()).to_string();
	let ordinal_suffix = raw_ordinal.trim_start_matches(&time.day().to_string());

	let mut reg = Handlebars::new();
	reg.set_strict_mode(true);
	reg.register_escape_fn(custom_escape);
	Ok(reg.render_template(
		&template_string,
		&json!({
			"time": {
				"simple_date": time.format("%F").to_string(),
				"day": time.day(),
				"year": time.year(),
				"date": match format {
					Format::Markdown => time.format(&format!("%A, %B %e^{}^, %Y", ordinal_suffix)).to_string(),
					Format::LaTeX => time.format(&format!("%A, %B %e\\textsuperscript{{{}}}, %Y", ordinal_suffix)).to_string()
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
			},
			"type": doc_type.to_string(),
			"required_preamble": "\\def\\tightlist{}"
		}),
	).context("Handlebar template injection failed")?)
}

fn custom_escape(s: &str) -> String {
	let mut output = String::with_capacity(s.len());
	for c in s.chars() {
		match c {
			'&' => output.push_str("\\&"),
			'$' => output.push_str("\\$"),
			_ => output.push(c),
		}
	}
	output
}
