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
