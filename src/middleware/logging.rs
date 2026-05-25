use axum::{
    body::Body,
    extract::Request,
    middleware::Next,
    response::Response,
};
use std::time::Instant;
use tracing::{info, span, Level};

pub async fn log_requests(request: Request<Body>, next: Next) -> Response {
    let start = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone(); 

    let request_span = span!(
        Level::INFO,
        "http_request",
        method = %method,
        uri = %uri
    );

    let _enter = request_span.enter();

    info!("started processing request");

    let response = next.run(request).await;

    let latency = start.elapsed(); 
    let status = response.status();

    info!(
        status = %status.as_u16(),
        latency = ?latency,
        "finished processing request"
    );

    response
    
}
