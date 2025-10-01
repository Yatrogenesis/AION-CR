/*!
 * Connector Registry
 *
 * Central registry for managing all API connectors with automatic discovery,
 * health monitoring, and load balancing capabilities.
 */

use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use anyhow::Result;
use tracing::{info, warn, error, debug};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::*;
use crate::authentication::AuthenticationManager;
use crate::rate_limiting::RateLimitingService;
use crate::cache::CacheLayer;
use crate::monitoring::MarketplaceMonitor;

/// Connector Registry
///
/// Manages the lifecycle of all API connectors including registration,
/// health monitoring, load balancing, and automatic failover.
pub struct ConnectorRegistry {
    /// Registry configuration
    config: Arc<crate::MarketplaceConfig>,

    /// Registered connectors
    connectors: Arc<RwLock<HashMap<String, Arc<dyn ApiConnector>>>>,

    /// Connector configurations
    connector_configs: Arc<RwLock<HashMap<String, ConnectorConfig>>>,

    /// Connector health status
    health_status: Arc<RwLock<HashMap<String, HealthStatus>>>,

    /// Authentication manager
    auth_manager: Arc<AuthenticationManager>,

    /// Rate limiting service
    rate_limiter: Arc<RateLimitingService>,

    /// Cache layer
    cache: Arc<CacheLayer>,

    /// Monitoring service
    monitor: Arc<MarketplaceMonitor>,

    /// Active connections tracker
    active_connections: Arc<RwLock<HashMap<String, ConnectionMetrics>>>,

    /// Load balancer
    load_balancer: Arc<LoadBalancer>,

    /// Health monitor
    health_monitor: Arc<HealthMonitor>,
}

/// Connection metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionMetrics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time: chrono::Duration,
    pub last_request: DateTime<Utc>,
    pub connection_start: DateTime<Utc>,
}

/// Registry status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorRegistryStatus {
    pub total_connectors: u32,
    pub active_connectors: u32,
    pub healthy_connectors: u32,
    pub failed_connectors: u32,
    pub total_requests: u64,
    pub requests_per_minute: u64,
    pub average_response_time: chrono::Duration,
    pub last_health_check: DateTime<Utc>,
}

impl ConnectorRegistry {
    /// Create a new connector registry
    pub async fn new(
        config: Arc<crate::MarketplaceConfig>,
        auth_manager: Arc<AuthenticationManager>,
        rate_limiter: Arc<RateLimitingService>,
        cache: Arc<CacheLayer>,
        monitor: Arc<MarketplaceMonitor>,
    ) -> Result<Self> {
        info!("🏗️ Creating connector registry");

        let load_balancer = Arc::new(LoadBalancer::new().await?);
        let health_monitor = Arc::new(HealthMonitor::new().await?);

        Ok(Self {
            config,
            connectors: Arc::new(RwLock::new(HashMap::new())),
            connector_configs: Arc::new(RwLock::new(HashMap::new())),
            health_status: Arc::new(RwLock::new(HashMap::new())),
            auth_manager,
            rate_limiter,
            cache,
            monitor,
            active_connections: Arc::new(RwLock::new(HashMap::new())),
            load_balancer,
            health_monitor,
        })
    }

    /// Start the connector registry
    pub async fn start(&self) -> Result<()> {
        info!("🚀 Starting connector registry");

        // Start load balancer
        self.load_balancer.start().await?;

        // Start health monitor
        self.health_monitor.start(
            self.connectors.clone(),
            self.health_status.clone(),
        ).await?;

        info!("✅ Connector registry started");
        Ok(())
    }

    /// Stop the connector registry
    pub async fn stop(&self) -> Result<()> {
        info!("🛑 Stopping connector registry");

        // Stop health monitor
        self.health_monitor.stop().await?;

        // Stop load balancer
        self.load_balancer.stop().await?;

        // Stop all connectors
        let connectors = self.connectors.read().await;
        for (id, connector) in connectors.iter() {
            if let Err(e) = self.stop_connector(id).await {
                warn!("Failed to stop connector {}: {}", id, e);
            }
        }

        info!("✅ Connector registry stopped");
        Ok(())
    }

