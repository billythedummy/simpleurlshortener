use deadpool_postgres::Client;

use crate::err::Error;
use crate::url_from_hash;
use crate::viz::HitEntry;

pub async fn viz_hits(client: &Client, user_id: i32) -> Result<Vec<HitEntry>, Error> {
    let stmt = "SELECT b.hashval, a.c FROM\n\
    (SELECT url_hash_hashval, COUNT(*) AS c FROM hit GROUP BY url_hash_hashval) a\n\
    RIGHT JOIN\n\
    (SELECT hashval from url_hash WHERE app_user_id=$1) b\n\
    ON a.url_hash_hashval = b.hashval;";
    let pst = client.prepare(stmt).await?;
    let res = client
        .query(&pst, &[&user_id])
        .await?
        .iter()
        .map(|row| HitEntry {
            url: url_from_hash(row.get("hashval")),
            hits: match row.get("c") {
                Some(h) => h,
                None => 0
            },
        })
        .collect::<Vec<HitEntry>>();
    Ok(res)
}
