use std::sync::mpsc::channel;
use std::time::Duration;

use chrono::Local;
use chrono_humanize::{Accuracy, HumanTime, Tense};
use clap::ArgMatches;
use colorful::Colorful;
use notify::{DebouncedEvent, RecommendedWatcher, Watcher};
use task_log::task;

use crate::branch::Branch;
use crate::conf::Config;

pub fn run(args: &ArgMatches) {
	let config = Config::read().expect("Failed to read from configuration file");
	let branches = Branch::get_all(&config).expect("Failed to get all branches");
	let branch = branches.get(0).unwrap();

	let (tx, rx) = channel();
	let mut watcher: RecommendedWatcher =
		Watcher::new(tx, Duration::from_millis(20)).expect("Failed to setup watcher");
	watcher
		.watch(&branch.path, notify::RecursiveMode::NonRecursive)
		.expect("Failed to watch recent branch file");

	watcher
		.watch(
			&branch.root_template.path,
			notify::RecursiveMode::NonRecursive,
		)
		.expect("Failed to watch branch's root template file");

	task(
		format!(
			"Opening \"{}\" with {}",
			branch.name,
			config.view_with.as_ref().unwrap().get(0).unwrap()
		),
		|| {
			branch
				.view(&config, false, true)
				.expect("Failed to open branch with viewer");
		},
	);

	loop {
		let event = rx.recv().expect("Failed to receive event");
		match event {
			DebouncedEvent::Write(_) => {
				let start = Local::now();
				println!(
					"\n  {}",
					format!(" BUILD INCOMING at {} ", start.format("%x %r"))
						.bg_yellow()
						.black()
				);
				let result = branch.build(&config, &(config.latexmk || args.is_present("latexmk")));
				if result.is_err() {
					println!("   {}", "BUILD FAILED".red());
				} else {
					println!(
						"   {}",
						format!(
							"BUILD DONE after {}",
							HumanTime::from(Local::now() - start)
								.to_text_en(Accuracy::Precise, Tense::Present)
						)
						.green()
						.underlined()
					)
				}
			}
			DebouncedEvent::NoticeRemove(path) | DebouncedEvent::Remove(path) => {
				println!("\n{} has been deleted. Stopping watch", path.display());
				return;
			}
			_ => (),
		}
	}
}
