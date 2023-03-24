use crate::network;

type CommandFunction = fn(Option<Vec<String>>) -> Result<Option<String>, String>;
pub struct CommandStruct<'a> {
	pub name: &'a str,
	pub alias: Option<Vec<&'a str>>,
	pub func: CommandFunction,
	pub help: Option<&'a str>,
	pub help_long: Option<&'a str>,
	pub usage: &'a str,
}

pub fn get_commands() -> [CommandStruct<'static>; 5] {
	[
		CommandStruct {
			name: "help",
			alias: Some(vec!["h"]),
			func: help_func,
			help: Some("Lists help information about the program and subcommands."),
			help_long: None,
			usage: "help [command]",
		},
		CommandStruct {
			name: "exit",
			alias: Some(vec!["q", "quit"]),
			func: exit_func,
			help: Some("Exits the program"),
			help_long: None,
			usage: "exit",
		},
		CommandStruct {
			name: "clear",
			alias: Some(vec!["cls"]),
			func: clear_screen_func,
			help: Some("clears the screen buffer"),
			help_long: None,
			usage: "clear",
		},
		CommandStruct {
			name: "test",
			alias: Some(vec!["ta"]),
			func: test_func,
			help: None,
			help_long: None,
			usage: "test <args>",
		},
		CommandStruct {
			name: "load_config",
			alias: None,
			func: load_config,
			help: None,
			help_long: None,
			usage: "load_config",
		},
		// CommandStruct {
		// 	name: "rec",
		// 	alias: None,
		// 	func: rec_func,
		// 	help: Some("Receive raw"),
		// 	help_long: Some("Receives raw from a port"),
		// 	usage: "rec <port>",
		// },
		// CommandStruct {
		// 	name: "tra",
		// 	alias: None,
		// 	func: tra_func,
		// 	help: Some("Transmit text"),
		// 	help_long: Some("Transmits text using an address:port pair"),
		// 	usage: "tra <addr> <data>",
		// },
		// CommandStruct {
		// 	name: "rec_file",
		// 	alias: None,
		// 	func: rec_file_func,
		// 	help: Some("Receive files"),
		// 	help_long: Some("Receives files from a port"),
		// 	usage: "rec_file <port>",
		// },
		// CommandStruct {
		// 	name: "tra_file",
		// 	alias: None,
		// 	func: tra_file_func,
		// 	help: Some("Transmit file"),
		// 	help_long: Some("Transmits files using an address:port pair"),
		// 	usage: "tra_file <addr> <filepath>",
		// },
	]
}

pub fn get_command(name: &str) -> Result<CommandStruct, &str> {
	for i in get_commands() {
		if i.name == name {
			return Ok(i);
		}
	}
	Err("Invalid Command")
}

// COMMAND FUNCTIONS

pub fn load_config(args: Option<Vec<String>>) -> Result<Option<String>, String> {
	crate::config::load_config(None);
	Ok(None)
}

fn test_func(args: Option<Vec<String>>) -> Result<Option<String>, String> {
	for arg in args.unwrap() {
		if arg == "nuke" {
			return Err("Nuked".to_string());
		}
		print!("[{arg}] ");
	}

	println!();

	Ok(None)
}

fn clear_screen_func(_args: Option<Vec<String>>) -> Result<Option<String>, String> {
	#[cfg(feature = "windows-os")]
	// print!("\x1B[2J\x1B[1;1H");
	// print!("{}[2J", 27 as char);
	println!("Unimplemented feature because of how wack windows is.");

	#[cfg(feature = "linux-os")]
	print!("{esc}c", esc = 27 as char);
	Ok(None)
}

fn exit_func(_args: Option<Vec<String>>) -> Result<Option<String>, String> {
	println!("Exiting.");
	std::process::exit(0)
}

fn help_func(args: Option<Vec<String>>) -> Result<Option<String>, String> {
	if args.is_none() {
		for i in get_commands() {
			println!(
				"{}\t\t{:?}",
				i.name,
				i.help
					.unwrap_or("There is no help available for this command.")
			);
			println!("\t\t{:?}", i.usage);
		}
	} else {
		let name = &args.unwrap()[0];
		if get_command(name).is_ok() {
			println!(
				"{}\n\n{}",
				get_command(name).unwrap().usage,
				get_command(name)
					.unwrap()
					.help_long
					.unwrap_or_else(|| get_command(name)
						.unwrap()
						.help
						.unwrap_or("There is no help available for this command."))
			);
		} else {
			println!("Invalid command: {}", name);
		}
	}

	Ok(None)
}

fn rec_func(args: Option<Vec<String>>) -> Result<Option<String>, String> {
	network::receive_data_func(args.unwrap())
}

fn tra_func(args: Option<Vec<String>>) -> Result<Option<String>, String> {
	network::send_data_func(args.unwrap())
}

fn rec_file_func(args: Option<Vec<String>>) -> Result<Option<String>, String> {
	network::receive_file_func(args.unwrap())
}

fn tra_file_func(args: Option<Vec<String>>) -> Result<Option<String>, String> {
	network::send_file_func(args.unwrap())
}