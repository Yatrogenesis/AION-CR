use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn logging_middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let method = request.method().clone();
    let uri = request.uri().clone();

    tracing::info!("Request: {} {}", method, uri);

    let response = next.run(request).await;

    tracing::info!("Response: {}", response.status());

    Ok(response)
}