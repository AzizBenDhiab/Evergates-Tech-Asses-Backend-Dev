use std::sync::Arc;
use tokio_postgres::{Client, NoTls};
use std::env;
use dotenv::dotenv;

pub async fn init_db() -> Arc<Client> {
    dotenv().ok();

    let host = env::var("DATABASE_HOST").expect("DATABASE_HOST must be set");
    let user = env::var("DATABASE_USER").expect("DATABASE_USER must be set");
    let password = env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set");
    let dbname = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");

    let database_url = format!(
        "host={} user={} password={} dbname={}",
        host, user, password, dbname
    );

    let (client, connection) = tokio_postgres::connect(
       &database_url,NoTls)
    .await
    .expect("Failed to connect to database");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Arc::new(client)
}
