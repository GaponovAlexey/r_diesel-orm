use actix_web::{ get, post, put, web, delete, Responder, HttpResponse };
use serde::Deserialize;
use crate::{ AppState, TodoListEntry };
#[get("/todolist/entries")]
async fn get_entries(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(data.todo_list_entries.lock().expect("err entries").to_vec())
}

#[derive(Deserialize, Clone)]
pub struct CreateEntryData {
    pub title: String,
    pub date: i64,
}

#[derive(Deserialize, Clone)]
pub struct UpdateEntryData {
    pub title: String,
}

#[derive(Deserialize)]
pub struct CreateArticleBody {
    pub title: String,
    pub content: String,
}

#[post("/todolist/entries")]
async fn create_entry(
    data: web::Data<AppState>,
    param_obj: web::Json<CreateEntryData>
) -> impl Responder {
    let mut todo_list_entries = data.todo_list_entries.lock().expect("err post");
    let mut max_id = 0;

    for i in 0..todo_list_entries.len() {
        if todo_list_entries[i].id > max_id {
            max_id = todo_list_entries[i].id;
        }
    }

    todo_list_entries.push(TodoListEntry {
        id: max_id + 1,
        title: param_obj.title.clone(),
        date: param_obj.date,
    });

    HttpResponse::Ok().json(todo_list_entries.to_vec())
}

#[put("todolist/entries/{id}")]
async fn update_entry(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    param_obj: web::Json<UpdateEntryData>
) -> impl Responder {
    let id = path.into_inner();
    let mut todo_list_entries = data.todo_list_entries.lock().expect("err");
    for i in 0..todo_list_entries.len() {
        if todo_list_entries[i].id == id {
            todo_list_entries[i].title = param_obj.title.clone();
            break;
        }
    }

    HttpResponse::Ok().json(todo_list_entries.to_vec())
}
#[delete("todolist/entries/{id}")]
async fn delete_entry(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let mut todo_list_entries = data.todo_list_entries.lock().expect("err");

    let id = path.into_inner();
    *todo_list_entries = todo_list_entries
        .to_vec()
        .into_iter()
        .filter(|x| x.id != id)
        .collect();

    HttpResponse::Ok().json(todo_list_entries.to_vec())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_entries).service(create_entry).service(update_entry).service(delete_entry);
}
