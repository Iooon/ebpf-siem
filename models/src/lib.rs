use mongodb::{bson::doc, options::IndexOptions, Client, IndexModel};
use serde::{Deserialize, Serialize};
#[cfg(test)]
mod tests;

#[derive(Debug, Serialize, Deserialize)]
pub struct Command {
    pub userid: u32,
    pub command: String,
}

impl Command {
    pub const COMMAND: &str = "commands";

    pub async fn setup_entry(client: &Client) {
        let options = IndexOptions::builder().unique(true).build();
        let model = IndexModel::builder()
            .keys(doc! { "userid": 1 })
            .options(options)
            .build();
        client
            .default_database()
            .expect("No default database found")
            .collection::<Command>(Command::COMMAND)
            .create_index(model)
            .await
            .expect("Failed to create index on commands");
    }

    pub async fn clear(client: &Client) {
        client
            .default_database()
            .expect("No default database found")
            .collection::<Command>(Command::COMMAND)
            .drop()
            .await
            .expect("Failed to clear commands");
    }
}
