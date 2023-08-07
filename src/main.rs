use actix_web::{ web::Data, App, HttpServer };
use actix::SyncArbiter;

use serde::{ Deserialize, Serialize };

// use diesel::{ r2d2::{ ConnectionManager, Pool }, PgConnection };

use dotenv::dotenv;
use std::env;

// mod import 
mod services;
mod db_utils;
mod messages;
mod actors;
mod db_models;
mod schema;
mod insertables;


#[derive(Serialize, Deserialize, Clone)]
struct TodoListEntry {
    id: i32,
    date: i64,
    title: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
   
    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
   
    let pool = db_utils::get_pool(&db_url);
   
    let db_addr = SyncArbiter::start(5, move || db_utils::DbActor(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .app_data(
                Data::new(db_utils::AppState {
                    db: db_addr.clone(),
                })
            )
            .configure(services::config)
    })
        .bind(("127.0.0.1", 3000))?
        .run().await
}
