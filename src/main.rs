#[macro_use]
extern crate clap;
extern crate time;

use clap::App;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn main() {
	let yaml = load_yaml!("cli.yml");
	let matches = App::from_yaml(yaml).get_matches();

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

		// chat is basically a non-starter
		// we would need to use attachments
		// and at that point we may as well interact directly with kbfs
		// and gain the additional flexibility of dir structure etc.

		// so write a file to
		// /keybase/private/<from>,<recipient>/grinbox_<recipient>/<timestamp>_foo_txn


		// let output = Command::new("keybase")
		// 	.arg("chat")
		// 	.arg("send")
		// 	.arg(recipient)
		// 	.arg(msg)
		// 	.output()
		// 	.expect("failed to run keybase");

		// println!("{:?}", output)
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

	// Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
	match file.write_all("this is not json".as_bytes()) {
		Err(why) => {
			panic!("couldn't write to {}: {}", display, why.description())
		},
		Ok(_) => println!("successfully wrote to {}", display),
	}
}

fn poll_for_messages() {

}
