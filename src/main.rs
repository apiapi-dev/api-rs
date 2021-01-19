#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpRequest, HttpServer, Responder};

// use diesel::prelude::*;
// use diesel::r2d2::{self, ConnectionManager};

mod routes {
    pub mod debug;
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default().allow_any_origin().max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .service(routes::debug::status)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
