//! AION-CR: AI-powered Regulatory Compliance Framework
//!
//! Complete implementation with ECTUS-R native integration and maximum autonomy

use std::sync::Arc;
use anyhow::Result;
use tracing::{info, error};
use uuid::Uuid;

// Re-export core modules
pub use aion_core::*;
pub use aion_api::*;

// Re-export integration module
pub use aion_integration::*;

/// AION-CR main system with ECTUS-R integration
pub struct AionCrSystem {
    pub system_id: Uuid,
    pub integration: Arc<aion_integration::AionEctusIntegration>,
    pub api_server: Arc<aion_api::ApiServer>,
    pub core_engine: Arc<aion_core::CoreEngine>,
    pub running: Arc<tokio::sync::RwLock<bool>>,
}

impl AionCrSystem {
    /// Initialize AION-CR system with native ECTUS-R integration and maximum autonomy
    pub async fn new_with_ectus_integration() -> Result<Self> {
        info!("🚀 Initializing AION-CR system with native ECTUS-R integration");

        let system_id = Uuid::new_v4();

        // Initialize native integration with maximum autonomy
        let integration = aion_integration::initialize_native_integration().await?;

        // Initialize API server
        let api_server = Arc::new(aion_api::ApiServer::new().await?);

        // Initialize core engine
        let core_engine = Arc::new(aion_core::CoreEngine::new().await?);

        let system = Self {
            system_id,
            integration,
            api_server,
            core_engine,
            running: Arc::new(tokio::sync::RwLock::new(false)),
        };

        info!("✅ AION-CR system initialized with ID: {}", system_id);
        Ok(system)
    }

    /// Start the unified AION-CR ↔ ECTUS-R system
    pub async fn start_unified_system(&self) -> Result<()> {
        info!("🎯 Starting unified AION-CR ↔ ECTUS-R system");

        // Mark system as running
        {
            let mut running = self.running.write().await;
            *running = true;
        }

        // Start the integration (this starts both systems)
        self.integration.start_unified_operation().await?;

        // Start API server
        self.api_server.start().await?;

        // Start core engine
        self.core_engine.start().await?;

        info!("🚀 Unified system operational - AION-CR ↔ ECTUS-R running as one");
        Ok(())
    }

    /// Check system health
    pub async fn health_check(&self) -> Result<SystemHealth> {
        let integration_health = self.integration.health_check().await?;
        let api_health = self.api_server.health_check().await?;
        let core_health = self.core_engine.health_check().await?;
        let running = *self.running.read().await;

        Ok(SystemHealth {
            system_id: self.system_id,
            running,
            integration_health,
            api_health,
            core_health,
            overall_status: if running &&
                integration_health.overall_status == aion_integration::HealthStatus::Healthy &&
                api_health.healthy &&
                core_health.healthy {
                HealthStatus::Healthy
            } else {
                HealthStatus::Degraded
            },
        })
    }

    /// Stop the system gracefully
    pub async fn stop(&self) -> Result<()> {
        info!("🛑 Stopping unified AION-CR ↔ ECTUS-R system");

        // Mark system as not running
        {
            let mut running = self.running.write().await;
            *running = false;
        }

        // Stop components in reverse order
        if let Err(e) = self.core_engine.stop().await {
            error!("Error stopping core engine: {}", e);
        }

        if let Err(e) = self.api_server.stop().await {
            error!("Error stopping API server: {}", e);
        }

        // Note: Integration stop would be handled by the integration module itself

        info!("✅ System stopped gracefully");
        Ok(())
    }
}

/// System health information
#[derive(Debug, Clone)]
pub struct SystemHealth {
    pub system_id: Uuid,
    pub running: bool,
    pub integration_health: aion_integration::IntegrationHealth,
    pub api_health: aion_api::ApiHealth,
    pub core_health: aion_core::CoreHealth,
    pub overall_status: HealthStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Critical,
    Offline,
}

/// Initialize and start the complete AION-CR ↔ ECTUS-R system
pub async fn start_aion_ectus_unified_system() -> Result<Arc<AionCrSystem>> {
    info!("🌟 Starting complete AION-CR ↔ ECTUS-R unified system with maximum autonomy");

    // Initialize the system
    let system = Arc::new(AionCrSystem::new_with_ectus_integration().await?);

    // Start unified operation
    system.start_unified_system().await?;

    info!("🎉 AION-CR ↔ ECTUS-R unified system fully operational");
    Ok(system)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test;

    #[tokio::test]
    async fn test_system_initialization() {
        let system = AionCrSystem::new_with_ectus_integration().await.unwrap();
        assert!(!*system.running.read().await);
    }

    #[tokio::test]
    async fn test_system_startup() {
        let system = AionCrSystem::new_with_ectus_integration().await.unwrap();
        let result = system.start_unified_system().await;
        assert!(result.is_ok());
        assert!(*system.running.read().await);
    }

    #[tokio::test]
    async fn test_health_check() {
        let system = AionCrSystem::new_with_ectus_integration().await.unwrap();
        let health = system.health_check().await.unwrap();
        assert_eq!(health.system_id, system.system_id);
    }
}