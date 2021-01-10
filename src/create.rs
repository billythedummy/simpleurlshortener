use actix_web::{get, post, web, Error, HttpResponse};
use askama::Template;
use deadpool_postgres::Pool;
use serde::Deserialize;

use crate::db::{create_new_hash, get_user_id};
use crate::{client_from_pool, unwrap_or_server_err, url_from_hash};

#[derive(Template)]
#[template(path = "create/create.html")]
struct CreateFormPage;

#[derive(Deserialize)]
struct CreateForm {
    user: String,
    url: String,
}

#[derive(Template)]
#[template(path = "create/create_success.html")]
struct CreateSuccessPage<'a> {
    url: &'a str,
}

#[get("/create")]
async fn create_form_page() -> Result<HttpResponse, Error> {
    let html = unwrap_or_server_err!(CreateFormPage.render());
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

#[post("/create")]
async fn create_post(
    form: web::Form<CreateForm>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client = client_from_pool!(db_pool);
    let user_id = get_user_id(&client, &form.user).await?;
    let hash = create_new_hash(&client, user_id, &form.url).await?;
    let success = unwrap_or_server_err!(CreateSuccessPage {
        url: &url_from_hash(hash)
    }
    .render());
    Ok(HttpResponse::Ok().content_type("text/html").body(success))
}
