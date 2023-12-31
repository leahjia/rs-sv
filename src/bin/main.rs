use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        println!("Connection established.");

        pool.execute(|| {
            handle_connection(stream);
        });

        println!("Shutting down.");
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (file, status) = if buffer.starts_with(get) {
        ("index.html", "HTTP/1.1 200 OK")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("index.html", "HTTP/1.1 200 OK")
    } else {
        ("404.html", "HTTP/1.1 404 NOT FOUND")
    };
    let content = fs::read_to_string(file).unwrap();

    let response = format! {"{}\r\nContent-Length: {}\r\n\r\n{}", status, content.len(), content};
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
