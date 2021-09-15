use actix_web::{
    middleware::Logger,
    web::{scope, resource, get, post, delete},
    App,
    HttpServer
};
use actix_web::web::put;

mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db: models::user::RocksDB = models::user::KVStore::init("../../tmp/rocks/actix-db");

    HttpServer::new(move || {
        App::new()
            .data(db.clone())
            .service(
                scope("/api")
                    .route("", post().to(models::post))
                    .route("",put().to(models::put))
                    .service(
                        resource("/{key}")
                            .route(get().to(models::get))
                            .route(delete().to(models::delete)),
                    ),
            )
    })
        .bind("localhost:8080")?
        .run()
        .await
}