mod cli;

fn main() {
	let matches = cli::setup();
	println!("{}", matches.is_present("setup"));
}