    /// Register a new connector
    pub async fn register_connector(&self, config: ConnectorConfig) -> Result<String> {
        info!("📝 Registering connector: {}", config.name);

        let connector_id = config.id.clone();

        // Store configuration
        {
            let mut configs = self.connector_configs.write().await;
            configs.insert(connector_id.clone(), config.clone());
        }

        // Create and initialize connector
        let connector = self.create_connector(config).await?;

        // Test connection
        let connection_test = connector.test_connection().await;
        match connection_test {
            Ok(true) => info!("✅ Connector {} connection test passed", connector_id),
            Ok(false) => warn!("⚠️ Connector {} connection test failed", connector_id),
            Err(e) => warn!("❌ Connector {} connection test error: {}", connector_id, e),
        }

        // Register connector
        {
            let mut connectors = self.connectors.write().await;
            connectors.insert(connector_id.clone(), connector);
        }

        // Initialize health status
        {
            let mut health = self.health_status.write().await;
            health.insert(connector_id.clone(), HealthStatus {
                healthy: connection_test.unwrap_or(false),
                response_time: chrono::Duration::seconds(0),
                last_check: Utc::now(),
                error_message: None,
            });
        }

        // Initialize connection metrics
        {
            let mut connections = self.active_connections.write().await;
            connections.insert(connector_id.clone(), ConnectionMetrics {
                total_requests: 0,
                successful_requests: 0,
                failed_requests: 0,
                average_response_time: chrono::Duration::seconds(0),
                last_request: Utc::now(),
                connection_start: Utc::now(),
            });
        }

        // Add to load balancer
        self.load_balancer.add_connector(&connector_id).await?;

        info!("✅ Connector registered: {}", connector_id);
        Ok(connector_id)
    }

    /// Unregister a connector
    pub async fn unregister_connector(&self, connector_id: &str) -> Result<()> {
        info!("🗑️ Unregistering connector: {}", connector_id);

        // Remove from load balancer
        self.load_balancer.remove_connector(connector_id).await?;

        // Stop connector
        self.stop_connector(connector_id).await?;

        // Remove from registry
        {
            let mut connectors = self.connectors.write().await;
            connectors.remove(connector_id);
        }

        {
            let mut configs = self.connector_configs.write().await;
            configs.remove(connector_id);
        }

        {
            let mut health = self.health_status.write().await;
            health.remove(connector_id);
        }

        {
            let mut connections = self.active_connections.write().await;
            connections.remove(connector_id);
        }

        info!("✅ Connector unregistered: {}", connector_id);
        Ok(())
    }

    /// Execute a request through a connector
    pub async fn execute_request(
        &self,
        connector_id: &str,
        endpoint: &str,
        parameters: crate::ApiParameters,
    ) -> Result<ApiResponse> {
        debug!("📡 Executing request to connector: {} endpoint: {}", connector_id, endpoint);

        // Check rate limits
        self.rate_limiter.check_rate_limit(connector_id).await?;

        // Check cache first
        let cache_key = format!("{}:{}:{:?}", connector_id, endpoint, parameters);
        if let Some(cached_response) = self.cache.get(&cache_key).await? {
            debug!("💾 Returning cached response for {}", connector_id);
            return Ok(cached_response);
        }

        // Get connector
        let connector = {
            let connectors = self.connectors.read().await;
            connectors.get(connector_id)
                .ok_or_else(|| anyhow::anyhow!("Connector not found: {}", connector_id))?
                .clone()
        };

        // Execute request
        let start_time = Utc::now();
        let result = connector.execute_request(endpoint, &parameters).await;
        let response_time = Utc::now().signed_duration_since(start_time);

        // Update metrics
        self.update_connection_metrics(connector_id, &result, response_time).await?;

        // Handle result
        match result {
            Ok(response) => {
                // Cache successful response if configured
                if let Some(ttl) = self.get_cache_ttl(connector_id, endpoint).await? {
                    self.cache.set(&cache_key, &response, ttl).await?;
                }

                debug!("✅ Request successful for connector: {}", connector_id);
                Ok(response)
            }
            Err(e) => {
                error!("❌ Request failed for connector: {} error: {}", connector_id, e);

                // Update health status
                self.update_health_status(connector_id, false, Some(e.to_string())).await?;

                Err(e)
            }
        }
    }

