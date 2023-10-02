use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established.");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let (file, status) = if buffer.starts_with(get) {
        ("index.html", "HTTP/1.1 200 OK")
    } else {
        ("404.html", "HTTP/1.1 404 NOT FOUND")
    };
    let content = fs::read_to_string(file).unwrap();

    let response = format! {"{}\r\nContent-Length: {}\r\n\r\n{}", status, content.len(), content};
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
