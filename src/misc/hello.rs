use axum::Json;
use serde_json::json;

pub async fn get_hello() -> Json<serde_json::Value> {
    Json(json!({
        "status": "OK",
        "message": "This is the booklyst backend"
    }))
}
