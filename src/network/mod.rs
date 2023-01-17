use std::{io::*, net::*, result::Result, thread};

use self::net_io::handle_sender;

type Ret = Result<Option<String>, String>;

pub fn send_data_func(args: Vec<String>) -> Result<Option<String>, String> {
	let mut stream = TcpStream::connect(&args[0]).unwrap();

	stream.shutdown(std::net::Shutdown::Both).ok();

	Ok(None)
}

pub fn receive_data_func(args: Vec<String>) -> Result<Option<String>, String> {
	let loopback = Ipv4Addr::new(0, 0, 0, 0);
	let socket = SocketAddrV4::new(loopback, args[0].parse::<u16>().unwrap());
	let listener = TcpListener::bind(socket).unwrap();
	println!(
		"Listening on {}, access this port to end the program",
		listener.local_addr().unwrap()
	);
	let (mut tcp_stream, addr) = listener.accept().unwrap();
	println!("Connection received! {addr:?} is sending data.");
	let mut input = String::new();
	let _ = tcp_stream.read_to_string(&mut input);
	println!("{:?} \"{}\"", addr, input.trim());

	Ok(None)
}

pub fn send_file_func(args: Vec<String>) -> Result<Option<String>, String> {
	let stream = TcpStream::connect(&args[0]).unwrap();

	match net_io::push_connection(&stream) {
		Ok(_) => {
			stream.shutdown(std::net::Shutdown::Both).ok();
			Ok(None)
		}
		Err(e) => Err(e),
	}
}

pub fn receive_file_func(args: Vec<String>) -> Ret {
	let loopback = Ipv4Addr::new(0, 0, 0, 0);
	let socket = SocketAddrV4::new(loopback, args[0].parse::<u16>().unwrap());

	match net_io::start_receiving_server(std::net::SocketAddr::V4(socket)) {
		Ok(_) => Ok(None),
		Err(e) => Err(e),
	}
}

mod net_io {
	use std::{
		io::{self, BufRead, BufReader, Read, Write},
		net::{SocketAddr, TcpListener, TcpStream},
		thread, time,
	};

	const LENGTH: u16 = 2;
	const VERSION: u8 = 12;

	const GP_PACKET: u8 = 14;
	const SELF_ID: u8 = 19;
	const REQUEST_CONNECTION: u8 = 20;
	const ACCEPT_CONNECTION: u8 = 21;

	pub fn start_receiving_server(socket: SocketAddr) -> super::Ret {
		// Enable port 7878 binding
		let receiver_listener = TcpListener::bind(socket).expect("Failed and bind with the sender");
		// Getting a handle of the underlying thread.
		let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
		// listen to incoming connections messages and bind them to a sever socket address.
		for stream in receiver_listener.incoming() {
			let stream = stream.expect("failed");
			// let the receiver connect with the sender
			let handle = thread::spawn(move || {
				//receiver failed to read from the stream
				handle_sender(stream).unwrap_or_else(|error| eprintln!("{:?}", error))
			});

			// Push messages in the order they are sent
			thread_vec.push(handle);
		}

		for handle in thread_vec {
			// return each single value Output contained in the heap
			handle.join().unwrap();
		}

		Ok(None)
	}

	pub fn push_connection(mut stream: &TcpStream) -> super::Ret {
		use std::io::{self, prelude::*, BufReader, Write};
		use std::net::TcpStream;
		use std::str;

		// connect
		// Struct used to start requests to the server.
		// Check TcpStream Connection to the server
		// let mut stream = TcpStream::connect("127.0.0.1:7878");
		for _ in 0..1000 {
			// Allow sender to enter message input
			let mut input = String::new();
			// First access the input message and read it
			io::stdin().read_line(&mut input).expect("Failed to read");
			// Write the message so that the receiver can access it
			Write::write(&mut stream, input.as_bytes()).expect("failed to write");
			// Add buffering so that the receiver can read messages from the stream
			let mut reader = BufReader::new(stream);
			// Check if this input message values are u8
			let mut buffer: Vec<u8> = Vec::new();
			// Read input information
			reader.read_until(b'\n', &mut buffer);

			println!(
				"read from server:{}",
				str::from_utf8(&buffer).unwrap().trim()
			);
		}
		Ok(None)

		// let to_write = std::time::SystemTime::UNIX_EPOCH
		// 	.elapsed()
		// 	.unwrap()
		// 	.as_micros();

		// // Write::write(stream, &[0, 0, 0, 0, 0]);

		// match Write::write(&mut stream, format!("{to_write}").as_bytes()) {
		// 	Ok(_) => Ok(None),
		// 	Err(_) => Err("Failed to write data.".to_string()),
		// }
	}

	pub fn handle_sender(mut stream: TcpStream) -> io::Result<()> {
		// Handle multiple access stream
		let mut buf = [0; 512];
		for _ in 0..1000 {
			// let the receiver get a message from a sender
			let bytes_read = stream.read(&mut buf)?;
			// sender stream in a mutable variable
			if bytes_read == 0 {
				return Ok(());
			}
			stream.write(&buf[..bytes_read]);
			// Print acceptance message
			//read, print the message sent
			println!("from the sender:{}", String::from_utf8_lossy(&buf));

			// And you can sleep this connection with the connected sender
			thread::sleep(time::Duration::from_millis(1));
		}
		// success value
		Ok(())
	}
}
