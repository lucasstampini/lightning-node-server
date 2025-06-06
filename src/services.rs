use serde::{Serialize, Deserialize};
use chrono::{DateTime, TimeZone, Utc};

// Defines the Node struct, which represents a Lightning Network node retrieved from the external API.
// The server only extracts the relevant fields: public key, alias, capacity, and first seen timestamp.
#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    #[serde(rename = "publicKey")]
    public_key: String,
    alias: String,
    capacity: u64,
    #[serde(rename = "firstSeen")]
    first_seen: i64,
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