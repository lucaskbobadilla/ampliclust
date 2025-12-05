//! Reference-guided clustering
//!
//! Groups reads based on their placement to reference sequences.
//! This is the simplest and fastest clustering mode.

use crate::reads::SequenceRead;
use crate::clustering::{Cluster, ClusteringResult};
use std::collections::HashMap;
use anyhow::Result;
use log::info;

/// Represents a placed read with its reference information
#[derive(Debug, Clone)]
pub struct PlacedRead {
    pub read_id: String,
    pub reference_name: String,
    pub locus: Option<String>,
    pub confidence: f64,
    pub hits: usize,
}

/// Cluster reads by their reference placement
///
/// Groups reads that were placed to the same reference sequence.
/// This is used after the read placement phase.
///
/// # Arguments
/// * `placed_reads` - Read placements with reference names
/// * `min_confidence` - Minimum placement confidence to include read
///
/// # Returns
/// ClusteringResult with clusters grouped by reference
pub fn cluster_by_placement(
    placed_reads: &[PlacedRead],
    min_confidence: f64,
) -> Result<ClusteringResult> {
    info!("Clustering {} placements by reference", placed_reads.len());
    info!("Minimum confidence threshold: {:.3}", min_confidence);
    
    let mut result = ClusteringResult::new();
    let mut reference_groups: HashMap<String, Vec<String>> = HashMap::new();
    
    // Group reads by their placed reference
    for placed in placed_reads {
        if placed.confidence >= min_confidence {
            reference_groups
                .entry(placed.reference_name.clone())
                .or_insert_with(Vec::new)
                .push(placed.read_id.clone());
        } else {
            result.unassigned_reads.push(placed.read_id.clone());
        }
    }
    
    // Create clusters from groups
    let mut cluster_id = 0;
    for (reference_name, read_ids) in reference_groups {
        let mut cluster = Cluster::new(cluster_id, Some(reference_name.clone()));
        
        for read_id in read_ids {
            cluster.add_read(read_id);
        }
        
        info!("Cluster {}: {} -> {} reads", 
              cluster_id, reference_name, cluster.reads.len());
        
        result.add_cluster(cluster);
        cluster_id += 1;
    }
    
    // Calculate frequencies
    result.calculate_frequencies();
    
    info!("Created {} clusters from {} reads", 
          result.clusters.len(), result.total_reads);
    info!("Unassigned reads: {}", result.unassigned_reads.len());
    
    Ok(result)
}

/// Cluster reads by locus (for grouped references)
///
/// Groups reads by their locus assignment, then sub-clusters within each locus
/// based on similarity.
///
/// # Arguments
/// * `placed_reads` - Read placements from Phase 2
/// * `locus_map` - Mapping from reference name to locus name
/// * `min_confidence` - Minimum placement confidence
///
/// # Returns
/// ClusteringResult with clusters grouped by locus and reference
pub fn cluster_by_locus(
    placed_reads: &[PlacedRead],
    locus_map: &HashMap<String, String>,
    min_confidence: f64,
) -> Result<ClusteringResult> {
    info!("Clustering by locus with {} placements", placed_reads.len());
    
    let mut result = ClusteringResult::new();
    let mut locus_groups: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();
    
    // Group by locus, then by reference within locus
    for placed in placed_reads {
        if placed.confidence >= min_confidence {
            let locus = locus_map
                .get(&placed.reference_name)
                .cloned()
                .unwrap_or_else(|| placed.reference_name.clone());
            
            locus_groups
                .entry(locus)
                .or_insert_with(HashMap::new)
                .entry(placed.reference_name.clone())
                .or_insert_with(Vec::new)
                .push(placed.read_id.clone());
        } else {
            result.unassigned_reads.push(placed.read_id.clone());
        }
    }
    
    // Create clusters
    let mut cluster_id = 0;
    let num_loci = locus_groups.len();
    
    for (locus_name, reference_groups) in locus_groups {
        info!("Locus {}: {} references", locus_name, reference_groups.len());
        
        for (reference_name, read_ids) in reference_groups {
            let mut cluster = Cluster::new(cluster_id, Some(reference_name.clone()));
            
            for read_id in read_ids {
                cluster.add_read(read_id);
            }
            
            info!("  Cluster {}: {} -> {} reads", 
                  cluster_id, reference_name, cluster.reads.len());
            
            result.add_cluster(cluster);
            cluster_id += 1;
        }
    }
    
    // Calculate frequencies
    result.calculate_frequencies();
    
    info!("Created {} clusters across {} loci", 
          result.clusters.len(), num_loci);
    
    Ok(result)
}

