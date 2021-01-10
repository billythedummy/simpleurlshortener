#[macro_export]
macro_rules! unwrap_or_server_err {
    ($res : expr) => {
        $res.map_err(|_e| {
            actix_web::Error::from(actix_web::HttpResponse::InternalServerError().finish())
        })?
    };
}

#[macro_export]
macro_rules! client_from_pool {
    ($db_pool : expr) => {
        $db_pool
            .get()
            .await
            .map_err(|e| Into::<actix_web::Error>::into(crate::err::Error::from(e)))?
    };
}
