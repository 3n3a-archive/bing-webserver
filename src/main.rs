// use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::borrow::Cow;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let (http_method, http_path, _http_version) = parse_http_request(&mut buffer);

    log_request(&http_method, &http_path);

    let (status_line, contents) = handle_path(&http_method, &http_path);

    let res = format!("{status_line}<html><body>{contents}</body></html>");
    stream.write_all(res.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn log_request(method: &String, path: &String) {
    println!("{method} {path} \x07", method=method, path=path);
}

fn parse_http_request(buffer: &mut [u8; 1024]) -> (String, String, String) {
    let http_request: Cow<'_, str> = String::from_utf8_lossy(&buffer[..]).to_owned();
    let mut http_request_lines = http_request.lines();

    let http_header = http_request_lines.next().unwrap();

    let mut http_header_parts = http_header.split_ascii_whitespace();
    let http_method = http_header_parts.next().unwrap();
    let http_path = http_header_parts.next().unwrap();
    let http_version = http_header_parts.next().unwrap();

    (http_method.to_owned(), http_path.to_owned(), http_version.to_owned())
}

fn handle_path(http_method: &str, http_path: &str) -> (String, String) {
    let (status_line, contents) = match &http_method {
        &"GET" => handle_get(&http_path),
        &"POST" => handle_get(&http_path),
        // add more methods here
        _ => ("HTTP/1.1 501 Not Implemented\r\n\r\n".to_owned(), "<h1>501 Not Implemented</h1>".to_owned())
    };
    
    (status_line.to_string(), contents.to_string())
}

fn handle_get(http_path: &str) -> (String, String) {
    let (status_line, contents) = match &http_path {
        &"/" => ("HTTP/1.1 200 OK\r\n\r\n", "<h1>Hello</h1>"),
        &"/hello" => ("HTTP/1.1 200 OK\r\n\r\n", "<h1>My lord, how can I help you?</h1>"),
        _ => ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "<h1>404 Not Found</h1>")
    };

    (status_line.to_owned(), contents.to_owned())
}