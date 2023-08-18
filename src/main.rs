use actix_web::{web, App, HttpServer};
mod controller;
use controller::post_controller::{
    create_post, delete_post, get_post, get_post_all, get_post_all_in, update_post,
};
mod domain;
mod infrastructure;
pub mod schema;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _db_connetion = infrastructure::configuration::opendb();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(_db_connetion.clone()))
            .service(
                web::scope("/api/v1/posts")
                    .service(create_post)
                    .service(get_post)
                    .service(get_post_all)
                    .service(get_post_all_in)
                    .service(update_post)
                    .service(delete_post),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
