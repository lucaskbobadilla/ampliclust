/// Minimap2-style alignment using our k-mer index
/// This is a pure Rust implementation inspired by minimap2's approach
use crate::alignment::kmer::KmerIndex;
use crate::alignment::placement::Placement;

/// Minimap2-like alignment parameters
#[derive(Debug, Clone)]
pub struct MinimapConfig {
    /// K-mer size
    pub k: usize,
    
    /// Window size for minimizers
    pub w: usize,
    
    /// Minimum number of minimizer hits
    pub min_hits: usize,
    
    /// Minimum chain score
    pub min_chain_score: f64,
    
    /// Bandwidth for chaining
    pub bandwidth: usize,
}

impl Default for MinimapConfig {
    fn default() -> Self {
        Self {
            k: 15,
            w: 10,
            min_hits: 3,
            min_chain_score: 40.0,
            bandwidth: 500,
        }
    }
}

/// Preset configurations for different data types
impl MinimapConfig {
    /// Preset for PacBio HiFi data (high accuracy)
    pub fn pacbio_hifi() -> Self {
        Self {
            k: 19,
            w: 10,
            min_hits: 5,
            min_chain_score: 50.0,
            bandwidth: 500,
        }
    }
    
    /// Preset for ONT data (higher error rate)
    pub fn ont() -> Self {
        Self {
            k: 15,
            w: 10,
            min_hits: 3,
            min_chain_score: 40.0,
            bandwidth: 500,
        }
    }
    
    /// Preset for Illumina data (short reads)
    pub fn illumina() -> Self {
        Self {
            k: 13,
            w: 5,
            min_hits: 2,
            min_chain_score: 30.0,
            bandwidth: 100,
        }
    }
}

/// A minimizer (reduced k-mer representation)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Minimizer {
    hash: u64,
    pos: usize,
}

/// An anchor point between query and reference
#[derive(Debug, Clone, Copy)]
pub struct Anchor {
    pub query_pos: usize,
    pub ref_pos: usize,
    pub length: usize,
}

/// A chain of anchors
#[derive(Debug, Clone)]
pub struct Chain {
    pub anchors: Vec<Anchor>,
    pub score: f64,
    pub ref_id: usize,
}

/// Align a sequence using minimap2-like approach
pub fn align_minimap_style(
    query: &[u8],
    index: &KmerIndex,
    config: &MinimapConfig,
) -> Option<Placement> {
    // Extract minimizers from query
    let query_minimizers = extract_minimizers(query, config.k, config.w);
    
    if query_minimizers.is_empty() {
        return None;
    }
    
    // Find candidate reference sequences
    let hits = index.query(query);
    if hits.is_empty() {
        return None;
    }
    
    // For the best candidate, build chains
    let (ref_id, hit_count) = hits[0];
    
    // Get detailed matches for chaining
    let matches = index.get_matches(query, ref_id);
    
    if matches.is_empty() {
        return None;
    }
    
    // Convert matches to anchors
    let anchors: Vec<Anchor> = matches
        .iter()
        .map(|m| Anchor {
            query_pos: m.query_pos,
            ref_pos: m.ref_pos,
            length: config.k,
        })
        .collect();
    
    // Build chains from anchors
    let chains = chain_anchors(&anchors, config.bandwidth);
    
    if chains.is_empty() {
        return None;
    }
    
    // Get best chain
    let best_chain = chains.iter().max_by(|a, b| {
        a.score.partial_cmp(&b.score).unwrap_or(std::cmp::Ordering::Equal)
    })?;
    
    if best_chain.score < config.min_chain_score {
        return None;
    }
    
    // Calculate confidence from chain score
    let max_possible_score = query.len() as f64;
    let confidence = (best_chain.score / max_possible_score).min(1.0);
    
    Some(Placement {
        ref_id,
        confidence,
        hits: hit_count,
        edit_distance: None,
        is_secondary: false,
    })
}

/// Extract minimizers from a sequence
fn extract_minimizers(sequence: &[u8], k: usize, w: usize) -> Vec<Minimizer> {
    if sequence.len() < k {
        return vec![];
    }
    
    let mut minimizers = Vec::new();
    let num_kmers = sequence.len() - k + 1;
    
    if num_kmers < w {
        // If sequence is shorter than window, just take all k-mers
        for (pos, window) in sequence.windows(k).enumerate() {
            if let Some(hash) = hash_kmer(window) {
                minimizers.push(Minimizer { hash, pos });
            }
        }
        return minimizers;
    }
    
    // Sliding window approach to find minimizers
    for window_start in 0..=(num_kmers - w) {
        let mut min_hash = u64::MAX;
        let mut min_pos = window_start;
        
        // Find minimum hash in window
        for i in 0..w {
            let pos = window_start + i;
            if pos + k <= sequence.len() {
                if let Some(hash) = hash_kmer(&sequence[pos..pos + k]) {
                    if hash < min_hash {
                        min_hash = hash;
                        min_pos = pos;
                    }
                }
            }
        }
        
        if min_hash != u64::MAX {
            // Only add if this minimizer is different from the last one
            if minimizers.is_empty() || minimizers.last().unwrap().pos != min_pos {
                minimizers.push(Minimizer {
                    hash: min_hash,
                    pos: min_pos,
                });
            }
        }
    }
    
    minimizers
}

