use aion_core::types::*;
use aion_core::{AionResult};
use std::collections::{HashMap, HashSet};

/// Conflict graph for analyzing complex regulatory conflicts
pub struct ConflictGraph {
    nodes: HashMap<NormativeId, ConflictNode>,
    edges: HashMap<String, ConflictEdge>,
    conflict_clusters: Vec<ConflictCluster>,
}

#[derive(Debug, Clone)]
pub struct ConflictNode {
    pub framework_id: NormativeId,
    pub conflict_count: usize,
    pub centrality_score: f64,
    pub connected_frameworks: HashSet<NormativeId>,
}

#[derive(Debug, Clone)]
pub struct ConflictEdge {
    pub source: NormativeId,
    pub target: NormativeId,
    pub conflict: NormativeConflict,
    pub weight: f64,
}

#[derive(Debug, Clone)]
pub struct ConflictCluster {
    pub id: String,
    pub frameworks: Vec<NormativeId>,
    pub conflicts: Vec<NormativeConflict>,
    pub resolution_priority: f64,
}

impl ConflictGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            conflict_clusters: Vec::new(),
        }
    }

    pub fn add_conflict(&mut self, conflict: NormativeConflict) -> AionResult<()> {
        // Add nodes for frameworks involved
        self.ensure_node(&conflict.normative_a);
        self.ensure_node(&conflict.normative_b);

        // Add edge for the conflict
        let edge_id = format!("{}_{}", conflict.normative_a.0, conflict.normative_b.0);
        let weight = self.calculate_conflict_weight(&conflict);

        let edge = ConflictEdge {
            source: conflict.normative_a.clone(),
            target: conflict.normative_b.clone(),
            conflict: conflict.clone(),
            weight,
        };

        self.edges.insert(edge_id, edge);

        // Update node connections
        if let Some(node_a) = self.nodes.get_mut(&conflict.normative_a) {
            node_a.connected_frameworks.insert(conflict.normative_b.clone());
            node_a.conflict_count += 1;
        }

        if let Some(node_b) = self.nodes.get_mut(&conflict.normative_b) {
            node_b.connected_frameworks.insert(conflict.normative_a.clone());
            node_b.conflict_count += 1;
        }

        Ok(())
    }

    fn ensure_node(&mut self, framework_id: &NormativeId) {
        if !self.nodes.contains_key(framework_id) {
            let node = ConflictNode {
                framework_id: framework_id.clone(),
                conflict_count: 0,
                centrality_score: 0.0,
                connected_frameworks: HashSet::new(),
            };
            self.nodes.insert(framework_id.clone(), node);
        }
    }

    fn calculate_conflict_weight(&self, conflict: &NormativeConflict) -> f64 {
        match conflict.severity {
            ConflictSeverity::Critical => 10.0,
            ConflictSeverity::High => 7.5,
            ConflictSeverity::Medium => 5.0,
            ConflictSeverity::Low => 2.5,
            ConflictSeverity::Informational => 1.0,
        }
    }

    pub fn analyze_centrality(&mut self) -> AionResult<()> {
        let total_nodes = self.nodes.len() as f64;

        for node in self.nodes.values_mut() {
            // Simple degree centrality calculation
            node.centrality_score = node.connected_frameworks.len() as f64 / total_nodes.max(1.0);
        }

        Ok(())
    }

    pub fn identify_clusters(&mut self) -> AionResult<()> {
        self.conflict_clusters.clear();

        // Simple clustering based on connected components
        let mut visited: HashSet<NormativeId> = HashSet::new();
        let mut cluster_id = 0;

        for framework_id in self.nodes.keys() {
            if !visited.contains(framework_id) {
                let cluster = self.explore_cluster(framework_id, &mut visited)?;
                if cluster.frameworks.len() > 1 {
                    self.conflict_clusters.push(cluster);
                    cluster_id += 1;
                }
            }
        }

        Ok(())
    }

    fn explore_cluster(&self, start: &NormativeId, visited: &mut HashSet<NormativeId>) -> AionResult<ConflictCluster> {
        let mut frameworks = Vec::new();
        let mut conflicts = Vec::new();
        let mut to_visit = vec![start.clone()];

        while let Some(current) = to_visit.pop() {
            if visited.contains(&current) {
                continue;
            }

            visited.insert(current.clone());
            frameworks.push(current.clone());

            if let Some(node) = self.nodes.get(&current) {
                for connected in &node.connected_frameworks {
                    if !visited.contains(connected) {
                        to_visit.push(connected.clone());

                        // Find the conflict between current and connected
                        let edge_id = format!("{}_{}", current.0, connected.0);
                        if let Some(edge) = self.edges.get(&edge_id) {
                            conflicts.push(edge.conflict.clone());
                        }
                    }
                }
            }
        }

        let resolution_priority = self.calculate_cluster_priority(&conflicts);

        Ok(ConflictCluster {
            id: format!("cluster_{}", frameworks.len()),
            frameworks,
            conflicts,
            resolution_priority,
        })
    }

    fn calculate_cluster_priority(&self, conflicts: &[NormativeConflict]) -> f64 {
        if conflicts.is_empty() {
            return 0.0;
        }

        let total_weight: f64 = conflicts.iter()
            .map(|c| self.calculate_conflict_weight(c))
            .sum();

        total_weight / conflicts.len() as f64
    }

    pub fn get_most_critical_conflicts(&self, limit: usize) -> Vec<&ConflictEdge> {
        let mut edges: Vec<&ConflictEdge> = self.edges.values().collect();
        edges.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap_or(std::cmp::Ordering::Equal));
        edges.into_iter().take(limit).collect()
    }

    pub fn get_high_centrality_frameworks(&self, limit: usize) -> Vec<&ConflictNode> {
        let mut nodes: Vec<&ConflictNode> = self.nodes.values().collect();
        nodes.sort_by(|a, b| b.centrality_score.partial_cmp(&a.centrality_score).unwrap_or(std::cmp::Ordering::Equal));
        nodes.into_iter().take(limit).collect()
    }

    pub fn get_conflict_clusters(&self) -> &[ConflictCluster] {
        &self.conflict_clusters
    }
}

impl Default for ConflictGraph {
    fn default() -> Self {
        Self::new()
    }
}