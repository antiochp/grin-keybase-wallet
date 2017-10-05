#[macro_use]
extern crate clap;
extern crate time;
extern crate glob;

use clap::App;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use glob::glob;


fn main() {
	let yaml = load_yaml!("cli.yml");
	let matches = App::from_yaml(yaml).get_matches();

	if let Some(matches) = matches.subcommand_matches("receive") {
		let recipient = "antiochp";
		poll_for_files(recipient);
	}

	if let Some(matches) = matches.subcommand_matches("send") {
		let sender = "alt_antiochp";

		let recipient = matches.value_of("RECIPIENT").unwrap();
		println!("*** sending to - {:?}", recipient);

		let msg = matches.value_of("MESSAGE").unwrap();
		println!("*** message is - {:?}", msg);

		if recipient != "antiochp" {
			panic!("*** only allowed to send to antiochp for now... (for safety reasons...)");
		}

		write_txn(sender, recipient);
	}
}

fn write_txn(sender: &str, recipient: &str) {
	let path = format!(
		"/keybase/private/{},{}/grinbox_{}/{}_txn.json",
		sender, recipient, recipient, time::now_utc().strftime("%Y%m%d%H%M%S_%f").unwrap());
	let path = Path::new(&path);
	let display = path.display();

	// Open a file in write-only mode, returns `io::Result<File>`
	let mut file = match File::create(&path) {
		Err(why) => panic!("couldn't create {}: {}",
						   display,
						   why.description()),
		Ok(file) => file,
	};

	match file.write_all("this is not json".as_bytes()) {
		Err(why) => {
			panic!("couldn't write to {}: {}", display, why.description())
		},
		Ok(_) => println!("successfully wrote to {}", display),
	}
}

fn poll_for_files(recipient: &str) {
	let pattern = format!("/keybase/private/{},*/grinbox_{}/*_txn.json", recipient, recipient);
	for entry in glob(&pattern).expect("Failed to read glob pattern") {
		match entry {
			Ok(path) => {
				println!("{:?}", path.display());
				
			},
			Err(e) => println!("{:?}", e),
		}
	}
}
