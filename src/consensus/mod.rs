//! Consensus sequence generation
//!
//! This module provides algorithms for generating consensus sequences
//! from clustered reads, including simple majority-vote and quality-weighted
//! consensus methods.

pub mod simple;
pub mod poa;
pub mod spoa_wrapper;
pub mod polish;
pub mod quality;

use crate::reads::SequenceRead;
use crate::clustering::ConsensusSequence;
use anyhow::Result;

/// Generate consensus from a set of reads
pub fn generate_consensus(
    reads: &[&SequenceRead],
    method: ConsensusMethod,
) -> Result<ConsensusSequence> {
    match method {
        ConsensusMethod::Simple => simple::simple_consensus(reads),
        ConsensusMethod::QualityWeighted => simple::quality_weighted_consensus(reads),
    }
}

/// Consensus generation method
#[derive(Debug, Clone, Copy)]
pub enum ConsensusMethod {
    /// Simple majority vote
    Simple,
    /// Quality-weighted consensus
    QualityWeighted,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reads::{SequenceRead, Platform};

    #[test]
    fn test_simple_consensus() {
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
        
        let reads = vec![&read1, &read2];
        let consensus = generate_consensus(&reads, ConsensusMethod::Simple).unwrap();
        
        assert_eq!(consensus.sequence, b"ACGT");
        assert_eq!(consensus.length, 4);
    }
}