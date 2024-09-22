use misc::misc_router;
use tokio::net::TcpListener;

use axum::{Extension, Router};
use sea_orm::{ConnectOptions, Database};
use user::user_router;

mod handlers;
mod models;
mod routes;
mod util;
use crate::routes::*;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Unable to access .env file,");

    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_owned());
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL missing from the .env file.");
    let _ = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let mut opt = ConnectOptions::new(&db_url);
    opt.max_connections(20);

    let db = Database::connect(opt)
        .await
        .expect("connection to database failed.");

    let listener = TcpListener::bind(&server_address).await.expect(&format!(
        "failed to create TCP listener at port {:?}.",
        &server_address.split(":").last(),
    ));

    println!("listening on {}", listener.local_addr().unwrap());

    let app = Router::new()
        .merge(misc_router())
        .nest("/user", user_router())
        .layer(Extension(db));

    axum::serve(listener, app)
        .await
        .expect("error running the server.");
}
