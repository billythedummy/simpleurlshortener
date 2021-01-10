mod create;
mod db;
mod err;
mod hit;
mod macros;
mod viz;

use actix_web::{App, HttpServer};
use std::env;

use create::*;
use db::create_db_pool;
use err::Error;
use hit::*;
use viz::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = create_db_pool();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(create_form_page)
            .service(create_post)
            .service(viz_index_page)
            .service(viz_hits_post)
            // must put this last else take precedence
            .service(hit_page)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

pub fn url_from_hash(hash: i32) -> String {
    // 2's complement hex
    let hashstr = format!("{:01$x}", hash, 8);
    match env::var("URL_HOST") {
        Ok(s) => format!("{}/{}", s, hashstr),
        Err(_) => hashstr,
    }
}

pub fn hash_from_url(hex: &str) -> Result<i32, Error> {
    let mut buf: [u8; 4] = [0; 4];
    let mut success = true;
    if hex.len() == 8 {
        for i in 0..4 {
            buf[i] = match u8::from_str_radix(&hex[2*i..2*i+2], 16) {
                Ok(b) => b,
                Err(_parse_err) => {
                    success = false;
                    break;
                }
            }
        }
        if success {
            return Ok(i32::from_be_bytes(buf));
        }
    }
    Err(Error {
        pub_reason: format!("Invalid URL {}", hex),
        status_code: 401,
    })
}
