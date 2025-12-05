//! Simple consensus generation algorithms
//!
//! Provides majority-vote and quality-weighted consensus methods.

use crate::reads::SequenceRead;
use crate::clustering::ConsensusSequence;
use anyhow::{Result, bail};
use std::collections::HashMap;

/// Generate simple majority-vote consensus
///
/// For each position, selects the most common base across all reads.
/// Quality scores are averaged across all supporting reads.
pub fn simple_consensus(reads: &[&SequenceRead]) -> Result<ConsensusSequence> {
    if reads.is_empty() {
        bail!("Cannot generate consensus from empty read set");
    }
    
    // Find the maximum length
    let max_len = reads.iter().map(|r| r.sequence.len()).max().unwrap();
    
    let mut consensus_seq = Vec::with_capacity(max_len);
    let mut consensus_qual = Vec::with_capacity(max_len);
    
    // For each position
    for pos in 0..max_len {
        let mut base_counts: HashMap<u8, usize> = HashMap::new();
        let mut base_quals: HashMap<u8, Vec<u8>> = HashMap::new();
        
        // Count bases at this position
        for read in reads {
            if pos < read.sequence.len() {
                let base = read.sequence[pos];
                *base_counts.entry(base).or_insert(0) += 1;
                
                if let Some(ref qual) = read.quality {
                    if pos < qual.len() {
                        base_quals.entry(base)
                            .or_insert_with(Vec::new)
                            .push(qual[pos]);
                    }
                }
            }
        }
        
        if base_counts.is_empty() {
            // No coverage at this position
            consensus_seq.push(b'N');
            consensus_qual.push(0);
            continue;
        }
        
        // Find most common base
        let (&consensus_base, &_count) = base_counts.iter()
            .max_by_key(|(_, &count)| count)
            .unwrap();
        
        consensus_seq.push(consensus_base);
        
        // Calculate average quality for this base
        let avg_qual = if let Some(quals) = base_quals.get(&consensus_base) {
            if !quals.is_empty() {
                (quals.iter().map(|&q| q as u32).sum::<u32>() / quals.len() as u32) as u8
            } else {
                30 // Default quality
            }
        } else {
            30
        };
        
        consensus_qual.push(avg_qual);
    }
    
    Ok(ConsensusSequence {
        sequence: consensus_seq.clone(),
        quality: consensus_qual.clone(),
        length: consensus_seq.len(),
    })
}

/// Generate quality-weighted consensus
///
/// Weights each base by its quality score when determining consensus.
/// Bases with higher quality have more influence on the final consensus.
pub fn quality_weighted_consensus(reads: &[&SequenceRead]) -> Result<ConsensusSequence> {
    if reads.is_empty() {
        bail!("Cannot generate consensus from empty read set");
    }
    
    // Find the maximum length
    let max_len = reads.iter().map(|r| r.sequence.len()).max().unwrap();
    
    let mut consensus_seq = Vec::with_capacity(max_len);
    let mut consensus_qual = Vec::with_capacity(max_len);
    
    // For each position
    for pos in 0..max_len {
        let mut base_scores: HashMap<u8, f64> = HashMap::new();
        let mut base_quals: HashMap<u8, Vec<u8>> = HashMap::new();
        
        // Accumulate quality-weighted scores
        for read in reads {
            if pos < read.sequence.len() {
                let base = read.sequence[pos];
                let qual = read.quality.as_ref()
                    .and_then(|q| q.get(pos))
                    .copied()
                    .unwrap_or(20); // Default quality if missing
                
                // Convert Phred score to probability of correctness (1 - error_prob)
                let error_prob = phred_to_prob(qual);
                let weight = 1.0 - error_prob; // Higher quality = higher weight
                *base_scores.entry(base).or_insert(0.0) += weight;
                
                base_quals.entry(base)
                    .or_insert_with(Vec::new)
                    .push(qual);
            }
        }
        
        if base_scores.is_empty() {
            // No coverage at this position
            consensus_seq.push(b'N');
            consensus_qual.push(0);
            continue;
        }
        
        // Find base with highest quality-weighted score
        let (&consensus_base, _) = base_scores.iter()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();
        
        consensus_seq.push(consensus_base);
        
        // Calculate weighted average quality
        let avg_qual = if let Some(quals) = base_quals.get(&consensus_base) {
            if !quals.is_empty() {
                (quals.iter().map(|&q| q as u32).sum::<u32>() / quals.len() as u32) as u8
            } else {
                30
            }
        } else {
            30
        };
        
        consensus_qual.push(avg_qual);
    }
    
    Ok(ConsensusSequence {
        sequence: consensus_seq.clone(),
        quality: consensus_qual.clone(),
        length: consensus_seq.len(),
    })
}

