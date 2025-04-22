use actix_web::{post, web, HttpResponse};
use models::Command;
use mongodb::Client;

#[post("/command")]
pub async fn add_command(client: web::Data<Client>, form: web::Form<Command>) -> HttpResponse {
    let collection = client
        .default_database()
        .expect("No default database found.")
        .collection(Command::COMMAND);
    let result = collection.insert_one(form.into_inner()).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("Command added"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
