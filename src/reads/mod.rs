pub mod sequence;
pub mod quality;
pub mod platform;
pub mod filtering;

pub use quality::QualityScore;
pub use platform::Platform;

use ahash::HashMap;
use serde::{Deserialize, Serialize};

/// Represents a sequencing read with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceRead {
    /// Read identifier
    pub id: String,
    
    /// DNA sequence as bytes (A, C, G, T, N)
    pub sequence: Vec<u8>,
    
    /// Quality scores (Phred scale), if available
    pub quality: Option<Vec<u8>>,
    
    /// Sequencing platform
    pub platform: Platform,
    
    /// Sequence length (cached)
    pub length: usize,
    
    /// Average quality score
    pub avg_quality: Option<f64>,
    
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl SequenceRead {
    /// Create a new sequence read
    pub fn new(id: String, sequence: Vec<u8>, quality: Option<Vec<u8>>, platform: Platform) -> Self {
        let length = sequence.len();
        let avg_quality = quality.as_ref().map(|q| {
            q.iter().map(|&qv| qv as f64).sum::<f64>() / q.len() as f64
        });
        
        Self {
            id,
            sequence,
            quality,
            platform,
            length,
            avg_quality,
            metadata: HashMap::default(),
        }
    }

    /// Get sequence as string
    pub fn sequence_str(&self) -> String {
        String::from_utf8_lossy(&self.sequence).to_string()
    }

    /// Check if read passes quality threshold
    pub fn passes_quality(&self, min_quality: f64) -> bool {
        self.avg_quality.map_or(true, |q| q >= min_quality)
    }

    /// Check if read length is within bounds
    pub fn passes_length(&self, max_length: usize) -> bool {
        self.length <= max_length
    }

    /// Calculate GC content
    pub fn gc_content(&self) -> f64 {
        let gc_count = self.sequence.iter()
            .filter(|&&b| b == b'G' || b == b'C' || b == b'g' || b == b'c')
            .count();
        gc_count as f64 / self.length as f64
    }

    /// Reverse complement
    pub fn reverse_complement(&self) -> Self {
        let rc_seq: Vec<u8> = self.sequence.iter().rev().map(|&base| {
            match base {
                b'A' | b'a' => b'T',
                b'T' | b't' => b'A',
                b'C' | b'c' => b'G',
                b'G' | b'g' => b'C',
                _ => base,
            }
        }).collect();
        
        let rc_qual = self.quality.as_ref().map(|q| {
            q.iter().rev().copied().collect()
        });

        Self {
            id: format!("{}_rc", self.id),
            sequence: rc_seq,
            quality: rc_qual,
            platform: self.platform,
            length: self.length,
            avg_quality: self.avg_quality,
            metadata: self.metadata.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_read_creation() {
        let read = SequenceRead::new(
            "test_read".to_string(),
            b"ACGTACGT".to_vec(),
            Some(vec![30, 35, 40, 35, 30, 35, 40, 35]),
            Platform::PacBio,
        );
        
        assert_eq!(read.id, "test_read");
        assert_eq!(read.length, 8);
        assert!(read.avg_quality.is_some());
    }

    #[test]
    fn test_reverse_complement() {
        let read = SequenceRead::new(
            "test".to_string(),
            b"ACGT".to_vec(),
            None,
            Platform::PacBio,
        );
        
        let rc = read.reverse_complement();
        assert_eq!(rc.sequence_str(), "ACGT");
    }

    #[test]
    fn test_gc_content() {
        let read = SequenceRead::new(
            "test".to_string(),
            b"ACGT".to_vec(),
            None,
            Platform::PacBio,
        );
        
        assert_eq!(read.gc_content(), 0.5);
    }
}
