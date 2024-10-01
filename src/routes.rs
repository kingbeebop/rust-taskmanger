use rocket::{get, post, put, delete, serde::json::Json};
use crate::models::{Task, User};
use crate::db;
use sqlx::PgPool;

// Fetch all tasks
#[get("/tasks")]
pub async fn get_tasks(pool: &PgPool) -> Json<Vec<Task>> {
    let tasks = sqlx::query_as::<_, Task>("SELECT * FROM tasks")
        .fetch_all(pool)
        .await
        .unwrap();

    Json(tasks)
}

// Create a new task
#[post("/tasks", data = "<task>")]
pub async fn create_task(pool: &PgPool, task: Json<Task>) -> Json<Task> {
    let task = sqlx::query_as::<_, Task>(
        "INSERT INTO tasks (title, description, completed, user_id) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(&task.title)
    .bind(&task.description)
    .bind(task.completed)
    .bind(task.user_id)
    .fetch_one(pool)
    .await
    .unwrap();

    Json(task)
}
