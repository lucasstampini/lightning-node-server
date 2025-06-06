use tokio::time::{interval, Duration};
use sqlx::{PgPool};
use dotenv::dotenv;

mod services;
// Import the get_nodes function from the services module
use services::{get_nodes};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    // Load environment variables (DATABASE_URL) from .env file into process environment
    dotenv().ok();
    // Read the database URL from the environment
    let database_url = std::env::var("DATABASE_URL")?;
     // Establish a connection pool to the PostgreSQL database
    let pool = PgPool::connect(&database_url).await?;
    // Create an interval that ticks every 600 seconds (10 minutes)
    let mut interval = interval(Duration::from_secs(600));

    // Loop to continuously import data from the external API
    loop{
        interval.tick().await;
        match get_nodes().await {
            Ok(nodes) => {
                // Insert each node, skipping duplicates based on public_key
                for node in nodes {
                    sqlx::query(
                    "INSERT INTO node (public_key, alias, capacity, first_seen)
                    VALUES ($1, $2, $3, $4) ON CONFLICT (public_key) DO NOTHING;"
                    )
                    .bind(node.public_key())
                    .bind(node.alias())
                    .bind(node.capacity())
                    .bind(node.first_seen())
                    .execute(&pool)
                    .await?;
                }
            }
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }
}
