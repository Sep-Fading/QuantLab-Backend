use yahoo_finance_api as yahoo;
use time::{OffsetDateTime, Duration};
use sqlx::{PgPool};

// Fetches and inserts data into the database for a given 
// stock (*symbol*) for the past given days (*days*).
pub async fn fetch_and_store_stock_data(
    pool: &PgPool,
    symbol: &str,
    days: i64,
) -> Result<usize, Box<dyn std::error::Error>> {
    println!("📥 Fetching data for {}...", symbol);
    
    let provider = yahoo::YahooConnector::new()?;

    // Historical Period from now going back *&days* days.
    let end = time::OffsetDateTime::now_utc();
    let start: OffsetDateTime = end - Duration::days(days);

    println!("📅 From {} to {}.", start.date(), end.date());

    // Fetch Data
    let response = match provider.get_quote_history(symbol, start, end).await {
        Ok(resp) => Some(resp),
        Err(err) => {
            println!("❌Failed to fetch quote history for {}: {}", symbol, err);
            None
        }
    };
    
    // Insert the fetched data into database.
    let mut rows_inserted: usize = 0;

    match response {
        Some(resp) => {
            // Process response
            let quotes = resp.quotes()?;

            if quotes.is_empty(){
                println!("❌No data found for {}", symbol);
            }


            for quote in quotes {
                let timestamp = match OffsetDateTime::from_unix_timestamp(quote.timestamp) {
                    Ok(ts) => ts,
                    Err(err) => {
                        eprintln!("⚠️  Skipping invalid timestamp {}: {}", quote.timestamp, err);
                        continue;
                    }
                };
                
                let results = sqlx::query::<sqlx::Postgres>(
                    r#"
                    INSERT INTO STOCK_PRICES (TIME, SYMBOL, OPEN, HIGH, LOW, CLOSE, VOLUME)
                    VALUES ($1, $2, $3, $4, $5, $6, $7)
                    ON CONFLICT DO NOTHING
                    "#
                )
                .bind(timestamp)
                .bind(symbol)
                .bind(quote.open)
                .bind(quote.high)
                .bind(quote.low)
                .bind(quote.close)
                .bind(quote.volume as i64)
                .execute(pool)
                .await;

                match results {
                    Ok(_) => rows_inserted += 1,
                    Err(e) => eprintln!("⚠️Failed to insert row: {}", e),
                };
            }

            println!("✅Successfully inserted {} rows into the database for {}!", rows_inserted, symbol);

        }
        None => {
            return Ok(0);
        }
    }

    Ok(rows_inserted)
}
