// #![warn(clippy::pedantic)] // funny but no

// use self::commands::CommandStruct;

mod command_index;

pub fn init_loop() {
	println!("Use `help` for more information.");

	loop {
		use std::io;

		print!("cabin~> ");
		io::Write::flush(&mut io::stdout()).expect("Couldnt flush stdout");
		let mut buf = String::new();
		io::stdin()
			.read_line(&mut buf)
			.expect("Couldnt read buffer");

		match run_commands(&buf) {
			Ok(_) => (),
			Err(x) => println!("Error: {x}"),
		}
	}
}

pub fn run_commands(buf: &str) -> Result<Option<String>, String> {
	// let args = shellwords::split(&buf).expect("Invalid quotes");

	let args_multiple = &super_duper_ultra_spliterator(buf);

	for args in args_multiple {
		// if args.is_empty() {
		// 	return Err("Empty".to_owned());
		// }
		let new_args = {
			if args.split_at(1).1.to_vec().is_empty() {
				None
			} else {
				Some(args.split_at(1).1.to_vec())
			}
		};

		match run_command(&args[0], new_args) {
			Ok(_) => (),
			Err(err) => return Err(err),
		}
	}

	Ok(None)
}

pub fn run_command(name: &str, args: Option<Vec<String>>) -> Result<Option<String>, String> {
	let result = command_index::get_command(name);

	match result {
		Ok(_) => result.ok().unwrap().func.call((args,)),
		Err(e) => Err(e.to_owned()),
	}
}

fn super_duper_ultra_spliterator(buf: &str) -> Vec<Vec<String>> {
	let mut result_multiple: Vec<Vec<String>> = Vec::new();

	let mut result_single = Vec::new();
	let mut result_single_buffer: String = String::new();

	let delimiter_char = ';';
	let escape_char = '\\';
	let quote_char = '"'; // TODO implement quote differentiation
	let split_chars = vec![' ', '\n'];
	let mut currently_escaped = false;
	let mut currently_stringed = false;

	for c in buf.trim().chars() {
		if currently_escaped {
			let _ = &result_single_buffer.push_str(&c.to_string());
			currently_escaped = false;
			continue;
		}

		if c == escape_char {
			currently_escaped = !currently_escaped;
			continue;
		} else if c == quote_char && !currently_escaped {
			currently_stringed = !currently_stringed;
			continue;
		} else if c == delimiter_char && !currently_escaped {
			if !result_single_buffer.is_empty() {
				result_single.push(result_single_buffer.clone());
				result_single_buffer.clear();
			}
			if !result_single.is_empty() {
				result_multiple.push(result_single.clone());
				result_single.clear();
			}
			continue;
		}

		if currently_stringed {
			let _ = &result_single_buffer.push_str(&c.to_string());
			continue;
		}

		if split_chars.contains(&c) {
			if !result_single_buffer.is_empty() {
				result_single.push(result_single_buffer.trim().to_string());
				result_single_buffer.clear();
			}
		} else {
			// if !c.is_whitespace() {
			let _ = &result_single_buffer.push(c);
		}
	}

	result_single.push(result_single_buffer.clone());
	result_multiple.push(result_single.clone());

	// dbg!(&result_multiple);

	result_multiple
}
