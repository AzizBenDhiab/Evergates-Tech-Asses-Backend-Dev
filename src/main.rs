mod db;
mod handlers;
mod models;

use axum::{extract::Extension, routing::{get, post}, Router};
use std::net::SocketAddr;
use db::init_db;
use handlers::{get_details, post_details};
use dotenv::dotenv;


#[tokio::main]
async fn main() {
    let client = init_db().await;

    let app = Router::new()
        .route("/details", post(post_details))
        .route("/details", get(get_details))
        .layer(Extension(client));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
