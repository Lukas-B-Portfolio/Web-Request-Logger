use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
};
//use std::collections::HashMap;
use chrono::prelude::*;
use std::fs::{File, OpenOptions};
use std::io::BufReader;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let mut log = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .expect("Could not open File.");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        log_request(&stream, &mut log);
        response(stream);
    }
}

/// Always sends 200 OK response
fn response(mut stream: TcpStream) {
    let status_line = "HTTP/1.1 200 OK";
    let response = format!("{status_line}\r\n");

    stream.write_all(response.as_bytes()).unwrap();
}

/// Writes Time and Ip to log file as well as a List of all lines in the stream
fn log_request(mut stream: &TcpStream, log: &mut File) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let client_ip = stream.local_addr().unwrap().to_string();

    log.write("\n-------------------------------------------\n".as_bytes())
        .expect("Failed to write to log.");
    log.write("Received Request at:\t".as_bytes())
        .expect("Failed to write to log.");
    log.write(timestamp().as_bytes())
        .expect("Failed to write Time to log.");
    log.write("\nFrom IP:\t\t\t\t".as_bytes())
        .expect("Failed to write to log.");
    log.write(client_ip.as_bytes())
        .expect("Failed to write IP to log.");
    log.write("\nRequest:\n".as_bytes())
        .expect("Failed to write to log.");

    for s in &http_request {
        log.write(s.as_bytes())
            .expect("Failed to write part of the request to log.");
        log.write("\n".as_bytes()).expect("Failed to write to log.");
    }
    println!("Request: {:#?}", http_request);
}

/// Returns current (local) timestamp as String formatted as: Day/Month/Year Hour:Minute:Second
fn timestamp() -> String {
    format!("{}", Local::now().format("%d/%m/%Y %H:%M:%S"))
}

/*
struct HttpRequest {
    start_line : StartLine,
    headers : HashMap<String, String>,
    // body
}

struct StartLine {
    verb : Verb,
    target : Target,
    version : HTTPVersion
}

enum Verb {
    GET, HEAD, POST, PUT, DELETE, CONNECT, OPTIONS, TRACE, PATCH
}
enum Target {
    AbsolutePath,
    CompleteUrl,
    AuthorityComponent,
    AsteriskForm
}
enum HTTPVersion {
    HTTP1_0,
    HTTP1_1
}
*/
