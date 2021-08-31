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
	reg.register_escape_fn(handlebars::no_escape);
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
			"name": custom_escape(branch_filename.replace("_", " ").replace("-", " ").trim_end_matches(".md").trim_end_matches(".tex"), format),
			"root_filename": root_filename,
			"author": config.name,
			"class": {
				"name": custom_escape(class_name, format),
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
			"required_preamble": "
% PANDOC STUFF:
\\usepackage{iftex}
\\ifPDFTeX
  \\usepackage[T1]{fontenc}
  \\usepackage[utf8]{inputenc}
  \\usepackage{textcomp} % provide euro and other symbols
\\else % if luatex or xetex
  \\usepackage{unicode-math}
  \\defaultfontfeatures{Scale=MatchLowercase}
  \\defaultfontfeatures[\\rmfamily]{Ligatures=TeX,Scale=1}
\\fi
% Use upquote if available, for straight quotes in verbatim environments
\\IfFileExists{upquote.sty}{\\usepackage{upquote}}{}
\\IfFileExists{microtype.sty}{% use microtype if available
  \\usepackage[]{microtype}
  \\UseMicrotypeSet[protrusion]{basicmath} % disable protrusion for tt fonts
}{}
\\makeatletter
\\@ifundefined{KOMAClassName}{% if non-KOMA class
  \\IfFileExists{parskip.sty}{%
    \\usepackage{parskip}
  }{% else
    \\setlength{\\parindent}{0pt}
    \\setlength{\\parskip}{6pt plus 2pt minus 1pt}}
}{% if KOMA class
  \\KOMAoptions{parskip=half}}
\\makeatother
\\usepackage{xcolor}
\\IfFileExists{xurl.sty}{\\usepackage{xurl}}{} % add URL line breaks if available
\\IfFileExists{bookmark.sty}{\\usepackage{bookmark}}{\\usepackage{hyperref}}
\\hypersetup{
  hidelinks,
  pdfcreator={LaTeX via pandoc}}
\\urlstyle{same} % disable monospaced font for URLs
\\usepackage{longtable,booktabs,array}
\\usepackage{calc} % for calculating minipage widths
% Correct order of tables after \\paragraph or \\subparagraph
\\usepackage{etoolbox}
\\makeatletter
\\patchcmd\\longtable{\\par}{\\if@noskipsec\\mbox{}\\fi\\par}{}{}
\\makeatother
% Allow footnotes in longtable head/foot
\\IfFileExists{footnotehyper.sty}{\\usepackage{footnotehyper}}{\\usepackage{footnote}}
\\makesavenoteenv{longtable}
\\setlength{\\emergencystretch}{3em} % prevent overfull lines
\\providecommand{\\tightlist}{%
  \\setlength{\\itemsep}{0pt}\\setlength{\\parskip}{0pt}}
\\setcounter{secnumdepth}{-\\maxdimen} % remove section numbering
\\ifLuaTeX
  \\usepackage{selnolig}  % disable illegal ligatures
\\fi
"
		}),
	).context("Handlebar template injection failed")?)
}

pub fn custom_escape(s: &str, format: &Format) -> String {
	if format == &Format::Markdown {
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
			_ => output.push(c),
		}
	}
	output
}
