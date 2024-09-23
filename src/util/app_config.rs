#[derive(Debug, Clone)]
pub struct AppConfig {
    pub server_address: String,
    pub db_url: String,
    pub jwt_secret: String,
}

impl AppConfig {
    pub fn init() -> AppConfig {
        AppConfig {
            server_address: std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_owned()),
            db_url: std::env::var("DATABASE_URL")
                .expect("DATABASE_URL missing from the .env file."),
            jwt_secret: std::env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
        }
    }
}
