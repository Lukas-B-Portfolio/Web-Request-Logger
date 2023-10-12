use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream, SocketAddr},
    fs::{File, OpenOptions},
    collections::HashMap,
    path::Path,
};
use chrono::prelude::*;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7879").unwrap();

    /*
    let mut log = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .expect("Could not open File.");

     */

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        //log_request(&stream, &mut log);
        let test = parse_request(&stream);
        println!("{:?}", test);
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

/// Parses request from TcpStream into HttpRequest struct
fn parse_request(mut stream: &TcpStream) -> HttpRequest {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut line = String::new();
    buf_reader.read_line(&mut line).expect("Unable to read line from BufReader.");
    // Read request line
    let request_line = parse_request_line(&line).expect("Unable to parse request line.");
    let mut headers = HashMap::new();
    line.clear();   // buf_reader.read_line appends to the String buffer thus it need to be cleared here
    // Read headers
    buf_reader.read_line(&mut line).expect("Unable to read line from BufReader.");
    while line.contains(":") {
        let line_vec: Vec<&str> = line.trim().split(": ").collect();
        headers.insert(line_vec[0].to_string(), line_vec[1].to_string());
        line.clear();   // buf_reader.read_line appends to the String buffer thus it need to be cleared here
        buf_reader.read_line(&mut line).expect("Unable to read line from BufReader.");
    }
    // Read body (if one was declared in headers)
    // Currently only reads UTF-8 bodies
    // TODO Implement MIME type bodies
    let body: Option<String>;
    if headers.contains_key("content-length") {
        let body_length = headers.get("content-length").unwrap().parse::<usize>().
            expect("Content length header could not be parsed to usize.");
        let mut body_buffer = vec![0u8; body_length];
        buf_reader.read_exact(&mut body_buffer).expect("Unable to read body from buf_reader.");
        body = match String::from_utf8(body_buffer) {
            Ok(v) => Some(v),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
    } else {
        body = None;
    }
    // Return HttpRequest adding timestamp and sender ip in the process
    HttpRequest {
        request_line: Box::from(request_line),
        headers,
        body,
        timestamp: timestamp(),
        sender_ip: stream.local_addr().expect("Could not read sender ip."),
    }
}

/// Expects the first line from an HTTP Request and parses it into the RequestLine struct
fn parse_request_line(mut line: &String) -> Result<RequestLine, &str, > {
    let line: Vec<&str> = line.split(" ").collect();
    if line.len() != 3 {
        return Err("Invalid Request Line.");
    }
    let verb= match line[0] {
            "GET" => Verb::GET,
            "HEAD" => Verb::HEAD,
            "POST" => Verb::POST,
            "PUT" => Verb::PUT,
            "DELETE" => Verb::DELETE,
            "CONNECT" => Verb::CONNECT,
            "OPTIONS" => Verb::OPTIONS,
            "TRACE" => Verb::TRACE,
            "PATCH" => Verb::PATCH,
            _ => return Err("Invalid HTTP Verb.")
        };
        let target =
            if matches!(line[1], "/*") && matches!(verb, Verb::OPTIONS) {
                Box::from(Target::AsteriskForm('*'))
            } else if matches!(verb, Verb::GET) && line[1].contains("https://") {
                Box::from(Target::CompleteUrl(String::from(line[1])))
            } else if matches!(verb, Verb::CONNECT) {
                Box::from(Target::AuthorityComponent)
            } else {
                Box::from(Target::AbsolutePath(Box::from(Path::new(line[1]))))
            };
        let version =
            if line[2].contains("HTTP/1.0") {
                HTTPVersion::HTTP1_0
            } else if line[2].contains("HTTP/1.1") {
                HTTPVersion::HTTP1_1
            } else {
                return Err("Invalid HTTP Version.")
            };

    Ok(RequestLine{verb, target, version})
}


#[derive(Debug)]
struct HttpRequest {
    timestamp: String,
    sender_ip: SocketAddr,
    request_line : Box<RequestLine>,
    headers : HashMap<String, String>,
    body : Option<String>
}

#[derive(Debug)]
struct RequestLine {
    verb : Verb,
    target : Box<Target>,
    version : HTTPVersion
}

#[derive(Debug)]
enum Verb {
    GET, HEAD, POST, PUT, DELETE, CONNECT, OPTIONS, TRACE, PATCH
}

#[derive(Debug)]
enum Target {                   // defaults to AbsolutePath
    AbsolutePath(Box<Path>),
    CompleteUrl(String),        // TODO Check for better type
    AuthorityComponent,         // TODO figure out good representation
    AsteriskForm(char)
}

#[derive(Debug)]
enum HTTPVersion {
    HTTP1_0,
    HTTP1_1
}
