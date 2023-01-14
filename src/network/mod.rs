use std::{io::*, net::*, result::Result};

struct NetworkPacketType {
	begin: u8,
	end: u8,
	next: u8,
	version: u8,

	gp_packet: u8,
}

const NETWORK_PACKET_TYPE: NetworkPacketType = NetworkPacketType {
	begin: 1,
	end: 3,
	next: 5,
	version: 12,
	
	gp_packet: 14,
};

fn send_data_func(args: Option<Vec<String>>) -> Result<Option<String>, String> {
	let mut stream = TcpStream::connect("127.0.0.1:25565").unwrap();

	match stream.write_fmt(format_args!("{:?}", args.unwrap())) {
		Ok(_) => (),
		Err(e) => println!("Err: {}", e),
	}

	stream.shutdown(std::net::Shutdown::Both).ok();

	Ok(None)
}

fn receive_data_func(_args: Option<Vec<String>>) -> Result<Option<String>, String> {
	let loopback = Ipv4Addr::new(0, 0, 0, 0);
	let socket = SocketAddrV4::new(loopback, 25565);
	let listener = TcpListener::bind(socket).unwrap();
	println!(
		"Listening on {}, access this port to end the program",
		listener.local_addr().unwrap()
	);
	let (mut tcp_stream, addr) = listener.accept().unwrap();
	println!("Connection received! {:?} is sending data.", addr);
	let mut input = String::new();
	let _ = tcp_stream.read_to_string(&mut input);
	println!("{:?} \"{}\"", addr, input.trim());

	Ok(None)
}
