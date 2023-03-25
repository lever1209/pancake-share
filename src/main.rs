#![feature(fn_traits)]

use std::path::PathBuf;

use network as net; 

use clap::{arg, command, Command};

mod interactive_cli;
mod network;

fn main() {
	let matches = command!()
		.subcommand_required(false)
		// .arg_required_else_help(true)
		.propagate_version(true)
		.version(env!("CARGO_PKG_VERSION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.about(env!("CARGO_PKG_DESCRIPTION"))
		.subcommand(
			Command::new("server").arg(
				arg!(-c --config <VALUE> "Path to an alternate config file.")
					.id("config_path")
					.action(clap::ArgAction::Set),
			),
		)
		.get_matches();

	match matches.subcommand() {
		Some(("server", matched_command)) => {
			println!("server executing");
			
			println!("{:?}",matched_command.get_one::<String>("config_path"));
			
			net::server::init();
		}
		None => eprintln!("no command Supplied."),
		_ => {
			panic!("????")
		} // clap handles this
	}
}

#[cfg(feature = "windows-os")]
fn hide_console_window() {
	unsafe {
		winapi::um::wincon::FreeConsole();
	}
}

mod config {
	// use std::{
	// 	fs,
	// 	io::BufReader,
	// 	net::{Ipv4Addr, SocketAddr, SocketAddrV4},
	// 	path::Path,
	// };

	// use serde::{Deserialize, Serialize};

	// #[derive(Deserialize, Serialize, Debug)]
	// struct Contact {
	// 	known_ips: Vec<SocketAddr>,
	// }

	pub fn load_config(path: Option<&String>) {
		// 	let binding = get_home()
		// 		.unwrap()
		// 		.join(Path::new(".config/pancake-share/contacts/contacts.yaml"));
		// 	let config_path = Path::new(&binding);
		// 	dbg!(config_path);
		// 	let config_file = fs::OpenOptions::new()
		// 		.write(true)
		// 		.read(true)
		// 		.create(true)
		// 		.open(config_path)
		// 		.expect("open file");

		// 	// let config_file = fs::File::open(config_path).expect("File doesnt exist.");
		// 	dbg!(&config_file);

		// 	// let buf = [0;512];
		// 	let reader = BufReader::new(config_file);

		// 	let conf: Result<Contact, serde_yaml::Error> = serde_yaml::from_reader(reader); // .expect("Failed to read reader.");

		// 	dbg!(conf);

		// 	let contact = Contact {
		// 		known_ips: vec![std::net::SocketAddr::V4(SocketAddrV4::new(
		// 			Ipv4Addr::new(0, 0, 0, 0),
		// 			get_port(),
		// 		))],
		// 	};

		// 	// let serde_serialized_contact = contact.serialize(serde_yaml::Serializer);

		// 	// println!(serde_serialized_contact);

		// 	// exam bus home: 12:15 3:44
		// }

		// fn get_home() -> std::option::Option<std::path::PathBuf> {
		// 	home::home_dir()
	}

	fn create_required_config_folders() {

		// let home = home::home_dir();

		// let dirs = [];

		// fs::OpenOptions::new().write(true).read(true).create(true).open().expect("Unable to create config file dir.")
	}

	pub fn get_port() -> u16 {
		59217
	}
}