/// Convert Phred quality score to error probability
fn phred_to_prob(phred: u8) -> f64 {
    10.0_f64.powf(-(phred as f64) / 10.0)
}

/// Convert error probability to Phred quality score
#[allow(dead_code)]
fn prob_to_phred(prob: f64) -> u8 {
    (-10.0 * prob.log10()).round().min(60.0).max(0.0) as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reads::Platform;
    
    #[test]
    fn test_simple_consensus_identical() {
        let read1 = SequenceRead::new(
            "read1".to_string(),
            b"ACGT".to_vec(),
            Some(vec![30, 30, 30, 30]),
            Platform::PacBio,
        );
        let read2 = SequenceRead::new(
            "read2".to_string(),
            b"ACGT".to_vec(),
            Some(vec![35, 35, 35, 35]),
            Platform::PacBio,
        );
        
        let reads = vec![&read1, &read2];
        let consensus = simple_consensus(&reads).unwrap();
        
        assert_eq!(consensus.sequence, b"ACGT");
        assert_eq!(consensus.length, 4);
        // Average quality should be around 32-33
        assert!(consensus.quality[0] >= 30 && consensus.quality[0] <= 35);
    }
    
    #[test]
    fn test_simple_consensus_with_variant() {
        let read1 = SequenceRead::new(
            "read1".to_string(),
            b"ACGT".to_vec(),
            Some(vec![30, 30, 30, 30]),
            Platform::PacBio,
        );
        let read2 = SequenceRead::new(
            "read2".to_string(),
            b"ACGT".to_vec(),
            Some(vec![30, 30, 30, 30]),
            Platform::PacBio,
        );
        let read3 = SequenceRead::new(
            "read3".to_string(),
            b"AGGT".to_vec(), // C->G at position 1
            Some(vec![30, 30, 30, 30]),
            Platform::PacBio,
        );
        
        let reads = vec![&read1, &read2, &read3];
        let consensus = simple_consensus(&reads).unwrap();
        
        // Position 1: C appears 2x, G appears 1x -> C wins
        assert_eq!(consensus.sequence, b"ACGT");
    }
    
    #[test]
    fn test_simple_consensus_different_lengths() {
        let read1 = SequenceRead::new(
            "read1".to_string(),
            b"ACGT".to_vec(),
            Some(vec![30, 30, 30, 30]),
            Platform::PacBio,
        );
        let read2 = SequenceRead::new(
            "read2".to_string(),
            b"ACGTAA".to_vec(), // Longer
            Some(vec![30, 30, 30, 30, 30, 30]),
            Platform::PacBio,
        );
        
        let reads = vec![&read1, &read2];
        let consensus = simple_consensus(&reads).unwrap();
        
        // Should be as long as the longest read
        assert_eq!(consensus.length, 6);
        assert_eq!(&consensus.sequence[0..4], b"ACGT");
    }
    
    #[test]
    fn test_quality_weighted_consensus() {
        let read1 = SequenceRead::new(
            "read1".to_string(),
            b"A".to_vec(),
            Some(vec![40]), // High quality A
            Platform::PacBio,
        );
        let read2 = SequenceRead::new(
            "read2".to_string(),
            b"T".to_vec(), // Low quality T
            Some(vec![10]),
            Platform::PacBio,
        );
        
        let reads = vec![&read1, &read2];
        let consensus = quality_weighted_consensus(&reads).unwrap();
        
        // High quality A should win over low quality T
        assert_eq!(consensus.sequence, b"A");
    }
    
    #[test]
    fn test_phred_conversion() {
        assert!((phred_to_prob(30) - 0.001).abs() < 0.0001);
        assert!((phred_to_prob(20) - 0.01).abs() < 0.001);
        
        assert_eq!(prob_to_phred(0.001), 30);
        assert_eq!(prob_to_phred(0.01), 20);
    }
    
    #[test]
    fn test_empty_reads() {
        let reads: Vec<&SequenceRead> = vec![];
        assert!(simple_consensus(&reads).is_err());
        assert!(quality_weighted_consensus(&reads).is_err());
    }
}
