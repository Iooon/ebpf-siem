use mongodb::{
    options::{ClientOptions, Credential},
    Client,
};

use models::Command;

pub struct Database;

impl Database {
    pub async fn init() -> Client {
        let uri = std::env::var("MONGODB_URI")
            .unwrap_or_else(|_| "mongodb://127.0.0.1:27017/?directConnection=true".into());
        let username = std::env::var("MONGODB_USERNAME").unwrap_or_else(|_| "admin".into());
        let password = std::env::var("MONGODB_PASSWORD").unwrap_or_else(|_| "admin".into());
        let db_name = std::env::var("MONGODB_DB").unwrap_or_else(|_| "test_data".into());

        let mut client_options = ClientOptions::parse(uri).await.expect("Cannot parse uri.");
        client_options.default_database = Some(db_name.clone());

        let default_cred = Credential::builder()
            .username(username)
            .password(password)
            .source(db_name)
            .build();

        client_options.credential = Some(default_cred);
        let client = Client::with_options(client_options).expect("Failed to connect.");

        Database::setup_database(&client).await;
        println!("Database connected");
        client
    }

    async fn setup_database(client: &Client) {
        Command::setup_entry(client).await;
    }

    pub async fn clear_database(client: &Client) {
        Command::clear(client).await;
    }
}
