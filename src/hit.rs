use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use deadpool_postgres::Pool;
use regex::Regex;
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;

use crate::db::{get_redirect_url, register_hit};
use crate::{client_from_pool, hash_from_url};

#[get("/{hashstr}")]
async fn hit_page(
    web::Path(hashstr): web::Path<String>,
    db_pool: web::Data<Pool>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let client = client_from_pool!(db_pool);
    let hash = hash_from_url(&hashstr)?;
    let url = get_redirect_url(&client, hash).await?;
    let ip_port_regex = Regex::new(r"^([\w:.]+):(\d{1,5})$").unwrap();
    // WARNING: just reads raw headers, can be spoofed by client
    let client_ip = match req.connection_info().realip_remote_addr() {
        Some(ipportstr) => {
            let ipstr = match ip_port_regex.captures(ipportstr) {
                Some(c) => c.get(1).unwrap().as_str(),
                None => ipportstr,
            };
            match IpAddr::from_str(ipstr) {
                Ok(ipaddr) => ipaddr,
                Err(_e) => {
                    eprintln!("Failed to parse client ip {}", ipstr);
                    IpAddr::from(Ipv4Addr::UNSPECIFIED)
                }
            }
        },
        None => {
            eprintln!("No IP address found in client request");
            IpAddr::from(Ipv4Addr::UNSPECIFIED)
        }
    };
    tokio::spawn(async move {
        if let Err(e) = register_hit(&client, hash, &client_ip).await {
            eprintln!("Error registering hit: {}", e);
        }
    });
    Ok(HttpResponse::PermanentRedirect()
        .header("Location", url)
        .finish())
}
