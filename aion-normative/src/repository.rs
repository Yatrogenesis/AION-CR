use aion_core::{AionError, AionResult, NormativeFramework, NormativeId, NormativeRepository};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct InMemoryNormativeRepository {
    frameworks: Arc<RwLock<HashMap<NormativeId, NormativeFramework>>>,
    index: Arc<RwLock<HashMap<String, Vec<NormativeId>>>>,
}

impl InMemoryNormativeRepository {
    pub fn new() -> Self {
        Self {
            frameworks: Arc::new(RwLock::new(HashMap::new())),
            index: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn update_index(&self, framework: &NormativeFramework) -> AionResult<()> {
        let mut index = self.index.write().map_err(|e| AionError::InternalError {
            message: format!("Failed to acquire write lock on index: {}", e),
        })?;

        let keywords = aion_core::extract_keywords(&format!("{} {}", framework.title, framework.description));

        for keyword in keywords {
            index.entry(keyword).or_insert_with(Vec::new).push(framework.id.clone());
        }

        let jurisdiction_key = format!("jurisdiction:{:?}", framework.jurisdiction);
        index.entry(jurisdiction_key).or_insert_with(Vec::new).push(framework.id.clone());

        let type_key = format!("type:{:?}", framework.normative_type);
        index.entry(type_key).or_insert_with(Vec::new).push(framework.id.clone());

        let authority_key = format!("authority:{}", aion_core::normalize_string(&framework.authority));
        index.entry(authority_key).or_insert_with(Vec::new).push(framework.id.clone());

        for tag in &framework.tags {
            let tag_key = format!("tag:{}", aion_core::normalize_string(tag));
            index.entry(tag_key).or_insert_with(Vec::new).push(framework.id.clone());
        }

        Ok(())
    }

    fn remove_from_index(&self, framework_id: &NormativeId) -> AionResult<()> {
        let mut index = self.index.write().map_err(|e| AionError::InternalError {
            message: format!("Failed to acquire write lock on index: {}", e),
        })?;

        for (_, ids) in index.iter_mut() {
            ids.retain(|id| id != framework_id);
        }

        index.retain(|_, ids| !ids.is_empty());

        Ok(())
    }

    pub fn get_frameworks_by_jurisdiction(&self, jurisdiction: &aion_core::Jurisdiction) -> AionResult<Vec<NormativeFramework>> {
        let index = self.index.read().map_err(|e| AionError::InternalError {
            message: format!("Failed to acquire read lock on index: {}", e),
        })?;

        let frameworks = self.frameworks.read().map_err(|e| AionError::InternalError {
            message: format!("Failed to acquire read lock on frameworks: {}", e),
        })?;

        let jurisdiction_key = format!("jurisdiction:{:?}", jurisdiction);

        if let Some(framework_ids) = index.get(&jurisdiction_key) {
            let mut result = Vec::new();
            for id in framework_ids {
                if let Some(framework) = frameworks.get(id) {
                    result.push(framework.clone());
                }
            }
            Ok(result)
        } else {
            Ok(Vec::new())
        }
    }

    pub fn get_frameworks_by_type(&self, normative_type: &aion_core::NormativeType) -> AionResult<Vec<NormativeFramework>> {
        let index = self.index.read().map_err(|e| AionError::InternalError {
            message: format!("Failed to acquire read lock on index: {}", e),
        })?;

        let frameworks = self.frameworks.read().map_err(|e| AionError::InternalError {
            message: format!("Failed to acquire read lock on frameworks: {}", e),
        })?;

        let type_key = format!("type:{:?}", normative_type);

        if let Some(framework_ids) = index.get(&type_key) {
            let mut result = Vec::new();
            for id in framework_ids {
                if let Some(framework) = frameworks.get(id) {
                    result.push(framework.clone());
                }
            }
            Ok(result)
        } else {
            Ok(Vec::new())
        }
    }

    pub fn get_frameworks_by_tag(&self, tag: &str) -> AionResult<Vec<NormativeFramework>> {
        let index = self.index.read().map_err(|e| AionError::InternalError {
            message: format!("Failed to acquire read lock on index: {}", e),
        })?;

        let frameworks = self.frameworks.read().map_err(|e| AionError::InternalError {
            message: format!("Failed to acquire read lock on frameworks: {}", e),
        })?;

        let tag_key = format!("tag:{}", aion_core::normalize_string(tag));

        if let Some(framework_ids) = index.get(&tag_key) {
            let mut result = Vec::new();
            for id in framework_ids {
                if let Some(framework) = frameworks.get(id) {
                    result.push(framework.clone());
                }
            }
            Ok(result)
        } else {
            Ok(Vec::new())
        }
    }

    pub fn get_conflicting_frameworks(&self, framework_id: &NormativeId) -> AionResult<Vec<NormativeFramework>> {
        let frameworks = self.frameworks.read().map_err(|e| AionError::InternalError {
            message: format!("Failed to acquire read lock on frameworks: {}", e),
        })?;

        let target_framework = frameworks.get(framework_id)
            .ok_or_else(|| AionError::NormativeNotFound { id: framework_id.0.to_string() })?;

        let mut conflicts = Vec::new();

        for (id, framework) in frameworks.iter() {
            if id != framework_id &&
               framework.jurisdiction == target_framework.jurisdiction &&
               self.has_content_overlap(target_framework, framework) {
                conflicts.push(framework.clone());
            }
        }

        Ok(conflicts)
    }

    fn has_content_overlap(&self, framework1: &NormativeFramework, framework2: &NormativeFramework) -> bool {
        let similarity = aion_core::calculate_similarity(&framework1.description, &framework2.description);
        similarity > 0.3 || framework1.tags.iter().any(|tag| framework2.tags.contains(tag))
    }

    pub fn get_statistics(&self) -> AionResult<HashMap<String, u64>> {
        let frameworks = self.frameworks.read().map_err(|e| AionError::InternalError {
            message: format!("Failed to acquire read lock on frameworks: {}", e),
        })?;

        let mut stats = HashMap::new();
        stats.insert("total_frameworks".to_string(), frameworks.len() as u64);

        let active_count = frameworks.values().filter(|f| f.is_active()).count();
        stats.insert("active_frameworks".to_string(), active_count as u64);

        let mut type_counts = HashMap::new();
        let mut jurisdiction_counts = HashMap::new();

        for framework in frameworks.values() {
            let type_key = format!("{:?}", framework.normative_type);
            *type_counts.entry(type_key).or_insert(0u64) += 1;

            let jurisdiction_key = format!("{:?}", framework.jurisdiction);
            *jurisdiction_counts.entry(jurisdiction_key).or_insert(0u64) += 1;
        }

        for (type_name, count) in type_counts {
            stats.insert(format!("type_{}", type_name.to_lowercase()), count);
        }

        for (jurisdiction_name, count) in jurisdiction_counts {
            stats.insert(format!("jurisdiction_{}", jurisdiction_name.to_lowercase()), count);
        }

        Ok(stats)
    }

    pub fn export_all(&self) -> AionResult<Vec<NormativeFramework>> {
        let frameworks = self.frameworks.read().map_err(|e| AionError::InternalError {
            message: format!("Failed to acquire read lock on frameworks: {}", e),
        })?;

        Ok(frameworks.values().cloned().collect())
    }

    pub fn import_frameworks(&mut self, frameworks: Vec<NormativeFramework>) -> AionResult<u64> {
        let mut imported_count = 0;

        for framework in frameworks {
            match self.store_framework(framework) {
                Ok(_) => imported_count += 1,
                Err(e) => {
                    tracing::warn!("Failed to import framework: {}", e);
                }
            }
        }

        Ok(imported_count)
    }

    pub fn compact(&mut self) -> AionResult<()> {
        self.remove_expired_frameworks()?;
        self.rebuild_index()?;
        Ok(())
    }

    fn remove_expired_frameworks(&mut self) -> AionResult<()> {
        let mut frameworks = self.frameworks.write().map_err(|e| AionError::InternalError {
            message: format!("Failed to acquire write lock on frameworks: {}", e),
        })?;

        let now = chrono::Utc::now();
        frameworks.retain(|_, framework| {
            framework.expiration_date.map_or(true, |exp| now < exp)
        });

        Ok(())
    }

    fn rebuild_index(&self) -> AionResult<()> {
        let frameworks = self.frameworks.read().map_err(|e| AionError::InternalError {
            message: format!("Failed to acquire read lock on frameworks: {}", e),
        })?;

        let mut index = self.index.write().map_err(|e| AionError::InternalError {
            message: format!("Failed to acquire write lock on index: {}", e),
        })?;

        index.clear();

        drop(index);

        for framework in frameworks.values() {
            self.update_index(framework)?;
        }

        Ok(())
    }
}

impl Default for InMemoryNormativeRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl NormativeRepository for InMemoryNormativeRepository {
    fn store_framework(&self, framework: NormativeFramework) -> AionResult<()> {
        let id = framework.id.clone();

        let mut frameworks = self.frameworks.write().map_err(|e| AionError::InternalError {
            message: format!("Failed to acquire write lock on frameworks: {}", e),
        })?;

        frameworks.insert(id, framework.clone());
        drop(frameworks);

        self.update_index(&framework)?;

        Ok(())
    }

    fn get_framework(&self, id: &NormativeId) -> AionResult<Option<NormativeFramework>> {
        let frameworks = self.frameworks.read().map_err(|e| AionError::InternalError {
            message: format!("Failed to acquire read lock on frameworks: {}", e),
        })?;

        Ok(frameworks.get(id).cloned())
    }

    fn list_frameworks(&self) -> AionResult<Vec<NormativeFramework>> {
        let frameworks = self.frameworks.read().map_err(|e| AionError::InternalError {
            message: format!("Failed to acquire read lock on frameworks: {}", e),
        })?;

        Ok(frameworks.values().cloned().collect())
    }

    fn update_framework(&self, framework: NormativeFramework) -> AionResult<()> {
        let id = framework.id.clone();

        let mut frameworks = self.frameworks.write().map_err(|e| AionError::InternalError {
            message: format!("Failed to acquire write lock on frameworks: {}", e),
        })?;

        if frameworks.contains_key(&id) {
            self.remove_from_index(&id)?;
            frameworks.insert(id, framework.clone());
            drop(frameworks);
            self.update_index(&framework)?;
            Ok(())
        } else {
            Err(AionError::NormativeNotFound { id: id.0.to_string() })
        }
    }

    fn delete_framework(&self, id: &NormativeId) -> AionResult<()> {
        let mut frameworks = self.frameworks.write().map_err(|e| AionError::InternalError {
            message: format!("Failed to acquire write lock on frameworks: {}", e),
        })?;

        if frameworks.remove(id).is_some() {
            drop(frameworks);
            self.remove_from_index(id)?;
            Ok(())
        } else {
            Err(AionError::NormativeNotFound { id: id.0.to_string() })
        }
    }

    fn search_frameworks(&self, query: &str) -> AionResult<Vec<NormativeFramework>> {
        let index = self.index.read().map_err(|e| AionError::InternalError {
            message: format!("Failed to acquire read lock on index: {}", e),
        })?;

        let frameworks = self.frameworks.read().map_err(|e| AionError::InternalError {
            message: format!("Failed to acquire read lock on frameworks: {}", e),
        })?;

        let query_terms = aion_core::extract_keywords(query);
        let mut matching_ids = std::collections::HashSet::new();

        for term in &query_terms {
            if let Some(ids) = index.get(term) {
                for id in ids {
                    matching_ids.insert(id.clone());
                }
            }
        }

        let mut results = Vec::new();
        for id in matching_ids {
            if let Some(framework) = frameworks.get(&id) {
                results.push(framework.clone());
            }
        }

        results.sort_by(|a, b| {
            let score_a = self.calculate_relevance_score(a, &query_terms);
            let score_b = self.calculate_relevance_score(b, &query_terms);
            score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(results)
    }

    fn get_active_frameworks(&self) -> AionResult<Vec<NormativeFramework>> {
        let frameworks = self.frameworks.read().map_err(|e| AionError::InternalError {
            message: format!("Failed to acquire read lock on frameworks: {}", e),
        })?;

        Ok(frameworks.values().filter(|f| f.is_active()).cloned().collect())
    }
}

impl InMemoryNormativeRepository {
    fn calculate_relevance_score(&self, framework: &NormativeFramework, query_terms: &[String]) -> f64 {
        let framework_text = format!("{} {} {}", framework.title, framework.description, framework.tags.join(" "));
        let framework_terms = aion_core::extract_keywords(&framework_text);

        let mut matches = 0;
        for term in query_terms {
            if framework_terms.contains(term) {
                matches += 1;
            }
        }

        if query_terms.is_empty() {
            0.0
        } else {
            matches as f64 / query_terms.len() as f64
        }
    }
}