#[macro_use] extern crate rocket;

mod db;
mod models;
mod routes;

use routes::{get_tasks, create_task};
use rocket::fairing::AdHoc;
use sqlx::PgPool;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AdHoc::on_ignite("Database", |rocket| async {
            let pool = db::establish_connection().await;
            rocket.manage(pool)
        }))
        .mount("/", routes![get_tasks, create_task])
}
