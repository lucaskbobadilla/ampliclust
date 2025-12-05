/// Read placement against reference sequences
use crate::alignment::kmer::KmerIndex;
use crate::reads::SequenceRead;
use anyhow::Result;
use rayon::prelude::*;

/// A read placement result
#[derive(Debug, Clone)]
pub struct Placement {
    /// Reference ID that the read was placed to
    pub ref_id: usize,
    
    /// Confidence score (0.0-1.0)
    pub confidence: f64,
    
    /// Number of k-mer hits
    pub hits: usize,
    
    /// Edit distance if calculated
    pub edit_distance: Option<usize>,
    
    /// Whether this is a secondary alignment
    pub is_secondary: bool,
}

/// Configuration for read placement
#[derive(Debug, Clone)]
pub struct PlacementConfig {
    /// K-mer size for indexing
    pub k: usize,
    
    /// Minimum k-mer hits to consider a placement
    pub min_hits: usize,
    
    /// Minimum confidence score (0.0-1.0)
    pub min_confidence: f64,
    
    /// Report secondary alignments
    pub report_secondary: bool,
    
    /// Maximum secondary alignments to report
    pub max_secondary: usize,
    
    /// Number of threads for parallel processing
    pub threads: usize,
}

impl Default for PlacementConfig {
    fn default() -> Self {
        Self {
            k: 15,
            min_hits: 5,
            min_confidence: 0.7,
            report_secondary: false,
            max_secondary: 5,
            threads: 4,
        }
    }
}

/// Place reads against reference sequences using k-mer index
pub fn place_reads(
    reads: &[SequenceRead],
    index: &KmerIndex,
    config: &PlacementConfig,
) -> Result<Vec<Option<Placement>>> {
    // Use rayon for parallel processing
    let placements: Vec<Option<Placement>> = reads
        .par_iter()
        .map(|read| place_read(&read.sequence, index, config))
        .collect();
    
    Ok(placements)
}

/// Place a single read against the index
pub fn place_read(
    sequence: &[u8],
    index: &KmerIndex,
    config: &PlacementConfig,
) -> Option<Placement> {
    // Query the index
    let hits = index.query(sequence);
    
    if hits.is_empty() {
        return None;
    }
    
    // Get the best hit
    let (best_ref_id, best_hits) = hits[0];
    
    // Check minimum thresholds
    if best_hits < config.min_hits {
        return None;
    }
    
    // Calculate confidence based on k-mer coverage
    let max_possible_kmers = if sequence.len() >= config.k {
        sequence.len() - config.k + 1
    } else {
        0
    };
    
    let confidence = if max_possible_kmers > 0 {
        best_hits as f64 / max_possible_kmers as f64
    } else {
        0.0
    };
    
    if confidence < config.min_confidence {
        return None;
    }
    
    Some(Placement {
        ref_id: best_ref_id,
        confidence,
        hits: best_hits,
        edit_distance: None,
        is_secondary: false,
    })
}

/// Place reads and optionally report secondary placements
pub fn place_reads_with_secondary(
    reads: &[SequenceRead],
    index: &KmerIndex,
    config: &PlacementConfig,
) -> Result<Vec<Vec<Placement>>> {
    let placements: Vec<Vec<Placement>> = reads
        .par_iter()
        .map(|read| place_read_with_secondary(&read.sequence, index, config))
        .collect();
    
    Ok(placements)
}

/// Place a single read with secondary alignments
fn place_read_with_secondary(
    sequence: &[u8],
    index: &KmerIndex,
    config: &PlacementConfig,
) -> Vec<Placement> {
    let hits = index.query(sequence);
    
    if hits.is_empty() {
        return vec![];
    }
    
    let max_possible_kmers = if sequence.len() >= config.k {
        sequence.len() - config.k + 1
    } else {
        return vec![];
    };
    
    let mut placements = Vec::new();
    
    for (idx, &(ref_id, hit_count)) in hits.iter().enumerate() {
        if hit_count < config.min_hits {
            break;
        }
        
        let confidence = hit_count as f64 / max_possible_kmers as f64;
        
        if confidence < config.min_confidence {
            break;
        }
        
        let is_secondary = idx > 0;
        
        if is_secondary && !config.report_secondary {
            break;
        }
        
        if is_secondary && placements.len() >= config.max_secondary + 1 {
            break;
        }
        
        placements.push(Placement {
            ref_id,
            confidence,
            hits: hit_count,
            edit_distance: None,
            is_secondary,
        });
    }
    
    placements
}

