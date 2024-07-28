use std::net::{TcpListener,TcpStream};
use std::io::{Read,Write};
use std::{env, fs::File};
use std::path::Path;
use std::str;
use itertools::Itertools;

use std::any::type_name;
#[warn(dead_code)]
fn print_type_of<T>(_: &T) {
    println!("Type: {}", type_name::<T>());
}

fn main() {

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    println!("Server Runnig port 4221");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || {
                    println!("Accepted new connection");
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream:TcpStream){
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    
    // Read and check request
    let request = match str::from_utf8(&buffer) {
        Ok(v) => v,
        Err(e) => {
            println!("Invalid UTF-8 sequence: {}", e);
            return;
        }
    };
    // Request String to Vec
    let lines: Vec<&str> = request.lines().collect_vec();
    let first_line = lines.get(0).unwrap();
    let split_first_line: Vec<&str> = first_line.split(' ').collect_vec();
    let method = split_first_line.get(0).unwrap();
    let path = split_first_line.get(1).unwrap();
    let _http_version = split_first_line.get(2).unwrap();
    
    // Initialize headers and body
    let mut user_agent = None;
    let mut accept_encoding = None;
    let mut content_encoding= None;
    let mut response_body = "";
    let mut status_line = "HTTP/1.1 200 OK\r\n";
    let content_type = "text/plain";
    let mut content_length = 0;

    // Iterate over the headers
    for line in &lines[1..] {
        if line.starts_with("User-Agent:") {
            user_agent = Some(line.trim_start_matches("User-Agent:").trim().to_string());
        }else if line.starts_with("Content-Length:") {
            content_length = line.trim_start_matches("Content-Length:").trim().parse().unwrap();
        } else if line.starts_with("Accept-Encoding:") {
            accept_encoding = Some(line.trim_start_matches("Accept-Encoding:").trim().to_string());
        }    
    }

    // Handle the request based on the path and method
    if method == &"GET" {
        if path == &"/" {
            // status_line = "HTTP/1.1 200 OK\r\n";
            response_body = "";
        } else if path.starts_with("/echo/"){
            let data = path.trim_start_matches("/echo/");
            response_body = data;
            if let Some(encoding) = accept_encoding {
                if encoding == "gzip" {
                    content_encoding = Some("gzip");
                }
            }
        } else if path == &"/user-agent" {
            if let Some(agent) = &user_agent {
                // status_line = "HTTP/1.1 200 OK\r\n";
                response_body = &agent;
            } else {
                status_line = "HTTP/1.1 404 Not Found\r\n";
            }
        } else if path.starts_with("/files"){
            let file_name = path.replace("/files/", "");

            if let Some(dir) = env::args().nth(2) {
                if let Ok(mut file) = File::open(Path::new(&dir).join(file_name)) {
                    let mut buf = Vec::new();
                    file.read_to_end(&mut buf).unwrap();
                    stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n", buf.len()).as_bytes()).unwrap();
                    stream.write_all(buf.as_slice()).unwrap();
                }else{
                    status_line = "HTTP/1.1 404 Not Found\r\n";
                    response_body = &"File not found";
                }
            }
        } else{
            status_line = "HTTP/1.1 404 Not Found\r\n";
        }
        
    } else if method == &"POST" {
        if path.starts_with("/files/") {
            let file_name = path.replace("/files/", "");

            // Find the body of the request
            let body_start = request.find("\r\n\r\n").unwrap() + 4;
            let body = &request[body_start..body_start + content_length];

            if let Some(dir) = env::args().nth(2) {
                let mut new_file = File::create(Path::new(&dir).join(file_name)).expect("Error creating new file");
                new_file.write_all(&body.as_bytes()).expect("Error writing to file");
                status_line = "HTTP/1.1 201 Created\r\n";
                response_body = &body;
            } else {
                status_line = "HTTP/1.1 500 Internal Server Error\r\n";
                response_body = "Directory not specified";
            }
        }
    } else {
        // response = &format!("HTTP/1.1 404 Not Found\r\n\r\n");
        status_line = "HTTP/1.1 404 Not Found\r\n";
        
    }

    let response = if status_line.starts_with("HTTP/1.1 200 OK") {
        let headers = if let Some(encoding) = content_encoding {
            format!(
                "{}Content-Type: {}\r\nContent-Length: {}\r\nContent-Encoding: {}\r\n\r\n",
                status_line,
                content_type,
                response_body.len(),
                encoding
            )
        } else {
            format!(
                "{}Content-Type: {}\r\nContent-Length: {}\r\n\r\n",
                status_line,
                content_type,
                response_body.len()
            )
        };
        format!("{}{}", headers, response_body)
    } 
    else {
        format!("{}Content-Type: text/plain\r\nContent-Length: 0\r\n\r\n", status_line)
    };

    // Send the response
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}
