use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub struct ApiResponse {
    pub status: StatusCode,
    pub message: String,
    pub details: Option<String>,
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> axum::response::Response {
        let mut body = json!({
            "message": self.message,
        });

        if let Some(details) = self.details {
            body.as_object_mut()
                .unwrap()
                .insert("details".to_string(), json!(details));
        }

        let body = Json(body);
        (self.status, body).into_response()
    }
}
