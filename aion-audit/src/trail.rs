use aion_core::{AionResult, AuditTrail};

pub struct AuditTrailManager;

impl AuditTrailManager {
    pub fn new() -> Self {
        Self
    }

    pub fn format_trail(&self, trail: &[AuditTrail]) -> String {
        let mut output = String::new();

        for entry in trail {
            output.push_str(&format!(
                "[{}] {} performed '{}' on {} ({})\n",
                entry.timestamp.format("%Y-%m-%d %H:%M:%S"),
                entry.actor,
                entry.action,
                entry.entity_type,
                entry.entity_id
            ));
        }

        output
    }
}

impl Default for AuditTrailManager {
    fn default() -> Self {
        Self::new()
    }
}