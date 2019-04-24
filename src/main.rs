use std::env;
use std::process;

use minigrep::Config;

fn main() {
	let config = Config::new(env::args()).unwrap_or_else(|err| {
		eprint!("Problem parsing elements: {}", err);
		process::exit(1);
	});

	println!("Serching for {}", config.query);
	println!("In File {}", config.filename);

	if let Err(e) = minigrep::run(config) {
		eprintln!("Application error: {}", e);
		process::exit(1)		
	}
}

