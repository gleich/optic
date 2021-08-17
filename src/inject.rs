use anyhow::Result;
use chrono::{Datelike, Local};
use handlebars::Handlebars;
use ordinal::Ordinal;
use serde_json::json;

use crate::conf::{Config, Format};

pub struct BaseData<'a> {
	pub doc_name: &'a String,
	pub class_name: &'a String,
	pub format: &'a Format,
	pub config: &'a Config,
}

impl BaseData<'_> {
	pub fn inject_into(&self, template_string: &str) -> Result<String> {
		let now = Local::now();
		Ok(Handlebars::new().render_template(
			template_string,
			&json!({
				"time": {
					"raw_date": &now.date().to_string(),
					"day": &now.day(),
					"year": &now.year(),
					"date": match &self.format {
						Format::Markdown => now.format(&format!("%A, %B ^{}^, %Y", Ordinal(now.day()))).to_string(),
						Format::LaTeX => now.format(&format!("%A, %B \\textsuperscript{{{}}}, %Y", Ordinal(now.day()))).to_string()
					}
				},
				"file_name": &self.doc_name,
                "name": &self.doc_name.replace("_", " ").replace("-", " ").trim_end_matches(match &self.format {
						Format::Markdown => ".md",
						Format::LaTeX => ".tex",
					}),
				"author": &self.config.name,
				"class": {
					"name": &self.class_name,
					"teacher": &self.config.classes.iter().find(|c| &c.name == self.class_name).unwrap().teacher,
				},
				"school": {
					"type": &self.config.school.type_name,
					"level": &self.config.school.level,
				},
			}),
		)?)
	}
}
