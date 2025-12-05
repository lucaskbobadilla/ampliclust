/// K-mer based indexing and alignment
use ahash::AHashMap as HashMap;

/// K-mer index for fast sequence lookup
pub struct KmerIndex {
    /// K-mer size
    k: usize,
    
    /// Index: encoded k-mer -> list of (reference_id, position)
    index: HashMap<u64, Vec<(usize, usize)>>,
    
    /// Total k-mers indexed
    total_kmers: usize,
}

impl KmerIndex {
    /// Create a new k-mer index with specified k-mer size
    pub fn new(k: usize) -> Self {
        assert!(k > 0 && k <= 31, "K-mer size must be between 1 and 31");
        
        Self {
            k,
            index: HashMap::default(),
            total_kmers: 0,
        }
    }
    
    /// Index a reference sequence
    pub fn index_reference(&mut self, ref_id: usize, sequence: &[u8]) {
        if sequence.len() < self.k {
            return; // Sequence too short for k-mers
        }
        
        for (pos, window) in sequence.windows(self.k).enumerate() {
            if let Some(encoded) = Self::encode_kmer(window) {
                self.index
                    .entry(encoded)
                    .or_insert_with(Vec::new)
                    .push((ref_id, pos));
                self.total_kmers += 1;
            }
        }
    }
    
    /// Encode a k-mer as a 64-bit integer (2-bit encoding: A=00, C=01, G=10, T=11)
    fn encode_kmer(kmer: &[u8]) -> Option<u64> {
        let mut encoded = 0u64;
        
        for &base in kmer {
            let bits = match base {
                b'A' | b'a' => 0b00,
                b'C' | b'c' => 0b01,
                b'G' | b'g' => 0b10,
                b'T' | b't' => 0b11,
                _ => return None, // Skip k-mers with ambiguous bases
            };
            encoded = (encoded << 2) | bits;
        }
        
        Some(encoded)
    }
    
    /// Decode a k-mer from its integer representation
    #[allow(dead_code)]
    fn decode_kmer(encoded: u64, k: usize) -> Vec<u8> {
        let mut kmer = Vec::with_capacity(k);
        let mut value = encoded;
        
        for _ in 0..k {
            let base = match value & 0b11 {
                0b00 => b'A',
                0b01 => b'C',
                0b10 => b'G',
                0b11 => b'T',
                _ => unreachable!(),
            };
            kmer.push(base);
            value >>= 2;
        }
        
        kmer.reverse();
        kmer
    }
    
    /// Query the index with a sequence, returning reference hits sorted by count
    pub fn query(&self, sequence: &[u8]) -> Vec<(usize, usize)> {
        let mut hit_counts: HashMap<usize, usize> = HashMap::default();
        
        if sequence.len() < self.k {
            return vec![];
        }
        
        // Count k-mer hits for each reference
        for window in sequence.windows(self.k) {
            if let Some(encoded) = Self::encode_kmer(window) {
                if let Some(positions) = self.index.get(&encoded) {
                    for &(ref_id, _pos) in positions {
                        *hit_counts.entry(ref_id).or_insert(0) += 1;
                    }
                }
            }
        }
        
        // Sort by hit count (descending)
        let mut sorted_hits: Vec<_> = hit_counts.into_iter().collect();
        sorted_hits.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
        
        sorted_hits
    }
    
    /// Get detailed k-mer matches for a sequence against a specific reference
    pub fn get_matches(&self, sequence: &[u8], ref_id: usize) -> Vec<KmerMatch> {
        let mut matches = Vec::new();
        
        if sequence.len() < self.k {
            return matches;
        }
        
        for (query_pos, window) in sequence.windows(self.k).enumerate() {
            if let Some(encoded) = Self::encode_kmer(window) {
                if let Some(positions) = self.index.get(&encoded) {
                    for &(rid, ref_pos) in positions {
                        if rid == ref_id {
                            matches.push(KmerMatch {
                                query_pos,
                                ref_pos,
                                kmer_size: self.k,
                            });
                        }
                    }
                }
            }
        }
        
        matches
    }
    
    /// Get statistics about the index
    pub fn stats(&self) -> IndexStats {
        IndexStats {
            k: self.k,
            unique_kmers: self.index.len(),
            total_kmers: self.total_kmers,
            avg_occurrences: if self.index.is_empty() {
                0.0
            } else {
                self.total_kmers as f64 / self.index.len() as f64
            },
        }
    }
}

