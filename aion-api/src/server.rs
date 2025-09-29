use axum::{Router, routing::get};
use aion_core::AionResult;

pub struct ApiServer {
    port: u16,
    host: String,
}

impl ApiServer {
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }

    pub async fn start(self) -> AionResult<()> {
        let app = Router::new()
            .route("/health", get(health_check));

        let listener = tokio::net::TcpListener::bind(format!("{}:{}", self.host, self.port))
            .await
            .map_err(|e| aion_core::AionError::NetworkError {
                operation: "bind".to_string(),
                reason: e.to_string(),
            })?;

        tracing::info!("Server starting on {}:{}", self.host, self.port);

        axum::serve(listener, app)
            .await
            .map_err(|e| aion_core::AionError::NetworkError {
                operation: "serve".to_string(),
                reason: e.to_string(),
            })?;

        Ok(())
    }
}

async fn health_check() -> &'static str {
    "OK"
}