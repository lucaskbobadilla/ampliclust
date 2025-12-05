pub mod kmeans;
pub mod hierarchical;
pub mod dbscan;
pub mod denovo;
pub mod reference_guided;

use serde::{Deserialize, Serialize};
use crate::metrics::ClusterMetrics;

/// Represents a cluster of reads
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cluster {
    /// Cluster identifier
    pub id: usize,
    
    /// Guide/reference name (if reference-guided)
    pub guide_name: Option<String>,
    
    /// Read IDs belonging to this cluster
    pub reads: Vec<String>,
    
    /// Consensus sequence (if generated)
    pub consensus: Option<ConsensusSequence>,
    
    /// Cluster metrics
    pub metrics: ClusterMetrics,
}

/// Consensus sequence with quality information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusSequence {
    pub sequence: Vec<u8>,
    pub quality: Vec<u8>,
    pub length: usize,
}

impl Cluster {
    pub fn new(id: usize, guide_name: Option<String>) -> Self {
        Self {
            id,
            guide_name,
            reads: Vec::new(),
            consensus: None,
            metrics: ClusterMetrics::default(),
        }
    }

    pub fn add_read(&mut self, read_id: String) {
        self.reads.push(read_id);
        self.metrics.read_count += 1;
    }

    pub fn set_consensus(&mut self, consensus: ConsensusSequence) {
        self.consensus = Some(consensus);
    }

    /// Generate FASTA header for cluster
    pub fn fasta_header(&self) -> String {
        format!(
            ">cluster_{}_guide-{}_reads-{} freq:{:.3} diversity:{:.3} quality:{:.1} chimera:{:.3} length:{}",
            self.id,
            self.guide_name.as_deref().unwrap_or("denovo"),
            self.metrics.read_count,
            self.metrics.frequency,
            self.metrics.diversity,
            self.metrics.avg_quality,
            self.metrics.chimera_score.unwrap_or(-1.0),
            self.consensus.as_ref().map(|c| c.length).unwrap_or(0)
        )
    }

    /// Check if cluster passes filtering criteria
    pub fn passes_filters(
        &self,
        min_frequency: f64,
        min_reads: usize,
        max_chimera: f64,
    ) -> bool {
        self.metrics.frequency >= min_frequency
            && self.metrics.read_count >= min_reads
            && self.metrics.chimera_score.map_or(true, |score| score <= max_chimera)
    }
}

/// Result of clustering analysis
#[derive(Debug)]
pub struct ClusteringResult {
    pub clusters: Vec<Cluster>,
    pub unassigned_reads: Vec<String>,
    pub total_reads: usize,
}

impl ClusteringResult {
    pub fn new() -> Self {
        Self {
            clusters: Vec::new(),
            unassigned_reads: Vec::new(),
            total_reads: 0,
        }
    }

    pub fn add_cluster(&mut self, cluster: Cluster) {
        self.total_reads += cluster.reads.len();
        self.clusters.push(cluster);
    }

    pub fn calculate_frequencies(&mut self) {
        let total = self.total_reads as f64;
        for cluster in &mut self.clusters {
            cluster.metrics.frequency = cluster.metrics.read_count as f64 / total;
        }
    }
    
    /// Calculate guide-specific frequencies (frequency within each guide/locus)
    pub fn calculate_guide_frequencies(&mut self) {
        use std::collections::HashMap;
        
        // Count total reads per guide
        let mut guide_totals: HashMap<String, usize> = HashMap::new();
        for cluster in &self.clusters {
            if let Some(ref guide) = cluster.guide_name {
                *guide_totals.entry(guide.clone()).or_insert(0) += cluster.metrics.read_count;
            }
        }
        
        // Calculate frequency relative to guide
        for cluster in &mut self.clusters {
            if let Some(ref guide) = cluster.guide_name {
                if let Some(&total) = guide_totals.get(guide) {
                    cluster.metrics.guide_frequency = 
                        cluster.metrics.read_count as f64 / total as f64;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cluster_creation() {
        let mut cluster = Cluster::new(0, Some("guide1".to_string()));
        cluster.add_read("read1".to_string());
        cluster.add_read("read2".to_string());
        
        assert_eq!(cluster.reads.len(), 2);
        assert_eq!(cluster.metrics.read_count, 2);
    }

    #[test]
    fn test_cluster_filtering() {
        let mut cluster = Cluster::new(0, None);
        cluster.metrics.frequency = 0.1;
        cluster.metrics.read_count = 10;
        cluster.metrics.chimera_score = Some(0.5);
        
        assert!(cluster.passes_filters(0.05, 5, 1.0));
        assert!(!cluster.passes_filters(0.15, 5, 1.0));
    }
}
