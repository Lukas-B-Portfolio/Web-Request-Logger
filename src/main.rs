mod http_request;

use std::{net::{TcpListener}, error::Error};
use sqlx::{Row};
use crate::http_request::{HttpRequest, parse_request, response};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://rust:rust@localhost:5432/HttpRequestLog";
    let pool = sqlx::postgres::PgPool::connect(url).await?;
    let listener = TcpListener::bind("127.0.0.1:7878")
        .expect("Unable to add TcpListener to address.");

    sqlx::migrate!("./migrations").run(&pool).await?;

    for stream in listener.incoming() {
        let stream = stream.expect("Unable to get TcpStream from TcpListener.");
        let request = parse_request(&stream);
        insert(&request, &pool).await?;
        println!("{:?}", request);
        response(stream);
    }

    let res = sqlx::query("SELECT 1 + 1 as sum")
        .fetch_one(&pool)
        .await?;

    let sum: i32 = res.get("sum");
    println!("1 + 1 = {}", sum);

    //println!("{:?}", read(&pool).await.unwrap());

    Ok(())
}

async fn insert(http_request: &HttpRequest, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO http_request (timestamp, host_ip, sender_ip, verb, target, version)\
                        VALUES ($1, $2, $3, $4, $5, $6) RETURNING id";

    let id: (i32, ) = sqlx::query_as(query)
        .bind(&http_request.timestamp)
        .bind(&http_request.sender_ip)  // TODO figure out how to represent host header
        .bind(&http_request.sender_ip)
        .bind(&http_request.request_line.verb.to_string())
        .bind(&http_request.request_line.target.to_string())
        .bind(&http_request.request_line.version.to_string())
        .fetch_one(pool)
        .await?;

    let query = "INSERT INTO headers (id, name, value)\
                        VALUES ($1, $2, $3)";

    for (key, value) in &http_request.headers {
        sqlx::query(query)
            .bind(id.0)
            .bind(key)
            .bind(value)
            .execute(pool)
            .await?;
    }

    // TODO Make last query conditional on body state
    let query = "INSERT INTO body (id, body_text)\
                        VALUES ($1, $2)";

    sqlx::query(query)
        .bind(id.0)
        .bind(&http_request.body)
        .execute(pool)
        .await?;

    Ok(())
}

/*
fn logging_setup() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let mut log = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .expect("Could not open File.");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        log_request(&stream, &mut log);
        let request = parse_request(&stream);
        println!("{:?}", request);
        response(stream);
    }
}
*/




