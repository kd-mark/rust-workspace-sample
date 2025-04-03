use std::time::Instant;

use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};

use crate::{
    helpers::logger::{DefaultLogger, Logger},
    App,
};

// Custom middleware function
pub async fn log_requests(req: Request, next: Next) -> Result<Response, StatusCode> {
    let logger = DefaultLogger::new::<App>();

    let start = Instant::now();
    let method = req.method().clone();
    let uri = req.uri().clone();

    let response = next.run(req).await; // Call the next handler in the chain

    let duration = start.elapsed();
    logger.debug(&format!("{} {} - Took {:?}", method, uri, duration));

    Ok(response)
}
