use std::io;

use actix_web::{App, HttpServer, middleware::Logger};
use env_logger::Env;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| App::new().wrap(Logger::default()))
        .bind(("0.0.0.0", 5000))?
        .run()
        .await
}
