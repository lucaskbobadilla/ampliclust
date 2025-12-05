/// Metrics for cluster quality assessment

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterMetrics {
    /// Number of reads in cluster
    pub read_count: usize,
    
    /// Frequency/abundance of cluster relative to total
    pub frequency: f64,
    
    /// Frequency relative to guide/locus (NEW)
    pub guide_frequency: f64,
    
    /// Diversity metric (Shannon entropy or similar)
    pub diversity: f64,
    
    /// Average quality score of reads
    pub avg_quality: f64,
    
    /// UCHIME chimera score (if calculated)
    pub chimera_score: Option<f64>,
    
    /// Parent sequences if chimeric
    pub chimera_parents: Option<(String, String)>,
    
    /// Average coverage depth
    pub coverage_depth: f64,
    
    /// Strand bias metric
    pub strand_bias: f64,
}

impl Default for ClusterMetrics {
    fn default() -> Self {
        Self {
            read_count: 0,
            frequency: 0.0,
            guide_frequency: 0.0,
            diversity: 0.0,
            avg_quality: 0.0,
            chimera_score: None,
            chimera_parents: None,
            coverage_depth: 0.0,
            strand_bias: 0.0,
        }
    }
}

pub mod cluster_stats;
pub mod diversity;
pub mod chimera;
pub mod quality_control;
