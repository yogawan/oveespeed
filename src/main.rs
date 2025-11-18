use axum::{
    Router, Server,
    extract::State,
    routing::{get, post},
};
use std::net::SocketAddr;

mod db;
mod handlers;
mod models;

use db::MongoRepo;
use handlers::user_handler::{create_user, get_user_by_id, get_users};

#[tokio::main]
async fn main() {
    let repo = MongoRepo::init().await;

    let app = Router::new()
        .route("/users", post(create_user))
        .route("/users", get(get_users))
        .route("/users/:id", get(get_user_by_id))
        .with_state(repo);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    // pakai Server langsung karena sudah di import
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
