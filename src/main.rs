#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

mod routes {
    use super::models;
    pub mod get;
    pub mod create;
    pub mod debug;
}
pub mod models;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin().max_age(3600);

        App::new()
            .data(pool.clone())
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .service(routes::debug::root)
            .service(routes::debug::status)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
