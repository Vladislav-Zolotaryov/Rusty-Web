use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use sqlx::{postgres::{PgPoolOptions, PgPool}, Row};
use log::error;
use env_logger;

async fn hello(pool: web::Data<PgPool>) -> impl Responder {
    let recs = sqlx::query(
        "
        SELECT id, description, done
        FROM todos
        ORDER BY id
        "
    )
    .fetch_all(pool.get_ref())
    .await;
    
    return match recs {
        Ok(_) => HttpResponse::Ok().body("Hello world!"),
        Err(error) => { error!("Failed due {}", error); return HttpResponse::InternalServerError().body("Something went wrong") }
    }    
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://rust:todoist@localhost/todoist")
        .await
        .expect("Failed to create DB connection");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::resource("/").route(web::get().to(hello))
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
