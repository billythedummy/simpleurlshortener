use actix_web::{get, post, web, Error, HttpResponse};
use askama::Template;
use deadpool_postgres::Pool;
use serde::Deserialize;

use crate::db::{get_user_id, viz_hits};
use crate::{client_from_pool, unwrap_or_server_err};

#[derive(Template)]
#[template(path = "viz/index.html")]
struct VizIndexPage;

#[derive(Deserialize)]
struct UserForm {
    user: String,
}

#[derive(Debug)]
pub struct HitEntry {
    pub url: String,
    pub hits: i64,
}

#[derive(Template)]
#[template(path = "viz/hits.html")]
struct VizPage<'a> {
    list: &'a Vec<HitEntry>,
}

#[get("/viz")]
async fn viz_index_page() -> Result<HttpResponse, Error> {
    let html = unwrap_or_server_err!(VizIndexPage.render());
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

#[post("/hits")]
async fn viz_hits_post(
    form: web::Form<UserForm>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client = client_from_pool!(db_pool);
    let user_id = get_user_id(&client, &form.user).await?;
    let hit_entry_list = viz_hits(&client, user_id).await?;
    let success = unwrap_or_server_err!(VizPage {
        list: &hit_entry_list
    }
    .render());
    Ok(HttpResponse::Ok().content_type("text/html").body(success))
}
