use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};

pub async fn auth_guard(req: Request, next: Next) -> Result<Response, StatusCode> {
    let headers = req.headers();
    if let Some(auth_header) = headers.get("Authorization") {
        if auth_header == "Bearer my_secret_token" {
            return Ok(next.run(req).await);
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}
