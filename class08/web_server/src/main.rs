// IKinder

use std::io::{BufRead, Write};

fn main() {
	let listener = std::net::TcpListener::bind("localhost:7878").unwrap();
	for stream in listener.incoming() {
		let stream = stream.unwrap();
		println!("Connection established!");
		handle_connection(stream);
	}
}

fn handle_connection(mut stream: std::net::TcpStream) {
	let buf_reader = std::io::BufReader::new(&mut stream);
	let request_line = buf_reader
		.lines()
		.next()
		.unwrap()
		.unwrap();

	let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
		("HTTP/1.1 200 OK", "hello.html")
	} else {
		("HTTP/1.1 404 NOT FOUND", "404.html")
	};

	let contents = std::fs::read_to_string(format!("html/{filename}")).unwrap();
	let length = contents.len();

	let response = format!("{status_line}\r\nContent-Type: text/html\r\nContent-Length: {length}\r\n\r\n{contents}");
	stream
		.write_all(response.as_bytes())
		.unwrap();
}