/// Calculate placement statistics for a set of reads
pub fn calculate_placement_stats(placements: &[Option<Placement>]) -> PlacementStats {
    let total_reads = placements.len();
    let placed_reads = placements.iter().filter(|p| p.is_some()).count();
    let unplaced_reads = total_reads - placed_reads;
    
    let avg_confidence = if placed_reads > 0 {
        placements
            .iter()
            .filter_map(|p| p.as_ref())
            .map(|p| p.confidence)
            .sum::<f64>()
            / placed_reads as f64
    } else {
        0.0
    };
    
    let avg_hits = if placed_reads > 0 {
        placements
            .iter()
            .filter_map(|p| p.as_ref())
            .map(|p| p.hits)
            .sum::<usize>() as f64
            / placed_reads as f64
    } else {
        0.0
    };
    
    PlacementStats {
        total_reads,
        placed_reads,
        unplaced_reads,
        placement_rate: if total_reads > 0 {
            placed_reads as f64 / total_reads as f64
        } else {
            0.0
        },
        avg_confidence,
        avg_hits,
    }
}

/// Statistics about read placement
#[derive(Debug, Clone)]
pub struct PlacementStats {
    pub total_reads: usize,
    pub placed_reads: usize,
    pub unplaced_reads: usize,
    pub placement_rate: f64,
    pub avg_confidence: f64,
    pub avg_hits: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_placement_config_default() {
        let config = PlacementConfig::default();
        assert_eq!(config.k, 15);
        assert_eq!(config.min_hits, 5);
        assert!((config.min_confidence - 0.7).abs() < 0.01);
    }
    
    #[test]
    fn test_place_read() {
        let mut index = KmerIndex::new(4);
        index.index_reference(0, b"ACGTACGTACGT");
        
        let config = PlacementConfig {
            k: 4,
            min_hits: 1,
            min_confidence: 0.5,
            ..Default::default()
        };
        
        let placement = place_read(b"ACGTACGT", &index, &config);
        assert!(placement.is_some());
        
        let p = placement.unwrap();
        assert_eq!(p.ref_id, 0);
        assert!(p.confidence > 0.5);
    }
    
    #[test]
    fn test_place_read_below_threshold() {
        let mut index = KmerIndex::new(4);
        index.index_reference(0, b"ACGTACGTACGT");
        
        let config = PlacementConfig {
            k: 4,
            min_hits: 100, // Too high
            min_confidence: 0.5,
            ..Default::default()
        };
        
        let placement = place_read(b"ACGTACGT", &index, &config);
        assert!(placement.is_none());
    }
    
    #[test]
    fn test_placement_stats() {
        let placements = vec![
            Some(Placement {
                ref_id: 0,
                confidence: 0.9,
                hits: 10,
                edit_distance: None,
                is_secondary: false,
            }),
            None,
            Some(Placement {
                ref_id: 1,
                confidence: 0.8,
                hits: 8,
                edit_distance: None,
                is_secondary: false,
            }),
        ];
        
        let stats = calculate_placement_stats(&placements);
        assert_eq!(stats.total_reads, 3);
        assert_eq!(stats.placed_reads, 2);
        assert_eq!(stats.unplaced_reads, 1);
        assert!((stats.placement_rate - 0.666).abs() < 0.01);
    }
    
    #[test]
    fn test_place_reads_parallel() {
        let mut index = KmerIndex::new(4);
        index.index_reference(0, b"ACGTACGTACGT");
        
        use crate::config::Platform;
        
        let reads = vec![
            SequenceRead::new("read1".to_string(), b"ACGTACGT".to_vec(), None, Platform::Unknown),
            SequenceRead::new("read2".to_string(), b"TGCATGCA".to_vec(), None, Platform::Unknown),
        ];
        
        let config = PlacementConfig {
            k: 4,
            min_hits: 1,
            min_confidence: 0.5,
            ..Default::default()
        };
        
        let placements = place_reads(&reads, &index, &config).unwrap();
        assert_eq!(placements.len(), 2);
    }
}
