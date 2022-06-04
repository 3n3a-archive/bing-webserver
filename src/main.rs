// use std::fs;
use async_std::io::prelude::*;
use async_std::net::TcpListener;
use async_std::net::TcpStream;
use futures::stream::StreamExt;
use std::borrow::Cow;
use std::time::Duration;
use async_std::task;

const HTTP_STATUS_200: &str = "HTTP/1.1 200 OK\r\n\r\n";
const HTTP_STATUS_404: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const HTTP_STATUS_501: &str = "HTTP/1.1 501 Not Implemented\r\n\r\n";

#[async_std::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    listener
        .incoming()
        .for_each_concurrent(None, |tcpstream| async move {
            let tcpstream = tcpstream.unwrap();
            task::spawn(handle_connection(tcpstream));
        })
        .await;
}

async fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let (http_method, http_path, _http_version) = parse_http_request(&mut buffer);

    log_request(&http_method, &http_path);

    let (status_line, contents) = handle_path(&http_method, &http_path).await;

    let res = format!("{status_line}<html><body>{contents}</body></html>");
    stream.write_all(res.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
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

async fn handle_path(http_method: &str, http_path: &str) -> (String, String) {
    let (status_line, contents) = match &http_method {
        &"GET" => handle_get(&http_path).await,
        &"POST" => handle_post(&http_path).await,
        // add more methods here
        _ => (HTTP_STATUS_501.to_owned(), "<h1>501 Not Implemented</h1>".to_owned())
    };
    
    (status_line.to_string(), contents.to_string())
}

async fn handle_get(http_path: &str) -> (String, String) {
    let (status_line, contents) = match &http_path {
        &"/" => (HTTP_STATUS_200, "<h1>Hello</h1>"),
        &"/hello" => (HTTP_STATUS_200, "<h1>My lord, how can I help you?</h1>"),
        _ => (HTTP_STATUS_404, "<h1>404 Not Found</h1>")
    };

    (status_line.to_owned(), contents.to_owned())
}

async fn handle_post(http_path: &str) -> (String, String) {
    task::sleep(Duration::from_secs(5)).await;
    let (status_line, contents) = match &http_path {
        &"/" => (HTTP_STATUS_200, "<h1>Hello</h1>"),
        &"/hello" => (HTTP_STATUS_200, "<h1>My lord, how can I help you?</h1>"),
        _ => (HTTP_STATUS_404, "<h1>404 Not Found</h1>")
    };

    (status_line.to_owned(), contents.to_owned())
}