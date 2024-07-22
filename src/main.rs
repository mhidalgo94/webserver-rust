use std::net::{TcpListener,TcpStream};
use std::io::{Read,Write};
use std::{env, fs::File};
use std::path::Path;
use std::str;
use itertools::Itertools;



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
    let mut response_body = "";
    let mut status_line = "HTTP/1.1 200 OK\r\n";
    let content_type = "text/plain";

    // Iterate over the headers
    for line in &lines[1..] {
        if line.starts_with("User-Agent:") {
            user_agent = Some(line.trim_start_matches("User-Agent:").trim().to_string());
        }
    }

    // Handle the request based on the path and method
    if method != &"GET" {
        // response = &format!("HTTP/1.1 404 Not Found\r\n\r\n");
        status_line = "HTTP/1.1 404 Not Found\r\n";

    } else {
        if path == &"/" {
            // status_line = "HTTP/1.1 200 OK\r\n";
            response_body = "";
        } else if path.starts_with("/echo/"){
            let data = path.trim_start_matches("/echo/");
            // status_line = "HTTP/1.1 200 OK\r\n";
            response_body = data;
            // let _ = stream.write(format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",data.len(),data).as_bytes());
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
                    // content_type = "application/octet-stream";
                    // response_body = buf.as_slice().;
                    stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n", buf.len()).as_bytes()).unwrap();
                    stream.write_all(buf.as_slice()).unwrap();
                }else{
                    status_line = "HTTP/1.1 404 Not Found\r\n";
                    response_body = &"File not found";
                }
            }
            // let env_args: Vec<String> = env::args().collect();
            // let mut dir = env_args[2].clone();
            // dir.push_str(&file_name);
            // let file: Result<Vec<u8>, std::io::Error> = fs::read(dir);
            // // println!("{}", String::from_utf8(file.unwrap()).expect("file content"));
            // match file{
            //     Ok(mut fc) => {

            //         let mut buf_file = Vec::new();
            //         fc.read(&mut buf_file).unwrap();
            //         content_type = "application/octet-stream";
            //         response_body = &String::from_utf8(&fc.).expect("file content");
                    
            //     }
            //     Err(_) => {
            //         status_line = "HTTP/1.1 404 Not Found\r\n";
            //         response_body = &"File not found";
            //     }
            // }
        } else{
            // response = &format!("HTTP/1.1 404 Not Found\r\n\r\n");
            // let _ = stream.write(b"HTTP/1.1 404 Not Found\r\n\r\n");
            status_line = "HTTP/1.1 404 Not Found\r\n";
        }
    }

    let response = if status_line.starts_with("HTTP/1.1 200 OK") {
        let headers = format!(
            "{}Content-Type: {}\r\nContent-Length: {}\r\n\r\n",
            status_line,
            content_type,
            response_body.len()
        );
        println!("{}", headers);
        format!("{}{}", headers, response_body)
    } else {
        format!("{}Content-Type: text/plain\r\nContent-Length: 0\r\n\r\n", status_line)
    };

    
    // Send the response
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}
