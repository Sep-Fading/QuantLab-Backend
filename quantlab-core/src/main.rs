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
    let pool = match PgPool::connect(&db_url).await {
        Ok(_pool) => {
            println!("✅ Successfully connected to QuantLab DB!");
            _pool
        }
        Err(e) => {
            println!("❌Failed to connect to QuantLab DB: {}!", e);
            return;
        }
    };

    // Run migrations
    println!("🔁 Running database migrations...");
    match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(_) => println!("✅Migrations complete!"),
        Err(e) => println!("⚠️Migrations error: {}", e),
    }

    // Verify tables exist
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM STOCK_PRICES")
        .fetch_one(&pool)
        .await
        .unwrap_or((0,));

    println!("📊Stock prices table has {} records", count.0);
}
