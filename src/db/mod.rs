mod create;
mod hit;
mod users;
mod viz;

// Re-exports
pub use create::create_new_hash;
pub use hit::{get_redirect_url, register_hit};
pub use users::get_user_id;
pub use viz::viz_hits;

use deadpool_postgres::config::Config;
use deadpool_postgres::Pool;
use std::env;
use std::process::exit;
use tokio_postgres::NoTls;

pub fn create_db_pool() -> Pool {
    let mut config = Config::new();
    config.user = match env::var("PG_USER") {
        Ok(s) => Some(s),
        Err(_) => Some("postgres".into()),
    };
    config.dbname = match env::var("PG_DB") {
        Ok(s) => Some(s),
        Err(_) => Some(config.user.as_ref().unwrap().clone()),
    };
    if let Ok(pass) = env::var("PG_PASSWORD") {
        config.password = Some(pass)
    }
    if let Ok(port_str) = env::var("PG_PORT") {
        if let Ok(port) = port_str.parse::<u16>() {
            config.port = Some(port);
        }
    }
    match config.create_pool(NoTls) {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("DB connection error: {}, exiting", e);
            exit(-1);
        }
    }
}