/// Cluster reads within a locus using sequence similarity
///
/// For reads in the same locus, perform finer clustering based on
/// sequence similarity or variant patterns.
///
/// # Arguments
/// * `reads` - Reads to cluster
/// * `reads_map` - Lookup map for read sequences
/// * `similarity_threshold` - Minimum similarity to group reads
///
/// # Returns
/// Vector of clusters (as Vec<String> of read IDs)
pub fn subcluster_by_similarity(
    read_ids: &[String],
    _reads_map: &HashMap<String, &SequenceRead>,
    _similarity_threshold: f64,
) -> Vec<Vec<String>> {
    // Simplified version: put all reads in one cluster
    // Full implementation would:
    // 1. Calculate pairwise edit distances
    // 2. Build similarity graph
    // 3. Find connected components or apply threshold-based clustering
    
    if read_ids.is_empty() {
        return vec![];
    }
    
    vec![read_ids.to_vec()]
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cluster_by_placement() {
        let placed_reads = vec![
            PlacedRead {
                read_id: "read1".to_string(),
                reference_name: "ref_A".to_string(),
                locus: None,
                confidence: 0.95,
                hits: 100,
            },
            PlacedRead {
                read_id: "read2".to_string(),
                reference_name: "ref_A".to_string(),
                locus: None,
                confidence: 0.90,
                hits: 95,
            },
            PlacedRead {
                read_id: "read3".to_string(),
                reference_name: "ref_B".to_string(),
                locus: None,
                confidence: 0.85,
                hits: 80,
            },
            PlacedRead {
                read_id: "read4".to_string(),
                reference_name: "ref_A".to_string(),
                locus: None,
                confidence: 0.60, // Below threshold
                hits: 50,
            },
        ];
        
        let result = cluster_by_placement(&placed_reads, 0.8).unwrap();
        
        assert_eq!(result.clusters.len(), 2); // ref_A and ref_B
        assert_eq!(result.total_reads, 3); // read4 excluded
        assert_eq!(result.unassigned_reads.len(), 1); // read4
        
        // Find ref_A cluster
        let ref_a_cluster = result.clusters.iter()
            .find(|c| c.guide_name.as_deref() == Some("ref_A"))
            .unwrap();
        assert_eq!(ref_a_cluster.reads.len(), 2);
    }
    
    #[test]
    fn test_cluster_by_locus() {
        let placed_reads = vec![
            PlacedRead {
                read_id: "read1".to_string(),
                reference_name: "allele_A1".to_string(),
                locus: Some("Locus_A".to_string()),
                confidence: 0.95,
                hits: 100,
            },
            PlacedRead {
                read_id: "read2".to_string(),
                reference_name: "allele_A2".to_string(),
                locus: Some("Locus_A".to_string()),
                confidence: 0.90,
                hits: 95,
            },
        ];
        
        let mut locus_map = HashMap::new();
        locus_map.insert("allele_A1".to_string(), "Locus_A".to_string());
        locus_map.insert("allele_A2".to_string(), "Locus_A".to_string());
        
        let result = cluster_by_locus(&placed_reads, &locus_map, 0.8).unwrap();
        
        assert_eq!(result.clusters.len(), 2); // Two alleles in same locus
        assert_eq!(result.total_reads, 2);
    }
    
    #[test]
    fn test_subcluster_by_similarity() {
        let read_ids = vec!["read1".to_string(), "read2".to_string(), "read3".to_string()];
        let reads_map = HashMap::new();
        let clusters = subcluster_by_similarity(&read_ids, &reads_map, 0.9);
        
        assert_eq!(clusters.len(), 1); // Simplified: all in one cluster
        assert_eq!(clusters[0].len(), 3);
    }
}

