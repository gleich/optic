use std::{env, fs, process};

use anyhow::{Context, Result};
use colored::Colorize;
use dialoguer::theme::Theme;
use dialoguer::{Confirm, Input, Select};
use which::which;

use crate::conf::{Class, Config};
use crate::out::success;
use crate::{conf, out};

pub fn run(prompt_theme: &dyn Theme) {
	check_env().expect("Required binary check failed");
	confirm(prompt_theme).expect("Failed to confirm setup with user");
	let (toml, config) =
		ask_config(prompt_theme).expect("Failed to ask some questions to generate a config file");
	create(toml, config).expect("Failed to create everything");
}

/// Confirm with the user that they want to create a kiwi project in the current working directory
fn confirm(prompt_theme: &dyn Theme) -> Result<()> {
	let cwd = env::current_dir().context("Failed to get current directory")?;
	if !Confirm::with_theme(prompt_theme)
		.with_prompt(format!("Created kiwi project in {:?}?", &cwd))
		.interact()
		.context("Failed to confirm setup with user")?
	{
		process::exit(0);
	}
	Ok(())
}

/// Make sure that the user has the required programs installed and everything is good to go
fn check_env() -> Result<()> {
	let bins = ["git", "pandoc", "pdflatex"];
	for binary in bins {
		let path = which(binary).context(format!("Missing required binary: {}", binary))?;
		success(&format!(
			"Required binary {} installed at {}",
			binary.green().underline(),
			path.to_str().unwrap().green().underline()
		));
	}
	success("Environment checked, initiating setup procedure.\n");
	Ok(())
}

/// Ask the user a number of questions to generate a toml configuration file
fn ask_config(prompt_theme: &dyn Theme) -> Result<(String, Config)> {
	println!(
		"\nYou're now going to input classes. Input \"Done\" as the class name once you've \
		 inputted all classes.\n"
	);
	let mut classes: Vec<Class> = Vec::new();
	loop {
		println!("  Class #{}", classes.len() + 1);
		let name: String = Input::with_theme(prompt_theme)
			.with_prompt("Name")
			.interact()
			.context("Failed to ask the name of the class")?;
		if name.to_lowercase() == "done" {
			break;
		}
		classes.push(Class {
			name,
			teacher: Input::with_theme(prompt_theme)
				.with_prompt("Teacher's Name")
				.interact()
				.context("Failed to ask the name of the teacher")?,
		});
		println!("");
	}

	println!("\nYou're now going to input some general information.\n");
	let school_levels = ["Freshman", "Sophomore", "Junior", "Senior"];
	let school_types = ["High School", "College"];
	let config = Config {
		name: Input::with_theme(prompt_theme)
			.with_prompt("First and last name?")
			.interact()
			.context("Failed to ask for first and last name")?,
		school: conf::School {
			level: school_levels
				.get(
					Select::with_theme(prompt_theme)
						.with_prompt("School level")
						.default(0)
						.items(&school_levels)
						.interact()
						.context("Failed to ask for school level")?,
				)
				.unwrap()
				.to_string(),

			type_name: school_types
				.get(
					Select::with_theme(prompt_theme)
						.with_prompt("School type")
						.default(0)
						.items(&school_types)
						.interact()
						.context("Failed to ask for school type")?,
				)
				.unwrap()
				.to_string(),
		},
		classes,
		..Default::default()
	};
	Ok((toml::to_string(&config)?, config))
}

/// Create all the files and run any setup commands needed
fn create(toml: String, config: Config) -> Result<()> {
	let readme_template = format!(
		"# {} Year of {}

🥝 A new [kiwi](https://github.com/gleich/kiwi) project.
",
		config.school.level, config.school.type_name
	);

	// Write to files
	println!("\n--- Creating Everything ---");
	fs::write("README.md", readme_template).context("Failed to write to README.md file")?;
	out::success("Created to README.md");

	fs::write(conf::FNAME, toml).context("Failed to write to configuration file (kiwi.toml)")?;
	out::success(&format!(
		"Created to kiwi configuration file ({})",
		conf::FNAME
	));

	fs::write(".gitignore", files::GITIGNORE).context("Failed to write to git ignore file")?;
	out::success("Created .gitignore");

	// Run commands
	println!();
	process::Command::new("git")
		.args(["init", "."])
		.status()
		.context("Failed to run git init .")?;

	Ok(())
}

mod files {
	pub const GITIGNORE: &str = "## Core latex/pdflatex auxiliary files:
*.aux
*.lof
*.log
*.lot
*.fls
*.out
*.toc

## Intermediate documents:
*.dvi
*-converted-to.*
# these rules might exclude image files for figures etc.
# *.ps
# *.eps
# *.pdf

## Bibliography auxiliary files (bibtex/biblatex/biber):
*.bbl
*.bcf
*.blg
*-blx.aux
*-blx.bib
*.brf
*.run.xml

## Build tool auxiliary files:
*.fdb_latexmk
*.synctex
*.synctex.gz
*.synctex.gz(busy)
*.pdfsync

## Auxiliary and intermediate files from other packages:


# algorithms
*.alg
*.loa

# achemso
acs-*.bib

# amsthm
*.thm

# beamer
*.nav
*.snm
*.vrb

#(e)ledmac/(e)ledpar
*.end
*.[1-9]
*.[1-9][0-9]
*.[1-9][0-9][0-9]
*.[1-9]R
*.[1-9][0-9]R
*.[1-9][0-9][0-9]R
*.eledsec[1-9]
*.eledsec[1-9]R
*.eledsec[1-9][0-9]
*.eledsec[1-9][0-9]R
*.eledsec[1-9][0-9][0-9]
*.eledsec[1-9][0-9][0-9]R

# glossaries
*.acn
*.acr
*.glg
*.glo
*.gls

# gnuplottex
*-gnuplottex-*

# hyperref
*.brf

# knitr
*-concordance.tex
*.tikz
*-tikzDictionary

# listings
*.lol

# makeidx
*.idx
*.ilg
*.ind
*.ist

# minitoc
*.maf
*.mtc
*.mtc[0-9]
*.mtc[1-9][0-9]

# minted
_minted*
*.pyg

# morewrites
*.mw

# mylatexformat
*.fmt

# nomencl
*.nlo

# sagetex
*.sagetex.sage
*.sagetex.py
*.sagetex.scmd

# sympy
*.sout
*.sympy
sympy-plots-for-*.tex/

# TikZ & PGF
*.dpth
*.md5
*.auxlock

# todonotes
*.tdo

# xindy
*.xdy

# WinEdt
*.bak
*.sav

# macOS data
.DS_Store

# LaTeX formatter temp file
__latexindent_temp.tex
";
}
