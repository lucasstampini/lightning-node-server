use sqlx::{PgPool};
use dotenv::dotenv;

mod services;
// Import the get_nodes function from the services module
use services::{import_nodes, export_nodes};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    // Load environment variables (DATABASE_URL) from .env file into process environment
    dotenv().ok();
    // Read the database URL from the environment
    let database_url = std::env::var("DATABASE_URL")?;
     // Establish a connection pool to the PostgreSQL database
    let pool = PgPool::connect(&database_url).await?;
    let pool_clone_for_import = pool.clone();

    // Spawn an asynchronous task to run the import_nodes function concurrently
    tokio::spawn(async move {
    if let Err(e) = import_nodes(pool.clone()).await {
            eprintln!("Erro in import_nodes: {:?}", e);
        }
    });

    // Run export_nodes
    if let Err(e) = export_nodes(pool_clone_for_import.clone()).await {
            eprintln!("Erro in export_nodes: {:?}", e);
    }

    // Prevent the program from exiting immediately by sleeping in an infinite loop
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}
