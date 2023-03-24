use core::result::Result;

type Ret = Result<Option<String>, String>;

pub fn send_data_func(args: Vec<String>) -> Result<Option<String>, String> {
	// let mut stream = TcpStream::connect(&args[0]).unwrap();
	
	// for arg in args {
	// 	match stream.write(arg.as_bytes()) {
	// 		Ok(_) => (),
	// 		Err(_) => return Err("Could not write raw string to outgoing tcp stream.".to_string()),
	// 	};
	// }

	// stream.shutdown(std::net::Shutdown::Both).ok();

	Ok(Some("Networking disabled for now.".to_string()))
}

pub fn receive_data_func(args: Vec<String>) -> Result<Option<String>, String> {
	// let loopback = Ipv4Addr::new(0, 0, 0, 0);
	// let socket = SocketAddrV4::new(loopback, args[0].parse::<u16>().unwrap());
	// let listener = TcpListener::bind(socket).unwrap();
	// println!("Listening on [{}].", listener.local_addr().unwrap());
	// let (mut tcp_stream, addr) = listener.accept().unwrap();
	// println!("Connection received! {addr:?} is sending data.");
	// let mut input = String::new();
	// let _ = tcp_stream.read_to_string(&mut input);
	// println!("{:?} \"{}\"", addr, input.trim());
	
	Ok(Some("Networking disabled for now.".to_string()))
}

pub fn send_file_func(args: Vec<String>) -> Result<Option<String>, String> {
	// match net_io::send_message() {
	// 	Ok(_) => Ok(None),
	// 	Err(e) => Err(e),
	// }
	Ok(Some("Networking disabled for now.".to_string()))
}

pub fn receive_file_func(args: Vec<String>) -> Ret {
	// match net_io::init_server() {
	// 	Ok(_) => Ok(None),
	// 	Err(e) => Err(e),
	// }
	Ok(Some("Networking disabled for now.".to_string()))
}

// mod net_io {

// 	use borsh::{BorshDeserialize, BorshSerialize};
// 	use RejectionReason::*;

// 	use version_0_packet::*;

// 	#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
// 	struct Packet {
// 		self_id: String,
// 		purpose: ConnectionPurpose,
// 	}

// 	const VERSION: usize = 0;
// 	const SIZE: (usize, usize) = (1, 2);

// 	mod version_0_packet {
// 		use borsh::{BorshDeserialize, BorshSerialize};

// 		#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
// 		pub enum ConnectionPurpose {
// 			RequestConnection,
// 			RejectRequest,
// 			AcceptRequest,
// 		}
// 	}

// 	pub fn init_server() -> super::Ret {
// 		let socket = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), crate::config::get_port());

// 		let receiver_listener = TcpListener::bind(socket).expect("Failed and bind with the sender");

// 		let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();

// 		for stream in receiver_listener.incoming() {
// 			let stream = stream.expect("failed");

// 			let handle = thread::spawn(move || {
// 				match handle_client(stream) {
// 					Ok(_) => (),
// 					Err(e) => eprintln!("Error: {e}"),
// 				};
// 			});

// 			// Push messages in the order they are sent
// 			thread_vec.push(handle);
// 		}

// 		for handle in thread_vec {
// 			handle.join().unwrap();
// 		}
// 		Ok(None)
// 	}

// 	fn start_packet_type_connection() -> Result<TcpStream, String> {
// 		// TcpStream::connect_timeout();

// 		Err("Could not create TcpStream".to_string())
// 	}

// 	pub fn send_message() -> super::Ret {
// 		let socket = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), crate::config::get_port());

// 		let mut stream = match TcpStream::connect(socket) {
// 			Ok(e) => e,
// 			Err(_) => return Err("Could not get outgoing stream.".to_string()),
// 		};
// 		{
// 			// create scope for easier management
// 			let packet = Packet {
// 				self_id: "developer-self-test-id".to_owned(),
// 				purpose: ConnectionPurpose::RequestConnection,
// 			};

// 			let mut request: Vec<u8> = Vec::new();
// 			let encoded_packet = packet.try_to_vec().unwrap();

// 			request.resize(20, 0);

// 			request[VERSION] = 1;
// 			request[SIZE.0] = encoded_packet.len() as u8;
// 			request[SIZE.1] = (encoded_packet.len() >> 8) as u8;

// 			for b in encoded_packet {
// 				// add bytes from encoded_packet to request
// 				request.push(b);
// 			}

// 			Write::write(&mut stream, &request).expect("Failed to write to message stream.");
// 		}

// 		let mut reader = BufReader::new(&stream);
// 		let mut buffer: Vec<u8> = Vec::new();

// 		Ok(None)
// 	}

// 	pub fn handle_client(mut stream: TcpStream) -> super::Ret {
// 		let mut packet_header = [0; 20];
// 		let header_bytes_read = stream.read(&mut packet_header).unwrap();
// 		if header_bytes_read == 0 {
// 			return reject_connection(stream, InvalidRequest, None);
// 		}

// 		// print!("Header: ");
// 		// for b in packet_header {
// 		// 	print!("[{b}]");
// 		// }
// 		// println!();

// 		let request_buffer_size =
// 			((packet_header[SIZE.1] as u16) << 8) | packet_header[SIZE.0] as u16;
// 		if request_buffer_size > 512 {
// 			return reject_connection(
// 				stream,
// 				BufferSizeLimited,
// 				Some("Buffer size not allowed above 512 bytes in this context."),
// 			);
// 		}
// 		let mut request_buffer = vec![0; request_buffer_size.into()];
// 		let request_bytes_read = stream.read(&mut request_buffer).unwrap();
// 		if request_bytes_read == 0 {
// 			return reject_connection(
// 				stream,
// 				DataEmpty,
// 				Some("Length of data == 0 (packet sent is just a header)"),
// 			);
// 		}

// 		// print!("Data: ");
// 		// for b in &request_buffer {
// 		// 	print!("[{b}]");
// 		// }
// 		// println!();

// 		let decoded_packet = Packet::try_from_slice(&request_buffer).unwrap();

// 		match decoded_packet.purpose {
// 			ConnectionPurpose::RequestConnection => process_connection_request(),
// 			_ => {
// 				reject_connection(stream, InvalidRequest, None);
// 				return Err("Invalid request.".to_owned());
// 			}
// 		};

// 		match stream.write(&packet_header[..header_bytes_read]) {
// 			Ok(_) => (),
// 			Err(e) => println!("{e}"),
// 		};

// 		Ok(None)
// 	}

// 	fn process_connection_request() {}

// 	#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
// 	enum RejectionReason {
// 		InvalidRequest,
// 		DataEmpty,
// 		BufferSizeLimited,
// 	}

// 	fn reject_connection(
// 		stream: TcpStream,
// 		reason: RejectionReason,
// 		reason2: Option<&str>,
// 	) -> super::Ret {
// 		Ok(None)
// 	}

// 	#[derive()]
// 	enum Error {}
// }