    /// Search connectors
    pub async fn search_connectors(&self, query: crate::ApiSearchQuery) -> Result<Vec<crate::ApiSearchResult>> {
        debug!("🔍 Searching connectors with query: {:?}", query);

        let configs = self.connector_configs.read().await;
        let health = self.health_status.read().await;

        let mut results = Vec::new();

        for (id, config) in configs.iter() {
            let mut matches = true;

            // Filter by keyword
            if let Some(keyword) = &query.keyword {
                if !config.name.to_lowercase().contains(&keyword.to_lowercase()) &&
                   !config.description.to_lowercase().contains(&keyword.to_lowercase()) {
                    matches = false;
                }
            }

            // Filter by category
            if let Some(category) = &query.category {
                if format!("{:?}", config.category).to_lowercase() != category.to_lowercase() {
                    matches = false;
                }
            }

            // Filter by jurisdiction
            if let Some(jurisdiction) = &query.jurisdiction {
                if !config.metadata.jurisdiction.iter()
                    .any(|j| j.to_lowercase().contains(&jurisdiction.to_lowercase())) {
                    matches = false;
                }
            }

            // Filter by compliance framework
            if let Some(framework) = &query.compliance_framework {
                if !config.metadata.compliance_frameworks.iter()
                    .any(|f| f.to_lowercase().contains(&framework.to_lowercase())) {
                    matches = false;
                }
            }

            // Filter by real-time support
            if let Some(real_time) = query.real_time {
                let supports_real_time = config.webhook_config
                    .as_ref()
                    .map(|wh| wh.supported)
                    .unwrap_or(false);
                if real_time && !supports_real_time {
                    matches = false;
                }
            }

            if matches {
                let health_status = health.get(id).cloned().unwrap_or(HealthStatus {
                    healthy: false,
                    response_time: chrono::Duration::seconds(0),
                    last_check: Utc::now(),
                    error_message: Some("Not initialized".to_string()),
                });

                results.push(crate::ApiSearchResult {
                    connector_id: id.clone(),
                    name: config.name.clone(),
                    description: config.description.clone(),
                    category: config.category.clone(),
                    jurisdiction: config.metadata.jurisdiction.clone(),
                    compliance_frameworks: config.metadata.compliance_frameworks.clone(),
                    real_time_support: config.webhook_config
                        .as_ref()
                        .map(|wh| wh.supported)
                        .unwrap_or(false),
                    documentation_url: config.metadata.documentation_url.clone(),
                    health_status,
                });
            }
        }

        debug!("✅ Found {} matching connectors", results.len());
        Ok(results)
    }

    /// Get connector count
    pub async fn get_connector_count(&self) -> Result<u32> {
        let connectors = self.connectors.read().await;
        Ok(connectors.len() as u32)
    }

    /// Get active connections count
    pub async fn get_active_connections(&self) -> Result<u32> {
        let health = self.health_status.read().await;
        let active = health.values().filter(|h| h.healthy).count();
        Ok(active as u32)
    }

    /// Get registry status
    pub async fn get_status(&self) -> Result<ConnectorRegistryStatus> {
        let connectors = self.connectors.read().await;
        let health = self.health_status.read().await;
        let connections = self.active_connections.read().await;

        let total_connectors = connectors.len() as u32;
        let healthy_connectors = health.values().filter(|h| h.healthy).count() as u32;
        let failed_connectors = total_connectors - healthy_connectors;

        let total_requests: u64 = connections.values().map(|c| c.total_requests).sum();
        let successful_requests: u64 = connections.values().map(|c| c.successful_requests).sum();

        let average_response_time = if !connections.is_empty() {
            let total_duration: i64 = connections.values()
                .map(|c| c.average_response_time.num_milliseconds())
                .sum();
            chrono::Duration::milliseconds(total_duration / connections.len() as i64)
        } else {
            chrono::Duration::seconds(0)
        };

        // Calculate requests per minute (simplified)
        let requests_per_minute = connections.values()
            .filter(|c| Utc::now().signed_duration_since(c.last_request) < chrono::Duration::minutes(1))
            .count() as u64;

        Ok(ConnectorRegistryStatus {
            total_connectors,
            active_connectors: healthy_connectors,
            healthy_connectors,
            failed_connectors,
            total_requests,
            requests_per_minute,
            average_response_time,
            last_health_check: Utc::now(),
        })
    }

