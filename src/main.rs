use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream, SocketAddr},
    fs::{File, OpenOptions},
    collections::HashMap,
    path::Path
};
use chrono::prelude::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7879").unwrap();

    /*
    let mut log = OpenOptions::new()
        .append(true)
        .open("log.txt")
        .expect("Could not open File.");

     */

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        //log_request(&stream, &mut log);
        parse_request(&stream);
        response(stream);
    }
}

fn response(mut stream: TcpStream) {
    let status_line = "HTTP/1.1 200 OK";
    let response =
        format!("{status_line}\r\n");

    stream.write_all(response.as_bytes()).unwrap();
}

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
    log.write("Received Request at:\t".as_bytes()).expect("Failed to write to log.");
    log.write(timestamp().as_bytes()).expect("Failed to write Time to log.");
    log.write("\nFrom IP:\t\t\t\t".as_bytes()).expect("Failed to write to log.");
    log.write(client_ip.as_bytes()).expect("Failed to write IP to log.");
    log.write("\nRequest:\n".as_bytes()).expect("Failed to write to log.");

    for s in &http_request {
        log.write(s.as_bytes()).expect("Failed to write part of the request to log.");
        log.write("\n".as_bytes()).expect("Failed to write to log.");
    }
    println!("Request: {:#?}", http_request);
}

fn timestamp() -> String{
    format!("{}", Local::now().format("%d/%m/%Y %H:%M:%S"))
}

fn parse_request(mut stream: &TcpStream) -> HttpRequest {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut test = String::new();
    buf_reader.read_line(&mut test).expect("TODO: panic message");
    let line = parse_request_line(&test);
    println!("{:?}", line.unwrap());
    HttpRequest {
        request_line: Box::from(RequestLine {
            verb: Verb::GET,
            target: Box::from(Target::CompleteUrl(String::from("Test"))),
            version: HTTPVersion::HTTP1_0,
        }),
        headers: Default::default(),
        body: None,
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



