use axum::Json;
use serde_json::json;

pub async fn get_health_handler() -> Json<serde_json::Value> {
    Json(json!({
        "status": "OK",
        "message": "Server is healthy"
    }))
}

pub async fn get_hello() -> Json<serde_json::Value> {
    Json(json!({
        "status": "OK",
        "message": "This is the booklyst backend"
    }))
}
