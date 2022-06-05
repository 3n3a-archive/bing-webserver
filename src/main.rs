use async_std::fs;
use async_std::io::prelude::*;
use async_std::net::TcpListener;
use async_std::net::TcpStream;
use async_std::task;
use futures::stream::StreamExt;
use markdown;
use std::borrow::Cow;
use std::path::Path;

const HTTP_STATUS_200: &str = "HTTP/1.1 200 OK\n";
const HTTP_STATUS_404: &str = "HTTP/1.1 404 NOT FOUND\n";
const HTTP_STATUS_501: &str = "HTTP/1.1 501 Not Implemented\n";
const STATIC_FILE_PATH: &str = ".";

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

    let (status_line, mut contents) = handle_path(&http_method, &http_path).await;

    if &http_method == "HEAD" {
        // omit body
        contents = "".to_owned();
    } else {
        contents = format!("{contents}");
    }

    let headers = "Server: EWS";
    let res = format!("{status_line}{headers}\r\n\r\n{contents}");
    stream.write_all(res.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}

fn log_request(method: &String, path: &String) {
    println!("{method} {path} \x07", method = method, path = path);
}

fn parse_http_request(buffer: &mut [u8; 1024]) -> (String, String, String) {
    let http_request: Cow<'_, str> = String::from_utf8_lossy(&buffer[..]).to_owned();
    let mut http_request_lines = http_request.lines();

    let http_header = http_request_lines.next().unwrap();

    let mut http_header_parts = http_header.split_ascii_whitespace();
    let http_method = http_header_parts.next().unwrap();
    let http_path = http_header_parts.next().unwrap();
    let http_version = http_header_parts.next().unwrap();

    (
        http_method.to_owned(),
        http_path.to_owned(),
        http_version.to_owned(),
    )
}

async fn handle_path(http_method: &str, http_path: &str) -> (String, String) {
    let (status_line, contents): (String, String) = match &http_method {
        &"GET" => handle_get(&http_path).await,
        &"POST" => handle_post(&http_path).await,
        &"HEAD" => handle_get(&http_path).await,
        _ => (
            String::from(HTTP_STATUS_501),
            String::from("<h1>501 Not Implemented</h1>"),
        ),
    };
    (status_line, contents)
}

async fn handle_get(http_path: &str) -> (String, String) {
    let (status_line, contents): (String, String) = match &http_path {
        &"/" => (
            String::from(HTTP_STATUS_200),
            String::from("<h1>Hello</h1>"),
        ),
        &"/hello" => (
            String::from(HTTP_STATUS_200),
            String::from("<h1>My lord, how can I help you?</h1>"),
        ),
        _ => handle_static_files(&http_path).await,
    };

    (status_line, contents)
}

async fn handle_post(http_path: &str) -> (String, String) {
    let (status_line, contents): (String, String) = match &http_path {
        &"/" => (
            String::from(HTTP_STATUS_200),
            String::from("<h1>Hello</h1>"),
        ),
        &"/hello" => (
            String::from(HTTP_STATUS_200),
            String::from("<h1>My lord, how can I help you?</h1>"),
        ),
        _ => (
            String::from(HTTP_STATUS_404),
            String::from("<h1>404 Not Found</h1>"),
        ),
    };

    (status_line, contents)
}

async fn handle_static_files(http_path: &str) -> (String, String) {
    let split_http_path: Vec<_> = http_path.split("/").collect();
    let filename: &str = split_http_path[1]; // [0] is empty string
    let file_extension: &str = {
        let split_filename: Vec<_> = filename.split(".").collect();
        split_filename.last().unwrap()
    };
    println!("{}", file_extension);

    let static_file_path = Path::new(STATIC_FILE_PATH);
    let requested_file = static_file_path.join(filename);

    let mut file_found: bool = true;
    let mut status_line: String = String::from("");
    let mut contents: String = String::from("");

    if is_file_of_type(&requested_file, ".html") {
        status_line = String::from(HTTP_STATUS_200);
        contents = fs::read_to_string(&requested_file).await.unwrap();
    } else if is_file_of_type(&requested_file, ".md") {
        (status_line, contents) = handle_markdown_files(&requested_file)
    } else {
        file_found = false;
    }


    if !file_found {
        status_line = String::from(HTTP_STATUS_404);
        contents = String::from("<h1>404 Not Found</h1>");
    }

    (status_line, contents)
}

fn is_file_of_type(file: &Path, filetype: &str) -> bool {
    file.is_file() && file.to_str().unwrap().ends_with(filetype)
}

fn handle_markdown_files(file: &Path) -> (String, String) {
    let status_line: String =
        String::from(HTTP_STATUS_200) + "Content-Type: text/html; charset=UTF-8\n";
    let contents: String = markdown::file_to_html(&file).unwrap();
    (status_line, contents)
}