/// Simple hash function for k-mers
fn hash_kmer(kmer: &[u8]) -> Option<u64> {
    let mut hash = 0u64;
    for &base in kmer {
        let val = match base {
            b'A' | b'a' => 0,
            b'C' | b'c' => 1,
            b'G' | b'g' => 2,
            b'T' | b't' => 3,
            _ => return None,
        };
        hash = hash.wrapping_mul(4).wrapping_add(val);
    }
    Some(hash)
}

/// Chain anchors using dynamic programming
fn chain_anchors(anchors: &[Anchor], bandwidth: usize) -> Vec<Chain> {
    if anchors.is_empty() {
        return vec![];
    }
    
    // Sort anchors by query position
    let mut sorted_anchors = anchors.to_vec();
    sorted_anchors.sort_by_key(|a| a.query_pos);
    
    let n = sorted_anchors.len();
    let mut scores = vec![0.0; n];
    let mut predecessors = vec![None; n];
    
    // Initialize first anchor
    scores[0] = sorted_anchors[0].length as f64;
    
    // Dynamic programming to find best chains
    for i in 1..n {
        let curr = &sorted_anchors[i];
        let mut best_score = curr.length as f64;
        let mut best_pred = None;
        
        // Look back at previous anchors within bandwidth
        for j in (0..i).rev() {
            let prev = &sorted_anchors[j];
            
            // Check if within bandwidth (roughly collinear)
            let query_dist = curr.query_pos.saturating_sub(prev.query_pos);
            let ref_dist = curr.ref_pos.saturating_sub(prev.ref_pos);
            
            if query_dist > bandwidth || ref_dist > bandwidth {
                continue;
            }
            
            // Calculate gap penalty (simplified)
            let gap = query_dist.abs_diff(ref_dist) as f64;
            let gap_penalty = gap * 0.1;
            
            let score = scores[j] + curr.length as f64 - gap_penalty;
            
            if score > best_score {
                best_score = score;
                best_pred = Some(j);
            }
        }
        
        scores[i] = best_score;
        predecessors[i] = best_pred;
    }
    
    // Backtrack to build chains
    let mut chains = Vec::new();
    let mut used = vec![false; n];
    
    // Start from highest scoring anchors
    loop {
        let mut max_score = 0.0;
        let mut max_idx = None;
        
        for i in 0..n {
            if !used[i] && scores[i] > max_score {
                max_score = scores[i];
                max_idx = Some(i);
            }
        }
        
        if max_idx.is_none() || max_score < 10.0 {
            break;
        }
        
        // Build chain by backtracking
        let mut chain_anchors = Vec::new();
        let mut idx = max_idx.unwrap();
        
        loop {
            used[idx] = true;
            chain_anchors.push(sorted_anchors[idx]);
            
            if let Some(pred) = predecessors[idx] {
                idx = pred;
            } else {
                break;
            }
        }
        
        chain_anchors.reverse();
        
        chains.push(Chain {
            anchors: chain_anchors,
            score: max_score,
            ref_id: 0, // Will be set by caller
        });
    }
    
    chains
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_minimap_config_defaults() {
        let config = MinimapConfig::default();
        assert_eq!(config.k, 15);
        assert_eq!(config.w, 10);
    }
    
    #[test]
    fn test_minimap_presets() {
        let hifi = MinimapConfig::pacbio_hifi();
        assert_eq!(hifi.k, 19);
        
        let ont = MinimapConfig::ont();
        assert_eq!(ont.k, 15);
        
        let illumina = MinimapConfig::illumina();
        assert_eq!(illumina.k, 13);
    }
    
    #[test]
    fn test_extract_minimizers() {
        let seq = b"ACGTACGTACGT";
        let minimizers = extract_minimizers(seq, 4, 3);
        assert!(!minimizers.is_empty());
    }
    
    #[test]
    fn test_hash_kmer() {
        let kmer = b"ACGT";
        let hash = hash_kmer(kmer);
        assert!(hash.is_some());
        
        let kmer_with_n = b"ACNT";
        let hash2 = hash_kmer(kmer_with_n);
        assert!(hash2.is_none());
    }
    
    #[test]
    fn test_chain_anchors() {
        let anchors = vec![
            Anchor { query_pos: 0, ref_pos: 0, length: 4 },
            Anchor { query_pos: 5, ref_pos: 5, length: 4 },
            Anchor { query_pos: 10, ref_pos: 10, length: 4 },
        ];
        
        let chains = chain_anchors(&anchors, 100);
        assert!(!chains.is_empty());
        assert!(chains[0].score > 0.0);
    }
    
    #[test]
    fn test_align_minimap_style() {
        let mut index = KmerIndex::new(4);
        index.index_reference(0, b"ACGTACGTACGT");
        
        let config = MinimapConfig {
            k: 4,
            w: 3,
            min_hits: 1,
            min_chain_score: 5.0,
            bandwidth: 100,
        };
        
        let placement = align_minimap_style(b"ACGTACGT", &index, &config);
        assert!(placement.is_some());
    }
}
