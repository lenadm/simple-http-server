use std::{
    net::TcpListener,
    io::{Write, BufRead, BufReader},
    fs
};

fn main() {
    let tcp = TcpListener::bind("127.0.0.1:4221");

    for stream in tcp.expect("invalid input").incoming() {
        let mut stream = stream.unwrap();
        let mut buf_reader = BufReader::new(&stream);

        let mut line = String::new();
        let _ = buf_reader.read_line(&mut line);
        
        let mut tokens = line.split_whitespace();
        if tokens.next().unwrap() == "GET" {
            let confirmation = "HTTP/1.1 200 OK";
            let content = fs::read_to_string("index.html").expect("failed to read file");
            let length = content.len();

            let response = format!("{confirmation}\r\nContent-Length: {length}\r\n\r\n{content}");
            let _ = stream.write(&response.into_bytes());
        }
    }
}