    /// Bulk import from source
    pub async fn bulk_import_from_source(&self, source: crate::BulkImportSource) -> Result<crate::BulkImportResult> {
        info!("📦 Starting bulk import from connector: {}", source.connector_id);

        let start_time = Utc::now();
        let mut total_records = 0u64;
        let mut successful_imports = 0u64;
        let mut failed_imports = 0u64;
        let mut errors = Vec::new();

        // Get connector
        let connector = {
            let connectors = self.connectors.read().await;
            connectors.get(&source.connector_id)
                .ok_or_else(|| anyhow::anyhow!("Connector not found: {}", source.connector_id))?
                .clone()
        };

        // Execute bulk import in batches
        let mut batch_start = 0;
        loop {
            let params = crate::ApiParameters::from([
                ("limit".to_string(), serde_json::Value::Number(source.import_config.batch_size.into())),
                ("offset".to_string(), serde_json::Value::Number(batch_start.into())),
            ]);

            match connector.execute_request(&source.data_source, &params).await {
                Ok(response) => {
                    // Process response data
                    if let Some(data) = response.body.as_array() {
                        if data.is_empty() {
                            break; // No more data
                        }

                        total_records += data.len() as u64;
                        successful_imports += data.len() as u64;
                        batch_start += source.import_config.batch_size;

                        // Store data (implementation would go here)
                        info!("📊 Imported batch of {} records", data.len());
                    } else {
                        break;
                    }
                }
                Err(e) => {
                    failed_imports += source.import_config.batch_size as u64;
                    errors.push(format!("Batch {}: {}", batch_start, e));

                    if errors.len() >= source.import_config.retry_attempts as usize {
                        break;
                    }
                }
            }
        }

        let duration = Utc::now().signed_duration_since(start_time);

        let result = crate::BulkImportResult {
            total_records,
            successful_imports,
            failed_imports,
            duration_seconds: duration.num_seconds() as u64,
            errors,
        };

        info!("✅ Bulk import completed: {} records in {} seconds",
              successful_imports, result.duration_seconds);

        Ok(result)
    }

    /// Sync all data
    pub async fn sync_all_data(&self) -> Result<crate::SyncResult> {
        info!("🔄 Starting data synchronization for all connectors");

        let start_time = Utc::now();
        let mut synced_apis = 0u32;
        let mut failed_syncs = 0u32;
        let mut total_records_synced = 0u64;
        let mut errors = Vec::new();

        let connector_ids: Vec<String> = {
            let connectors = self.connectors.read().await;
            connectors.keys().cloned().collect()
        };

        for connector_id in connector_ids {
            match self.sync_connector_data(&connector_id).await {
                Ok(records) => {
                    synced_apis += 1;
                    total_records_synced += records;
                    info!("✅ Synced {} records from connector: {}", records, connector_id);
                }
                Err(e) => {
                    failed_syncs += 1;
                    errors.push(format!("{}: {}", connector_id, e));
                    warn!("❌ Failed to sync connector: {} error: {}", connector_id, e);
                }
            }
        }

        let sync_duration = Utc::now().signed_duration_since(start_time);

        Ok(crate::SyncResult {
            synced_apis,
            failed_syncs,
            total_records_synced,
            sync_duration,
            errors,
        })
    }

    /// Private helper methods

    async fn create_connector(&self, config: ConnectorConfig) -> Result<Arc<dyn ApiConnector>> {
        // Create a concrete connector implementation
        let connector = StandardApiConnector::new(
            config,
            self.auth_manager.clone(),
            self.rate_limiter.clone(),
        ).await?;

        Ok(Arc::new(connector))
    }

    async fn stop_connector(&self, connector_id: &str) -> Result<()> {
        // Implementation would stop the specific connector
        debug!("🛑 Stopping connector: {}", connector_id);
        Ok(())
    }

    async fn update_connection_metrics(
        &self,
        connector_id: &str,
        result: &Result<ApiResponse>,
        response_time: chrono::Duration,
    ) -> Result<()> {
        let mut connections = self.active_connections.write().await;
        if let Some(metrics) = connections.get_mut(connector_id) {
            metrics.total_requests += 1;
            metrics.last_request = Utc::now();

            match result {
                Ok(_) => metrics.successful_requests += 1,
                Err(_) => metrics.failed_requests += 1,
            }

            // Update average response time (simple moving average)
            let total_time = metrics.average_response_time.num_milliseconds() * (metrics.total_requests - 1) as i64
                + response_time.num_milliseconds();
            metrics.average_response_time = chrono::Duration::milliseconds(total_time / metrics.total_requests as i64);
        }

        Ok(())
    }

    async fn update_health_status(
        &self,
        connector_id: &str,
        healthy: bool,
        error_message: Option<String>,
    ) -> Result<()> {
        let mut health = self.health_status.write().await;
        if let Some(status) = health.get_mut(connector_id) {
            status.healthy = healthy;
            status.last_check = Utc::now();
            status.error_message = error_message;
        }

        Ok(())
    }

    async fn get_cache_ttl(&self, connector_id: &str, endpoint: &str) -> Result<Option<chrono::Duration>> {
        let configs = self.connector_configs.read().await;
        if let Some(config) = configs.get(connector_id) {
            for endpoint_config in &config.endpoints {
                if endpoint_config.path.contains(endpoint) || endpoint_config.name == endpoint {
                    return Ok(endpoint_config.cache_ttl);
                }
            }
        }
        Ok(None)
    }

