use conf::Config;

mod conf;
mod format;

fn main() {
	let config = Config::read().expect("Failed to read config file");
	println!("{:?}", config);
}
