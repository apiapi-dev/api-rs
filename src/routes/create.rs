use crate::DbPool;
use actix_web::{post, web, Error, HttpResponse};
use diesel::prelude::*;

use super::models::NewApi;

#[post("/api")]
pub async fn add_api(
    pool: web::Data<DbPool>,
    json: web::Json<NewApi>,
) -> Result<HttpResponse, Error> {
    create_api(pool, json).await
}

fn insert_new_api<'a>(
    // prevent collision with `name` column imported inside the function
    new_user: &NewApi,
    conn: &PgConnection,
) -> Result<(), diesel::result::Error> {
    use crate::schema::apis::dsl::*;

    diesel::insert_into(apis).values(new_user).execute(conn)?;

    Ok(())
}

async fn create_api(
    pool: web::Data<DbPool>,
    json: web::Json<NewApi>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    
    //let user = web::block(move || actions::insert_new_user(&json.name, &json.username, &json.password, &json.email, &json.role, &conn))
    web::block(move || insert_new_api(&json.into_inner(), &conn))
        .await
        .map_err(|e| {
            eprintln!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    let response = format!("Successfully Created API");
    Ok(HttpResponse::Ok().body(response))
}