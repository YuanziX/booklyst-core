use axum::Router;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tokio::net::TcpListener;

mod middleware;
mod misc;
mod user;
mod book;
mod util;

use util::app_config::AppConfig;

#[derive(Clone)]
struct AppState {
    db: DatabaseConnection,
    app_config: AppConfig,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Unable to access .env file");

    let app_config = AppConfig::init();

    let mut opt = ConnectOptions::new(&app_config.db_url);
    opt.max_connections(20);

    let db = Database::connect(opt)
        .await
        .expect("connection to database failed.");

    let listener = TcpListener::bind(&app_config.server_address)
        .await
        .expect(&format!(
            "failed to create TCP listener at port {:?}.",
            &app_config.server_address.split(":").last(),
        ));

    println!("listening on {}", listener.local_addr().unwrap());

    let app_state = AppState { db, app_config };

    let app = Router::new()
        .nest("/user", user::router::user_router(&app_state))
        .nest("/book", book::router::book_router(&app_state))
        .with_state(app_state)
        .merge(misc::router::misc_router());

    axum::serve(listener, app)
        .await
        .expect("error running the server.");
}
