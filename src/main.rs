use async_std::fs;
use async_std::io::prelude::*;
use async_std::net::TcpListener;
use async_std::net::TcpStream;
use async_std::task;
use futures::stream::StreamExt;
use markdown;
use mime_guess;
use std::borrow::Cow;
use std::path::{Path, PathBuf};

const HTTP_STATUS_200: &str = "HTTP/1.1 200 OK\n";
const HTTP_STATUS_404: &str = "HTTP/1.1 404 NOT FOUND\n";
const HTTP_STATUS_501: &str = "HTTP/1.1 501 Not Implemented\n";
const SERVE_STATIC_FILES: bool = true;
const STATIC_FILE_PATH: &str = ".";
const ALLOWED_STATIC_FILE_EXTENSIONS: &'static [&str] = &[
    "html", "md", "css", "js", "jpg", "jpeg", "webp", "png", "avif",
];
const RING_BELL_ON_REQUEST: bool = false;

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
    }

    let headers = "Server: EWS";
    let res = format!("{status_line}{headers}\r\n\r\n{contents}");
    stream.write_all(res.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}

fn log_request(method: &String, path: &String) {
    let bell: &str = if RING_BELL_ON_REQUEST { "\x07" } else { "" };
    println!("{method} {path} {bell}", method = method, path = path);
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
        _ => {
            async {
                let (status_line, contents): (String, String);

                let (file_found, requested_file, file_extension) = get_static_file_info(&http_path);
                if SERVE_STATIC_FILES && file_found {
                    (status_line, contents) =
                        handle_static_files(&requested_file, file_extension).await;
                } else {
                    status_line = String::from(HTTP_STATUS_404);
                    contents = String::from("<h1>404 Not Found</h1>");
                }

                (status_line, contents)
            }
            .await
        }
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

async fn handle_static_files(requested_file: &PathBuf, file_extension: &str) -> (String, String) {
    let not_found = (
        String::from(HTTP_STATUS_404),
        String::from("<h1>404 Not Found</h1>"),
    );
    let (status_line, contents): (String, String) = if ALLOWED_STATIC_FILE_EXTENSIONS
        .to_vec()
        .contains(&file_extension)
    {
        match &file_extension {
            &"md" => handle_markdown_files(&requested_file),
            &"html" => handle_fs_files(&requested_file).await,
            _ => handle_fs_files(&requested_file).await,
        }
    } else {
        not_found
    };

    (status_line, contents)
}

fn get_static_file_info(http_path: &str) -> (bool, PathBuf, &str) {
    let split_http_path: Vec<_> = http_path.split("/").collect();
    let filename: &str = split_http_path[1]; // [0] is empty string
    let file_extension: &str = {
        let split_filename: Vec<_> = filename.split(".").collect();
        split_filename.last().unwrap()
    };

    let static_file_path = Path::new(STATIC_FILE_PATH);
    let requested_file = static_file_path.join(filename);
    let file_exists = &requested_file.is_file();

    (*file_exists, requested_file, file_extension)
}

async fn handle_fs_files(file: &Path) -> (String, String) {
    let content_type: &str = mime_guess::from_path(&file).first_raw().unwrap();
    let content_type_string: &str = &*format!("Content-Type: {content_type}; charset=UTF-8\n");
    let status_line: String = String::from(HTTP_STATUS_200) + content_type_string;
    let contents: String = fs::read_to_string(&file).await.unwrap();
    (status_line, contents)
}

fn handle_markdown_files(file: &Path) -> (String, String) {
    let status_line: String =
        String::from(HTTP_STATUS_200) + "Content-Type: text/html; charset=UTF-8\n";
    let contents: String = markdown::file_to_html(&file).unwrap();
    (status_line, contents)
}
