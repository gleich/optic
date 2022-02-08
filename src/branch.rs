use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::SystemTime;

use anyhow::{bail, Context, Result};
use chrono::{Date, Datelike, Local, Month, NaiveDate, TimeZone};
use handlebars::Handlebars;
use num_traits::FromPrimitive;
use ordinal::Ordinal;
use serde_json::json;
use walkdir::WalkDir;

use crate::conf::{Class, Config, DocumentType, Format};
use crate::locations::{self, folders};
use crate::template::{BranchTemplate, RootTemplate};

#[derive(Debug, PartialEq)]
pub struct Branch {
	pub name: String,
	pub format: Format,
	pub doc_type: DocumentType,
	pub class: Class,
	pub path: PathBuf,
	pub pdf_path: PathBuf,
	pub imgs_dir: PathBuf,
	pub branch_template: Option<BranchTemplate>,
	pub root_template: RootTemplate,
	pub creation_time: Date<Local>,
	pub mod_time: SystemTime,
}

impl Branch {
	pub fn new(
		name: String,
		format: Format,
		doc_type: DocumentType,
		class: Class,
		branch_template: Option<BranchTemplate>,
		root_template: RootTemplate,
		creation_time: Date<Local>,
		mod_time: SystemTime,
	) -> Result<Self> {
		let month_name = Month::from_u32(creation_time.month()).unwrap().name();
		Ok(Branch {
			path: PathBuf::from(locations::folders::BRANCHES)
				.join(&class.name)
				.join(month_name)
				.join(doc_type.to_string())
				.join(format!("{}{}", name, format.extension())),
			pdf_path: PathBuf::from(locations::folders::PDFS)
				.join(&class.name)
				.join(month_name)
				.join(doc_type.to_string())
				.join(format!("{}.pdf", name)),
			imgs_dir: PathBuf::from(locations::folders::IMAGES)
				.join(&class.name)
				.join(month_name)
				.join(&name),
			name,
			format,
			doc_type,
			class,
			branch_template,
			root_template,
			creation_time,
			mod_time,
		})
	}

	pub fn inject(
		&self,
		config: &Config,
		template_content: String,
		time: Date<Local>,
	) -> Result<String> {
		fn custom_escape(s: &str, format: &Format) -> String {
			if *format == Format::Markdown {
				return s.to_string();
			}
			let mut output = String::new();
			for (i, c) in s.chars().enumerate() {
				if s.chars().nth(i - 1).unwrap_or_default().to_string() == "\\".to_string() {
					output.push(c);
					continue;
				}
				match c {
					'&' => output.push_str("\\&"),
					'$' => output.push_str("\\$"),
					'#' => output.push_str("\\#"),
					'%' => output.push_str("\\%"),
					_ => output.push(c),
				}
			}
			output
		}

		let ordinal_suffix = Ordinal(time.day()).suffix();
		let mut reg = Handlebars::new();
		reg.set_strict_mode(true);
		reg.register_escape_fn(handlebars::no_escape);

		let branch_template = self
			.branch_template
			.clone()
			.unwrap_or(BranchTemplate::default());

		Ok(reg.render_template(
			&template_content,
			&json!({
				"time": {
					"simple_date": time.format("%F").to_string(),
					"day": time.day(),
					"year": time.year(),
					"date": match self.format {
						Format::Markdown => time.format(&format!("%A, %B %e^{}^, %Y", ordinal_suffix)).to_string(),
						Format::LaTeX => time.format(&format!("%A, %B %e\\textsuperscript{{{}}}, %Y", ordinal_suffix)).to_string()
					},
					"month": time.format("%B").to_string()
				},
				"author": config.author,
				"name": custom_escape(&self.name, &self.format),
				"class": {
					"name": custom_escape(&self.class.name, &self.format),
					"teacher": self.class.teacher,
				},
				"branch": {
					"filename": &branch_template.path.file_name().unwrap().to_str().unwrap().to_string(),
					"content": fs::read_to_string(&branch_template.path)?,
				},
				"root": {
					"filename": self.root_template.path.file_name().unwrap().to_str().unwrap().to_string()
				},
				"type": self.doc_type.to_string(),
				"required_preamble": include_str!("required_preamble.tex")
			}),
		)?)
	}

