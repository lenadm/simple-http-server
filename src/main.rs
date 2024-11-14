use std::{
    net::TcpListener,
    io::{Write, BufRead, BufReader},
    fs,
    collections::HashSet,
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
            let mut requested_file = tokens.next().unwrap();
            let valid_locations: HashSet<String> = HashSet::from(["/".to_string(), "/style.css".to_string(), "/script.js".to_string()]);
            if valid_locations.contains(requested_file) {
                if requested_file == "/" {
                    requested_file = "/index.html";
                }
                let file_name = &requested_file[1..requested_file.len()];

                let confirmation = "HTTP/1.1 200 OK";
                let extension = &file_name.split(".").last().unwrap();
                let content = fs::read_to_string(file_name).expect("failed to read file");
                let length = content.len();

                let response = format!("{confirmation}\r\nContent-Type: text/{extension}\r\nContent-Length: {length}\r\n\r\n{content}");

                let _ = stream.write(&response.into_bytes());
            } else {
                let confirmation = "HTTP/1.1 404 NOT FOUND";
                let content = fs::read_to_string("404.html").expect("failed to read file");
                let length = content.len();

                let response = format!("{confirmation}\r\nContent-Type: text/html\r\nContent-Length: {length}\r\n\r\n{content}");

                let _ = stream.write(&response.into_bytes());
            }
        }
    }
}
