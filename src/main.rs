use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
struct Task {
    id: Uuid,
    title: String,
    description: String,
}

struct AppState {
    tasks: Mutex<Vec<Task>>,
}

// Cria uma nova tarefa
async fn create_task(task: web::Json<Task>, data: web::Data<AppState>) -> impl Responder {
    let mut tasks = data.tasks.lock().unwrap();
    let new_task = Task {
        id: Uuid::new_v4(),
        title: task.title.clone(),
        description: task.description.clone(),
    };
    tasks.push(new_task.clone());
    HttpResponse::Created().json(new_task)
}

// Retorna todas as tarefas
async fn get_tasks(data: web::Data<AppState>) -> impl Responder {
    let tasks = data.tasks.lock().unwrap();
    HttpResponse::Ok().json(&*tasks)
}

// Retorna uma tarefa específica pelo ID
async fn get_task(task_id: web::Path<Uuid>, data: web::Data<AppState>) -> impl Responder {
    let tasks = data.tasks.lock().unwrap();
    if let Some(task) = tasks.iter().find(|task| task.id == *task_id) {
        HttpResponse::Ok().json(task)
    } else {
        HttpResponse::NotFound().body("Task not found")
    }
}

// Atualiza uma tarefa existente
async fn update_task(task_id: web::Path<Uuid>, updated_task: web::Json<Task>, data: web::Data<AppState>) -> impl Responder {
    let mut tasks = data.tasks.lock().unwrap();
    if let Some(task) = tasks.iter_mut().find(|task| task.id == *task_id) {
        task.title = updated_task.title.clone();
        task.description = updated_task.description.clone();
        HttpResponse::Ok().json(task)
    } else {
        HttpResponse::NotFound().body("Task not found")
    }
}

// Remove uma tarefa com base no ID
async fn delete_task(task_id: web::Path<Uuid>, data: web::Data<AppState>) -> impl Responder {
    let mut tasks = data.tasks.lock().unwrap();
    if tasks.iter().position(|task| task.id == *task_id).is_some() {
        tasks.retain(|task| task.id != *task_id);
        HttpResponse::Ok().body("Task deleted")
    } else {
        HttpResponse::NotFound().body("Task not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        tasks: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/tasks", web::post().to(create_task))
            .route("/tasks", web::get().to(get_tasks))
            .route("/tasks/{id}", web::get().to(get_task))
            .route("/tasks/{id}", web::put().to(update_task))
            .route("/tasks/{id}", web::delete().to(delete_task))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
