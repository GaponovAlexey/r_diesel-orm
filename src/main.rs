use actix_web::{ get, web, App, HttpServer, Responder };

use serde::{ Deserialize, Serialize };
mod services;
use std::sync::Mutex;

struct AppState {
    todo_list_entries: Mutex<Vec<TodoListEntry>>,
}
#[derive(Serialize, Deserialize, Clone)]
struct TodoListEntry {
    id: i32,
    date: i64,
    title: String,
}

#[get("/")]
async fn my_index() -> String {
    "say hi".to_string()
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState {
        todo_list_entries: Mutex::new(vec![]),
    });

    HttpServer::new(move || {
        App::new().app_data(app_data.clone()).configure(services::config).service(my_index)
    })
        .bind(("127.0.0.1", 3000))?
        .run().await
}