/// A k-mer match between query and reference
#[derive(Debug, Clone, Copy)]
pub struct KmerMatch {
    pub query_pos: usize,
    pub ref_pos: usize,
    pub kmer_size: usize,
}

/// Statistics about a k-mer index
#[derive(Debug, Clone)]
pub struct IndexStats {
    pub k: usize,
    pub unique_kmers: usize,
    pub total_kmers: usize,
    pub avg_occurrences: f64,
}

/// Calculate sequence similarity based on shared k-mers
pub fn kmer_similarity(seq1: &[u8], seq2: &[u8], k: usize) -> f64 {
    if seq1.len() < k || seq2.len() < k {
        return 0.0;
    }
    
    // Build k-mer set for first sequence
    let mut kmers1: HashMap<u64, usize> = HashMap::default();
    for window in seq1.windows(k) {
        if let Some(encoded) = KmerIndex::encode_kmer(window) {
            *kmers1.entry(encoded).or_insert(0) += 1;
        }
    }
    
    // Count shared k-mers with second sequence
    let mut shared = 0;
    let mut total2 = 0;
    
    for window in seq2.windows(k) {
        if let Some(encoded) = KmerIndex::encode_kmer(window) {
            total2 += 1;
            if kmers1.contains_key(&encoded) {
                shared += 1;
            }
        }
    }
    
    if total2 == 0 {
        return 0.0;
    }
    
    shared as f64 / total2 as f64
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_kmer_encoding() {
        let kmer = b"ACGT";
        let encoded = KmerIndex::encode_kmer(kmer).unwrap();
        assert!(encoded > 0);
        
        let decoded = KmerIndex::decode_kmer(encoded, 4);
        assert_eq!(decoded, b"ACGT");
    }
    
    #[test]
    fn test_kmer_encoding_with_n() {
        let kmer = b"ACNT";
        let encoded = KmerIndex::encode_kmer(kmer);
        assert!(encoded.is_none()); // Should return None for ambiguous bases
    }
    
    #[test]
    fn test_kmer_index_creation() {
        let index = KmerIndex::new(15);
        assert_eq!(index.k, 15);
        assert_eq!(index.total_kmers, 0);
    }
    
    #[test]
    fn test_kmer_indexing() {
        let mut index = KmerIndex::new(4);
        let seq = b"ACGTACGT";
        
        index.index_reference(0, seq);
        
        let stats = index.stats();
        assert!(stats.total_kmers > 0);
        assert!(stats.unique_kmers > 0);
    }
    
    #[test]
    fn test_kmer_query() {
        let mut index = KmerIndex::new(4);
        
        // Index two references
        index.index_reference(0, b"ACGTACGTACGT");
        index.index_reference(1, b"TGCATGCATGCA");
        
        // Query with sequence similar to reference 0
        let hits = index.query(b"ACGTACGT");
        
        assert!(!hits.is_empty());
        assert_eq!(hits[0].0, 0); // Should match reference 0 best
    }
    
    #[test]
    fn test_kmer_matches() {
        let mut index = KmerIndex::new(4);
        index.index_reference(0, b"ACGTACGT");
        
        let matches = index.get_matches(b"ACGTACGT", 0);
        assert!(!matches.is_empty());
    }
    
    #[test]
    fn test_kmer_similarity() {
        let sim = kmer_similarity(b"ACGTACGT", b"ACGTACGT", 4);
        assert!((sim - 1.0).abs() < 0.01); // Should be ~1.0 for identical sequences
        
        let sim2 = kmer_similarity(b"ACGTACGT", b"TGCATGCA", 4);
        assert!(sim2 < 0.5); // Should be low for different sequences
    }
    
    #[test]
    fn test_reverse_complement_encoding() {
        let seq1 = b"ACGT";
        let seq2 = b"ACGT"; // Reverse complement of ACGT is ACGT (palindrome)
        
        let enc1 = KmerIndex::encode_kmer(seq1);
        let enc2 = KmerIndex::encode_kmer(seq2);
        
        assert!(enc1.is_some());
        assert!(enc2.is_some());
    }
}
