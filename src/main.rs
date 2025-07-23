mod config;
mod models;
use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use config::database::{establish_connection, mongourl_url, port};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;
use tracing_subscriber;
// use config

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();
    let mongo_uri = env::var("MONGO_URL").expect("API KEY NOT FOUND in .env");

    let db = establish_connection(&mongo_uri);

    tracing::info!("Axum API");
    tracing::warn!("Starting");
    println!("Hello, world!");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    println!("Server running on http://{}", addr);
    println!("Connected to MongoDB database:");

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap()
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };
    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Deserialize, Serialize)]
struct User {
    id: u64,
    username: String,
}
