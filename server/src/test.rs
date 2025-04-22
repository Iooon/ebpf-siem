use actix_web::{
    test::{call_and_read_body, init_service, TestRequest},
    web::Bytes,
};

use super::*;
use models::Command;

#[actix_web::test]
async fn test() {
    let db = Database::new();
    let client = db.init().await;

    // Clear any data currently in the users collection.
    db.clear_database(&client).await;

    let app = init_service(
        App::new()
            .app_data(web::Data::new(client))
            .service(add_command),
    )
    .await;

    let command = Command {
        userid: 1,
        command: "ls".into(),
    };

    let req = TestRequest::post()
        .uri("/command")
        .set_form(&command)
        .to_request();

    let response = call_and_read_body(&app, req).await;
    assert_eq!(response, Bytes::from_static(b"Command added"));
}
