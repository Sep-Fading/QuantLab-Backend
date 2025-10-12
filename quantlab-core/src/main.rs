// Dependencies 
use sqlx::PgPool;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    println!("✅Successfully booted QuantLab Core!");

    // Load Env vars
    dotenv().ok();

    // Get DB url from the Env vars
    let db_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            println!("❌Error while retrieving Database URL.");
            println!("Ensure that you are fetching the correct fields from your .env file!");
            return;
        }
    };

    // Connect to DB
    match PgPool::connect(&db_url).await {
        Ok(_pool) => {
            println!("✅ Successfully connected to QuantLab DB!");
        }
        Err(e) => {
            println!("❌Failed to connect to QuantLab DB: {}!", e);
        }
    };
}
