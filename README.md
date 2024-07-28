# Learning Project in Rust

This project implements a basic HTTP server in Rust that handles GET and POST requests. The main purpose of this project is to learn and understand fundamental Rust programming concepts, including handling `TcpStream`s, processing HTTP requests, and compressing responses using gzip.


## Project Description

The HTTP server implemented in this project listens on port `4221` and handles various routes, including:

1. **GET /echo/{str}**
   - Responds with the `{str}` text received in the URL.
   - If the request contains the header `Accept-Encoding: gzip`, the response is compressed using gzip and the `Content-Encoding: gzip` header is added.
