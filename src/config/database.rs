use mongodb::{Client, Database};
use std::env;

pub async fn establish_connection(database_url: &str) -> Result<Database, mongodb::error::Error> {
    let client = Client::with_uri_str(database_url).await?;
    let db_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
    Ok(client.database(db_name.as_str()))
}

pub const mongourl_url: &str = "https://mongodb.com";
pub const port: u16 = 4000;
pub const database_name: &str = "obverse";
