use deadpool_postgres::Client;

use crate::err::Error;

pub async fn get_user_id(client: &Client, username: &str) -> Result<i32, Error> {
    let stmt = "SELECT id FROM app_user WHERE username=$1 LIMIT 1;";
    let pst = client.prepare(stmt).await?;
    let user_row = client
        .query_one(&pst, &[&username])
        .await
        .map_err(|_e| Error {
            pub_reason: "Failed to get user".into(),
            status_code: 401,
        })?;
    Ok(user_row.get("id"))
}
