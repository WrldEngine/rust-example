mod api;
mod db;

use actix_web::{App, HttpServer};
use api::validator;
use db::core::{DataBase};

async fn database_init() {
    let pool = DataBase::create_pool().await;
    DataBase::create_tables(&pool).await;
}


#[tokio::main]
async fn main() -> std::io::Result<()> {
    database_init().await;

    println!("server started");
    HttpServer::new(|| {
        App::new()
            .service(validator::hello)
            .service(validator::echo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}