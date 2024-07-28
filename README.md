# Learning Project in Rust

This project implements a basic HTTP server in Rust that handles GET and POST requests. The main purpose of this project is to learn and understand fundamental Rust programming concepts, including handling `TcpStream`s, processing HTTP requests, and compressing responses using gzip.


## Project Description

The HTTP server implemented in this project listens on port `4221` and handles various routes, including:

1. **GET /echo/{str}**
   - Responds with the `{str}` text received in the URL.
   - If the request contains the header `Accept-Encoding: gzip`, the response is compressed using gzip and the `Content-Encoding: gzip` header is added.

2. **GET /user-agent**
   - Responds with the value of the `User-Agent` header from the request.

3. **GET /files/{filename}**
   - Searches for and returns the file specified by `{filename}` in the directory provided as an argument when starting the server.
   - The response contains the file as an octet stream and the `Content-Type: application/octet-stream` header.

4. **POST /files/{filename}**
   - Creates a new file with the name `{filename}` in the specified directory.
   - The file content is the body of the request.

## Project Usage

### Prerequisites

- Rust (installed via `rustup`)
- Cargo (Rust package manager)

### Compilation and Execution

To compile and run the server:

```
cargo build
./run.sh --directory /tmp/
```
## Example Requests
 
### GET /echo/{str}
```
curl -v -H "Accept-Encoding: gzip" http://localhost:4221/echo/abc
```

### GET /user-agent
```
curl -v -H "User-Agent: MyTestAgent" http://localhost:4221/user-agent
```
### GET /files/{filename}
``` 
 curl -v http://localhost:4221/files/filename.txt
```
### POST /files/{filename}
 ```
curl -v --data "File content" -H "Content-Type: application/octet-stream" http://localhost:4221/files/new_file.txt
```

## Example Compressed Request
If the request contains Accept-Encoding: gzip:

```bash
curl -v -H "Accept-Encoding: gzip" http://localhost:4221/echo/abc
```
The server responds with:
- `200 OK`
- *Content-Type:* text/plain
- *Content-Encoding:* gzip
- *Content-Length:* [compressed body size]
- Response body compressed in gzip.