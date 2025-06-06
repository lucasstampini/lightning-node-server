use serde::{Serialize, Deserialize};
use chrono::{DateTime, TimeZone, Utc};
use tokio::time::{interval, Duration};
use sqlx::{PgPool, Row};

// Defines the Node struct, which represents a Lightning Network node retrieved from the external API.
// The server only extracts the relevant fields: public key, alias, capacity, and first seen timestamp.
#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    #[serde(rename = "publicKey")]
    public_key: String,
    alias: String,
    capacity: i64,
    #[serde(rename = "firstSeen")]
    first_seen: i64,
}

// Represents a Node for exporting data, with all fields as Strings to match the database structure (which stores them as text)
#[derive(Debug, Serialize, Deserialize)]
pub struct NodeExport {
    #[serde(rename = "publicKey")]
    public_key: String,
    alias: String,
    capacity: String,
    #[serde(rename = "firstSeen")]
    first_seen: String,
}

impl Node {
    pub fn public_key(&self) -> &str {
        &self.public_key
    }

    pub fn alias(&self) -> &str {
        &self.alias
    }

    // Converts capacity from sats to Bitcoins as a string
    pub fn capacity(&self) -> String {
        let value = self.capacity as f64 / 100_000_000.0;
        value.to_string()
    }

    // Converts the Unix timestamp to the ISO 8601 format (e.g., "2020-09-30T01:39:00Z")
    pub fn first_seen(&self) -> String {
        let datetime: DateTime<Utc> = Utc.timestamp_opt(self.first_seen, 0).unwrap();
        datetime.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
    }
}

// Fetches node data from the External API and deserializes it into a vector of Node
pub async fn get_nodes() -> Result<Vec<Node>, reqwest::Error> {
    let nodes: Vec<Node> = reqwest::Client::new()
    .get("https://mempool.space/api/v1/lightning/nodes/rankings/connectivity")
    .send()
    .await?
    .json()
    .await?;

    Ok(nodes)
}

// Periodically imports nodes from the external API into the PostgreSQL database
pub async fn import_nodes(pool: PgPool) -> Result<(), Box<dyn std::error::Error + Send + Sync>>{
    // Create an interval that ticks every 600 seconds (10 minutes)
    let mut interval = interval(Duration::from_secs(10));

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

// Exports all nodes from the database by querying and printing them as pretty JSON
pub async fn export_nodes(pool: PgPool) -> Result<(), Box<dyn std::error::Error>>{
    let rows = sqlx::query(
    "SELECT public_key, alias, capacity, first_seen FROM node"
    )
    .fetch_all(&pool)
    .await?;

    let mut nodes = Vec::new();

    // Convert each database row into a NodeExport struct
    for row in rows {
        let node = NodeExport {
            public_key: row.try_get("public_key")?,
            alias: row.try_get("alias")?,
            capacity: row.try_get("capacity")?,
            first_seen: row.try_get("first_seen")?,
        };
        nodes.push(node);
    }

    // Serialize the vector of nodes to a pretty JSON string
    let json = serde_json::to_string_pretty(&nodes)?;
    println!("{}", json);

    Ok(())
}