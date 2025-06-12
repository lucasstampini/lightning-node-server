# rust-lightning-server

## Build Tools & Versions Used

### Rust
- **Rust**: 1.87.0

### Rust Dependencies
- `reqwest` = "0.12" (features: `json`)
- `tokio` = "1.38" (features: `full`)
- `serde` = "1.0" (features: `derive`)
- `actix` = "0.13.5"
- `actix-web` = "4.11.0"
- `dotenv` = "0.15.0"
- `sqlx` = "0.8.6" (features: `runtime-async-std-native-tls`, `postgres`)
- `chrono` = "0.4.41"
- `serde_json` = "1.0.140"

### Development Tools
- **IDE**: Visual Studio Code 1.100
- **Database**: PostgreSQL 17.5

## Steps to Run the App

To run the app, follow these steps:

1. Clone the repository: git clone https://github.com/lucasstampini/lightning-node-server/
2. Open your PostgreSQL server (recommended version: 17.5) and create a database where the application's data will be stored.
3. Create the `.env` file based on the `.env.example` file provided. Replace `[DB]` with the name of the database you created.
4. Install all dependencies and build the application: `cargo build`
5. Start the application: `cargo run`
6. To test the app, go to the following endpoint: http://localhost:8080/nodes

## What was the reason for your focus? What problems were you trying to solve?

The main goal was to develop a server capable of serving data about nodes from the Lightning Network for an application. The server needed to use an external API to collect node data and store it in a database.

In addition, the server had to expose this data through a API endpoint `/nodes` which would return fields like:

- `public_key`: exactly as imported.
- `alias`: exactly as imported.
- `capacity`: converted from satoshis to Bitcoin.
- `first_seen`: formatted as a readable timestamp.

To ensure scalability and reliability, the project also required an import subroutine that would run periodically to fetch data from the external API and store it in the database. This separation meant that API requests to `/nodes` should only query the local database, not the external API directly.

The core challenge in this to me was handling data processing, data conversion (like capacity in satoshis to Bitcoin and timestamp formatting), and network interactions to ensure data was reliably fetched and served.

## How long did you spend on this project?

I spent a total of 8 hours on this project, divided over 4 days. This time was split between development and testing.

## Did you make any trade-offs for this project? What would you have done differently with more time?

Yes. Initially, I chose to use Axum to build the server, but I struggled to develop effectively with this dependency. Therefore, I decided to switch to Actix Web, which I found easier to work with. With more time, I could have dedicated more effort to implementing Axum in my code, despite finding it challenging.

## What do you think is the weakest part of your project?

As I mentioned before, I faced difficulties when creating the server because I initially chose to use Axum and later switched to Actix Web. So I think that is the weakest part of my project.

## Is there any other information you’d like us to know?

I would like to add that this challenge was excellent. I really enjoyed working on the problem, and the experience of developing this technical test was very rewarding. I appreciate the opportunity and look forward to any feedback you may have.
