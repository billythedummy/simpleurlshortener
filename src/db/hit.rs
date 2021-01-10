use chrono::offset::Utc;
use deadpool_postgres::Client;
use std::net::IpAddr;

use crate::err::Error;

pub async fn get_redirect_url(client: &Client, hash: i32) -> Result<String, Error> {
    let stmt = "SELECT url FROM url_hash WHERE hashval=$1 LIMIT 1;";
    let pst = client.prepare(stmt).await?;
    let mut url: String = client
        .query_one(&pst, &[&hash])
        .await
        .map_err(|_e| Error {
            pub_reason: "Failed to get URL".into(),
            status_code: 401,
        })?
        .get("url");
    if !url.contains("://") {
        url = format!("http://{}", url);
    }
    Ok(url)
}

pub async fn register_hit(client: &Client, hash: i32, client_addr: &IpAddr) -> Result<(), Error> {
    let stmt = "INSERT INTO hit (url_hash_hashval, ts, addr) VALUES ($1, $2, $3);";
    let pst = client.prepare(stmt).await?;
    client
        .query(&pst, &[&hash, &Utc::now(), &client_addr])
        .await?;
    Ok(())
}