	pub fn parse(path: PathBuf, config: &Config) -> Result<Self> {
		let format = Format::from_path(&path).unwrap();
		let mut data = HashMap::new();
		for line in fs::read_to_string(&path)?.lines() {
			let trimmed_line = line.trim();
			let raw_chunks = trimmed_line.split_once(&config.delimiter);
			if raw_chunks.is_none() {
				continue;
			}
			let chunks = raw_chunks.unwrap();
			data.insert(chunks.0.trim().to_string(), chunks.1.trim().to_string());
			if format == Format::Markdown && trimmed_line.starts_with("-->")
				|| format == Format::LaTeX && trimmed_line.starts_with("\\fi")
			{
				break;
			}
		}
		let required_keys = ["created", "root"];
		for key in required_keys {
			if !data.contains_key(key) {
				bail!("{} is missing required key: {}", path.display(), key);
			}
		}

		let path_chunks = path.iter().rev();
		let doc_type = path_chunks.clone().nth(1).unwrap().to_str().unwrap();
		let class_name = path_chunks.clone().nth(3).unwrap().to_str().unwrap();

		Ok(Self::new(
			path.file_name()
				.unwrap()
				.to_str()
				.unwrap()
				.strip_suffix(format.extension())
				.unwrap()
				.to_string(),
			format,
			DocumentType::from_str(doc_type)
				.context(format!("Failed to pair document type {}", &path.display()))?,
			config
				.classes
				.iter()
				.find(|c| c.name == class_name)
				.unwrap()
				.clone(),
			None,
			RootTemplate::from_filename(data.get("root").unwrap()),
			Local
				.from_local_date(&NaiveDate::parse_from_str(
					data.get("created").unwrap(),
					"%F",
				)?)
				.unwrap(),
			fs::metadata(path)?.modified()?,
		)?)
	}

	pub fn get_all(config: &Config) -> Result<Vec<Self>> {
		let mut branches: Vec<Self> = Vec::new();
		for entry in WalkDir::new(folders::BRANCHES) {
			let entry = entry?;
			let extension = Format::from_path(&entry.path().to_path_buf());
			if entry.file_type().is_file() && extension.is_some() {
				branches.push(Self::parse(entry.path().to_path_buf(), config)?)
			}
		}
		branches.sort_by(|a, b| b.mod_time.cmp(&a.mod_time));
		Ok(branches)
	}
}

#[cfg(test)]
mod test {
	use std::path::PathBuf;
	use std::time::SystemTime;

	use anyhow::Result;
	use chrono::Local;

	use crate::branch::Branch;
	use crate::conf::{Class, DocumentType, Format};
	use crate::template::{BranchTemplate, RootTemplate};

	#[test]
	fn new() -> Result<()> {
		let date_now = Local::now().date();
		let systemtime_now = SystemTime::now();
		assert_eq!(
			Branch::new(
				String::from("Working"),
				Format::LaTeX,
				DocumentType::Worksheet,
				Class {
					name: String::from("AP Physics 2"),
					teacher: String::from("Mr. Feynman"),
					active: true
				},
				Some(BranchTemplate {
					path: PathBuf::from("./templates/branch/base.tex.hbs"),
					name: String::from("base"),
					format: Format::LaTeX,
				}),
				RootTemplate {
					path: PathBuf::from("./templates/root/base.tex.hbs"),
					name: String::from("base")
				},
				date_now,
				systemtime_now
			)?,
			Branch {
				name: String::from("Working"),
				format: Format::LaTeX,
				doc_type: DocumentType::Worksheet,
				class: Class {
					name: String::from("AP Physics 2"),
					teacher: String::from("Mr. Feynman"),
					active: true
				},
				path: PathBuf::from("docs/AP Physics 2/February/Worksheet/Working.tex"),
				pdf_path: PathBuf::from("pdfs/AP Physics 2/February/Worksheet/Working.pdf"),
				imgs_dir: PathBuf::from("imgs/AP Physics 2/February/Working"),
				branch_template: Some(BranchTemplate {
					path: PathBuf::from("./templates/branch/base.tex.hbs"),
					name: String::from("base"),
					format: Format::LaTeX,
				}),
				root_template: RootTemplate {
					path: PathBuf::from("./templates/root/base.tex.hbs"),
					name: String::from("base")
				},
				creation_time: date_now,
				mod_time: systemtime_now
			}
		);

		assert_eq!(
			Branch::new(
				String::from("Hello World"),
				Format::Markdown,
				DocumentType::Other,
				Class {
					name: String::from("Economics Honors"),
					teacher: String::from("Mr. Buffet"),
					active: true
				},
				Some(BranchTemplate {
					path: PathBuf::from("./templates/branch/base.tex.hbs"),
					name: String::from("base"),
					format: Format::LaTeX,
				}),
				RootTemplate {
					path: PathBuf::from("./templates/root/base.tex.hbs"),
					name: String::from("base")
				},
				date_now,
				systemtime_now
			)?,
			Branch {
				name: String::from("Hello World"),
				format: Format::Markdown,
				doc_type: DocumentType::Other,
				class: Class {
					name: String::from("Economics Honors"),
					teacher: String::from("Mr. Buffet"),
					active: true
				},
				path: PathBuf::from("docs/Economics Honors/February/Other/Hello World.md"),
				pdf_path: PathBuf::from("pdfs/Economics Honors/February/Other/Hello World.pdf"),
				imgs_dir: PathBuf::from("imgs/Economics Honors/February/Hello World"),
				branch_template: Some(BranchTemplate {
					path: PathBuf::from("./templates/branch/base.tex.hbs"),
					name: String::from("base"),
					format: Format::LaTeX,
				}),
				root_template: RootTemplate {
					path: PathBuf::from("./templates/root/base.tex.hbs"),
					name: String::from("base")
				},
				creation_time: date_now,
				mod_time: systemtime_now
			}
		);

		Ok(())
	}
}
