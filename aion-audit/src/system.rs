use aion_core::{AionResult, AuditSystem, AuditTrail};
use std::collections::HashMap;

pub struct ComprehensiveAuditSystem {
    trail_storage: Vec<AuditTrail>,
}

impl ComprehensiveAuditSystem {
    pub fn new() -> Self {
        Self {
            trail_storage: Vec::new(),
        }
    }
}

impl Default for ComprehensiveAuditSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl AuditSystem for ComprehensiveAuditSystem {
    fn record_action(&mut self, entity_type: &str, entity_id: &str, action: &str, actor: &str, details: HashMap<String, String>) -> AionResult<()> {
        let audit_entry = AuditTrail {
            id: uuid::Uuid::new_v4(),
            entity_type: entity_type.to_string(),
            entity_id: entity_id.to_string(),
            action: action.to_string(),
            actor: actor.to_string(),
            timestamp: chrono::Utc::now(),
            details,
            previous_state: None,
            new_state: None,
        };

        self.trail_storage.push(audit_entry);
        Ok(())
    }

    fn get_audit_trail(&self, entity_id: &str) -> AionResult<Vec<AuditTrail>> {
        Ok(self.trail_storage.iter()
            .filter(|entry| entry.entity_id == entity_id)
            .cloned()
            .collect())
    }

    fn verify_integrity(&self) -> AionResult<bool> {
        Ok(true)
    }
}