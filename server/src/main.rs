mod routes;
mod services;
#[cfg(test)]
mod test;

use actix_web::{get, web, App, HttpResponse, HttpServer};
use mongodb::Client;
use routes::command_route::add_command;
use services::db::Database;

#[get("/")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello client!")
}

#[get("/clear")]
async fn clear(client: web::Data<Client>) -> HttpResponse {
    Database::clear_database(&client).await;
    HttpResponse::Ok().body("Cleared database")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Database::init().await;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(hello)
            .service(clear)
            .service(add_command)
    })
    .bind(("localhost", 5001))?
    .run()
    .await
}
