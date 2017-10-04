#[macro_use]
extern crate clap;
use clap::App;
use std::process::Command;


fn main() {
	let yaml = load_yaml!("cli.yml");
	let matches = App::from_yaml(yaml).get_matches();

	if let Some(matches) = matches.subcommand_matches("send") {
		let recipient = matches.value_of("RECIPIENT").unwrap();
		println!("*** sending to - {:?}", recipient);

		let msg = matches.value_of("MESSAGE").unwrap();
		println!("*** message is - {:?}", msg);

		if recipient != "antiochp" {
			panic!("*** only allowed to send to antiochp for now... (for safety reasons...)");
		}

		let output = Command::new("keybase")
			.arg("chat")
			.arg("send")
			.arg(recipient)
			.arg(msg)
			.output()
			.expect("failed to run keybase");

		println!("{:?}", output)
	}



    // // Gets a value for config if supplied by user, or defaults to "default.conf"
    // let config = matches.value_of("config").unwrap_or("default.conf");
    // println!("Value for config: {}", config);
	//
    // // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // // required we could have used an 'if let' to conditionally get the value)
    // println!("Using input file: {}", matches.value_of("INPUT").unwrap());
	//
    // // Vary the output based on how many times the user used the "verbose" flag
    // // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    // match matches.occurrences_of("v") {
    //     0 => println!("No verbose info"),
    //     1 => println!("Some verbose info"),
    //     2 => println!("Tons of verbose info"),
    //     3 | _ => println!("Don't be crazy"),
    // }
	//
    // // You can handle information about subcommands by requesting their matches by name
    // // (as below), requesting just the name used, or both at the same time
    // if let Some(matches) = matches.subcommand_matches("test") {
    //     if matches.is_present("debug") {
    //         println!("Printing debug info...");
    //     } else {
    //         println!("Printing normally...");
    //     }
    // }

    // more program logic goes here...
}
