use serde::Deserialize;
use strum_macros::EnumString;

#[derive(PartialEq, Debug, EnumString, Deserialize)]
pub enum Format {
	LaTeX,
	Markdown,
}
