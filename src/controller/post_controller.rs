use crate::service::service_post::*;
use crate::{domain::input::post_in_dto::PostInDto, infrastructure::configuration::DbPool};
use actix_web::{
    delete, get, post, put, web,
    web::{Data, Json, Path},
    HttpResponse, Responder, Result,
};

#[post("")]
pub async fn create_post(
    pool: web::Data<DbPool>,
    post_in: Json<PostInDto>,
) -> Result<impl Responder> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    match web::block(move || new_post(&mut conn, post_in.clone())).await {
        Ok(data) => Ok(HttpResponse::Ok().json(web::Json(data.ok()))),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Error getting post")),
    }
}

#[get("")]
pub async fn get_post_all(pool: Data<DbPool>) -> Result<impl Responder> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    match web::block(move || get_all_posts(&mut conn)).await {
        Ok(data) => Ok(HttpResponse::Ok().json(web::Json(data.ok()))),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Error getting post")),
    }
}

#[get("/{id}")]
pub async fn get_post(pool: Data<DbPool>, id: Path<i32>) -> Result<impl Responder> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let mut _id = id.clone();

    match web::block(move || get_post_by_id(&mut conn, _id)).await {
        Ok(data) => Ok(HttpResponse::Ok().json(web::Json(data))),
        Err(_) => {
            Ok(HttpResponse::NotFound()
                .body(format!("Error getting post by id: {}", id.into_inner())))
        }
    }
}

#[post("/all-in")]
pub async fn get_post_all_in(pool: Data<DbPool>, ids: Json<Vec<i32>>) -> Result<impl Responder> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let mut _ids = ids.clone();
    print!("{:?}", _ids);

    match web::block(move || by_ids(&mut conn, ids.clone())).await {
        Ok(data) => Ok(HttpResponse::Ok().json(web::Json(data.ok()))),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Error getting post")),
    }
}

#[put("/{id}")]
pub async fn update_post(
    pool: Data<DbPool>,
    id: Path<i32>,
    post_in: Json<PostInDto>,
) -> Result<impl Responder> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let mut _id = id.clone();
    match web::block(move || {
        crate::service::service_post::update_post(&mut conn, _id, post_in.clone())
    })
    .await
    {
        Ok(data) => Ok(HttpResponse::Ok().json(web::Json(data.ok()))),
        Err(_) => Ok(HttpResponse::NotModified()
            .body(format!("Error updating post by id: {}", id.into_inner()))),
    }
}

#[delete("/{id}")]
pub async fn delete_post(pool: Data<DbPool>, id: Path<i32>) -> Result<impl Responder> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let mut _id = id.clone();

    match web::block(move || crate::service::service_post::delete_post(&mut conn, _id)).await {
        Ok(data) => Ok(HttpResponse::Ok().json(web::Json(data.ok()))),
        Err(_) => Ok(HttpResponse::InternalServerError()
            .body(format!("Error deleting post by id: {}", id.into_inner()))),
    }
}