    async fn sync_connector_data(&self, connector_id: &str) -> Result<u64> {
        // Implementation would sync data from the specific connector
        // This is a simplified placeholder
        debug!("🔄 Syncing data from connector: {}", connector_id);

        // Simulate sync operation
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        Ok(100) // Placeholder: return number of synced records
    }
}

/// Load balancer for distributing requests across connectors
pub struct LoadBalancer {
    connector_weights: Arc<RwLock<HashMap<String, f64>>>,
}

impl LoadBalancer {
    async fn new() -> Result<Self> {
        Ok(Self {
            connector_weights: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    async fn start(&self) -> Result<()> {
        debug!("🚀 Starting load balancer");
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        debug!("🛑 Stopping load balancer");
        Ok(())
    }

    async fn add_connector(&self, connector_id: &str) -> Result<()> {
        let mut weights = self.connector_weights.write().await;
        weights.insert(connector_id.to_string(), 1.0);
        debug!("➕ Added connector to load balancer: {}", connector_id);
        Ok(())
    }

    async fn remove_connector(&self, connector_id: &str) -> Result<()> {
        let mut weights = self.connector_weights.write().await;
        weights.remove(connector_id);
        debug!("➖ Removed connector from load balancer: {}", connector_id);
        Ok(())
    }
}

/// Health monitor for checking connector status
pub struct HealthMonitor {
    monitoring: Arc<RwLock<bool>>,
}

impl HealthMonitor {
    async fn new() -> Result<Self> {
        Ok(Self {
            monitoring: Arc::new(RwLock::new(false)),
        })
    }

    async fn start(
        &self,
        _connectors: Arc<RwLock<HashMap<String, Arc<dyn ApiConnector>>>>,
        _health_status: Arc<RwLock<HashMap<String, HealthStatus>>>,
    ) -> Result<()> {
        let mut monitoring = self.monitoring.write().await;
        *monitoring = true;

        debug!("🚀 Starting health monitor");

        // In a real implementation, this would start a background task
        // to periodically check connector health

        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        let mut monitoring = self.monitoring.write().await;
        *monitoring = false;

        debug!("🛑 Stopping health monitor");
        Ok(())
    }
}

/// Standard API connector implementation
pub struct StandardApiConnector {
    config: ConnectorConfig,
    auth_manager: Arc<AuthenticationManager>,
    rate_limiter: Arc<RateLimitingService>,
    client: reqwest::Client,
}

impl StandardApiConnector {
    async fn new(
        config: ConnectorConfig,
        auth_manager: Arc<AuthenticationManager>,
        rate_limiter: Arc<RateLimitingService>,
    ) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        Ok(Self {
            config,
            auth_manager,
            rate_limiter,
            client,
        })
    }
}

#[async_trait]
impl ApiConnector for StandardApiConnector {
    fn get_config(&self) -> &ConnectorConfig {
        &self.config
    }

    async fn test_connection(&self) -> Result<bool> {
        // Implement connection test logic
        Ok(true)
    }

    async fn execute_request(
        &self,
        endpoint: &str,
        parameters: &crate::ApiParameters,
    ) -> Result<ApiResponse> {
        // Implementation would execute the actual HTTP request
        // This is a simplified placeholder

        let start_time = Utc::now();

        // Build URL
        let url = format!("{}{}", self.config.base_url, endpoint);

        // Make request (simplified)
        let response = self.client
            .get(&url)
            .query(parameters)
            .send()
            .await?;

        let status_code = response.status().as_u16();
        let headers: HashMap<String, String> = response.headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();

        let body: serde_json::Value = response.json().await.unwrap_or(serde_json::Value::Null);

        let response_time = Utc::now().signed_duration_since(start_time);

        Ok(ApiResponse {
            status_code,
            headers,
            body,
            response_time,
            timestamp: Utc::now(),
            cached: false,
        })
    }

    async fn subscribe_to_updates(
        &self,
        _event_types: Vec<String>,
        _callback_url: String,
    ) -> Result<String> {
        // Implementation would handle webhook subscriptions
        Ok(Uuid::new_v4().to_string())
    }

    async fn unsubscribe(&self, _subscription_id: &str) -> Result<()> {
        // Implementation would handle webhook unsubscription
        Ok(())
    }

    fn get_endpoints(&self) -> &[EndpointConfig] {
        &self.config.endpoints
    }

    async fn get_health(&self) -> Result<HealthStatus> {
        // Implementation would check connector health
        Ok(HealthStatus {
            healthy: true,
            response_time: chrono::Duration::seconds(1),
            last_check: Utc::now(),
            error_message: None,
        })
    }

    async fn refresh_auth(&self) -> Result<()> {
        // Implementation would refresh authentication tokens
        Ok(())
    }
}