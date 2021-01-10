use deadpool_postgres::Client;
use rand::Rng;

use crate::err::Error;

pub async fn create_new_hash(client: &Client, user_id: i32, url: &str) -> Result<i32, Error> {
    let mut rng = rand::thread_rng();
    let stmt = "INSERT INTO url_hash (hashval, url, app_user_id) VALUES ($1, $2, $3);";
    let pst = client.prepare(stmt).await?;
    // try random generation at most 3 times before erroring out
    for _i in 0..3 {
        let hash: i32 = rng.gen();
        if let Ok(_rows) = client.query(&pst, &[&hash, &url, &user_id]).await {
            return Ok(hash);
        }
    }
    Err(Error {
        pub_reason: "Failed to generate URL".into(),
        status_code: 500,
    })
}
