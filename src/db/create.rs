use deadpool_postgres::Client;
use rand::Rng;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::err::Error;

const APPROX_I32_MAX_SQRT: i32 = 46340;

pub async fn create_new_hash(client: &Client, user_id: i32, url: &str) -> Result<i32, Error> {
    let stmt = "INSERT INTO url_hash (hashval, url, app_user_id) VALUES ($1, $2, $3);";
    let pst = client.prepare(stmt).await?;
    // Calculate hash
    let mut rng = rand::thread_rng();
    let salt: u64 = rng.gen();
    let mut hasher = DefaultHasher::new();
    url.hash(&mut hasher);
    let hash_long = hasher.finish().wrapping_add(salt);
    let hash_long_bytes = hash_long.to_be_bytes();
    let mut hash_bytes = [0; 4];
    hash_bytes.copy_from_slice(&hash_long_bytes[4..]);
    let hash = i32::from_be_bytes(hash_bytes);
    let mut factor = 0;
    // Keep trying with "quadratic probing" if initial hash collision
    loop {
        if factor > APPROX_I32_MAX_SQRT {
            return Err(Error {
                pub_reason: "Failed to generate URL".into(),
                status_code: 500,
            });
        }
        if let Ok(_rows) = client
            .query(&pst, &[&hash.wrapping_add(factor * factor), &url, &user_id])
            .await
        {
            return Ok(hash);
        }
        factor += 1;
    }
}
