use sqlx::{PgPool};
use dotenv::dotenv;
use actix_web::{HttpServer, web, App};
use std::env;

mod services;
// Import the import_nodes and export_handler function from the services module
use services::{import_nodes, export_handler};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables (DATABASE_URL) from .env file into process environment
    dotenv().ok();
    // Read the database URL from the environment
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    // Establish a connection pool to the PostgreSQL database
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");
    // Clone the pool for use in a background task
    let import_pool = pool.clone();

    // Spawn an asynchronous task to run the import_nodes function periodically
    tokio::spawn(async move {
        if let Err(e) = import_nodes(import_pool.clone()).await {
            eprintln!("Error in import_nodes: {:?}", e);
        }
    });

    println!("Server running at http://localhost:8080");

     // Start the Actix Web HTTP server
    HttpServer::new(move || {
        // Share the database pool with handlers via App state
        App::new()
            .app_data(web::Data::new(pool.clone()))
            // Register the GET /nodes route that triggers export_handler
            .route("/nodes", web::get().to(export_handler))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

