use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;
use web_server::ThreadPool;

fn main() {
    // listen to connections at tis ip and port
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // create a thread pool to handle multiple request at the same time
    let thread_pool = ThreadPool::new(4);

    // loop through the connections received
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread_pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    // buffer stores the data received in 1024 bytes
    let mut buffer = [0; 1024];

    // populates the buffer with data
    stream.read(&mut buffer).unwrap();

    // home page
    let get = b"GET / HTTP/1.1\r\n";
    let sleep =  b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = 
        if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK", "index.html")
        } else if buffer.starts_with(sleep) {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

    let contents = fs::read_to_string(format!("templates/html/{}", filename)).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    
}
