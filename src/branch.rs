use std::env;
use std::path::PathBuf;

use anyhow::Result;
use chrono::{Datelike, Local, Month};
use num_traits::FromPrimitive;

use crate::conf::{DocumentType, Format};
use crate::locations;

#[derive(Debug, PartialEq)]
pub struct Branch {
	pub name: String,
	pub format: Format,
	pub doc_type: DocumentType,
	pub class: String,
	pub path: PathBuf,
	pub pdf_path: PathBuf,
	pub imgs_dir: PathBuf,
}

impl Branch {
	pub fn new(name: &str, format: Format, doc_type: DocumentType, class: &str) -> Result<Self> {
		let month_name = Month::from_u32(Local::now().month()).unwrap().name();
		Ok(Branch {
			path: PathBuf::from(locations::folders::BRANCHES)
				.join(&class)
				.join(month_name)
				.join(doc_type.to_string())
				.join(format!("{}{}", name, format.file_extension())),
			pdf_path: PathBuf::from(locations::folders::PDFS)
				.join(&class)
				.join(month_name)
				.join(doc_type.to_string())
				.join(format!("{}.pdf", name)),
			imgs_dir: PathBuf::from(locations::folders::IMAGES)
				.join(&class)
				.join(month_name)
				.join(&name),
			name: name.to_string(),
			format,
			doc_type,
			class: class.to_string(),
		})
	}
}

#[cfg(test)]
mod test {
	use std::path::PathBuf;

	use anyhow::Result;

	use crate::branch::Branch;
	use crate::conf::{DocumentType, Format};

	#[test]
	fn new() -> Result<()> {
		assert_eq!(
			Branch::new(
				"Working",
				Format::LaTeX,
				DocumentType::Worksheet,
				"AP Physics 2"
			)?,
			Branch {
				name: String::from("Working"),
				format: Format::LaTeX,
				doc_type: DocumentType::Worksheet,
				class: String::from("AP Physics 2"),
				path: PathBuf::from("docs/AP Physics 2/February/Worksheet/Working.tex"),
				pdf_path: PathBuf::from("pdfs/AP Physics 2/February/Worksheet/Working.pdf"),
				imgs_dir: PathBuf::from("imgs/AP Physics 2/February/Working")
			}
		);

		assert_eq!(
			Branch::new(
				"Hello World",
				Format::Markdown,
				DocumentType::Other,
				"Economics Honors"
			)?,
			Branch {
				name: String::from("Hello World"),
				format: Format::Markdown,
				doc_type: DocumentType::Other,
				class: String::from("Economics Honors"),
				path: PathBuf::from("docs/Economics Honors/February/Other/Hello World.md"),
				pdf_path: PathBuf::from("pdfs/Economics Honors/February/Other/Hello World.pdf"),
				imgs_dir: PathBuf::from("imgs/Economics Honors/February/Hello World")
			}
		);

		Ok(())
	}
}
