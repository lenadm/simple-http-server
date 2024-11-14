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
            let file = tokens.next().unwrap();
            println!("{}", file);
            match file{
                "/" => {
                    let confirmation = "HTTP/1.1 200 OK";
                    let content = fs::read_to_string("index.html").expect("failed to read file");
                    let length = content.len();

                    let response = format!("{confirmation}\r\nContent-Type: text/html\r\nContent-Length: {length}\r\n\r\n{content}");
                    let _ = stream.write(&response.into_bytes());
                }
                "/style.css" => {
                    let confirmation = "HTTP/1.1 200 OK";
                    let content = fs::read_to_string("style.css").expect("failed to read file");
                    let length = content.len();

                    let response = format!("{confirmation}\r\nContent-Type: text/css\r\nContent-Length: {length}\r\n\r\n{content}");
                    let _ = stream.write(&response.into_bytes());
                }
                "/script.js" => {
                    let confirmation = "HTTP/1.1 200 OK";
                    let content = fs::read_to_string("script.js").expect("failed to read file");
                    let length = content.len();

                    let response = format!("{confirmation}\r\nContent-Type: text/js\r\nContent-Length: {length}\r\n\r\n{content}");
                    let _ = stream.write(&response.into_bytes());
                }
                _ => {
                    let error = "HTTP/1.1 404 NOT FOUND";
                    let content = fs::read_to_string("404.html").expect("failed to read file");
                    let response = format!("{error}\r\nContent-Type: text/html\r\nContent-Length: {0}\r\n\r\n{content}", content.len());
                    let _ = stream.write(&response.into_bytes());
                }
            }
        }
    }
}
