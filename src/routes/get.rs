use crate::DbPool;
use actix_web::{get, web, Error, HttpMessage, HttpRequest, HttpResponse};
use diesel::prelude::*;
use diesel::PgConnection;
use uuid::Uuid;

use super::models::Api;

#[get("/apis")]
pub async fn apis(pool: web::Data<DbPool>, req: HttpRequest) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let apis = get_apis(&conn).unwrap();
    Ok(HttpResponse::Ok().json(apis))
}

pub fn get_apis(conn: &PgConnection) -> Result<Vec<Api>, diesel::result::Error> {
    use crate::schema::apis::dsl::*;

    let api_result = apis.load::<Api>(conn).expect("Error loading APIs");

    Ok(api_result)
}
