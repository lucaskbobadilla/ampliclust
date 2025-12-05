/// Edit distance calculations for alignment refinement
/// Pure Rust implementation of edit distance algorithms
use std::cmp::min;

/// Calculate edit distance between two sequences using Wagner-Fischer algorithm
/// Returns the minimum number of edits (insertions, deletions, substitutions)
pub fn edit_distance(seq1: &[u8], seq2: &[u8]) -> usize {
    let len1 = seq1.len();
    let len2 = seq2.len();
    
    if len1 == 0 {
        return len2;
    }
    if len2 == 0 {
        return len1;
    }
    
    // Create DP table
    let mut prev_row = vec![0; len2 + 1];
    let mut curr_row = vec![0; len2 + 1];
    
    // Initialize first row
    for j in 0..=len2 {
        prev_row[j] = j;
    }
    
    // Fill DP table
    for i in 1..=len1 {
        curr_row[0] = i;
        
        for j in 1..=len2 {
            let cost = if seq1[i - 1].to_ascii_uppercase() == seq2[j - 1].to_ascii_uppercase() {
                0
            } else {
                1
            };
            
            curr_row[j] = min(
                min(
                    prev_row[j] + 1,      // deletion
                    curr_row[j - 1] + 1   // insertion
                ),
                prev_row[j - 1] + cost    // substitution
            );
        }
        
        std::mem::swap(&mut prev_row, &mut curr_row);
    }
    
    prev_row[len2]
}

/// Calculate edit distance with early termination if distance exceeds threshold
/// More efficient when you only care if distance is below a certain value
pub fn edit_distance_bounded(seq1: &[u8], seq2: &[u8], max_distance: usize) -> Option<usize> {
    let len1 = seq1.len();
    let len2 = seq2.len();
    
    // Early termination based on length difference
    if len1.abs_diff(len2) > max_distance {
        return None;
    }
    
    if len1 == 0 {
        return if len2 <= max_distance { Some(len2) } else { None };
    }
    if len2 == 0 {
        return if len1 <= max_distance { Some(len1) } else { None };
    }
    
    let mut prev_row = vec![0; len2 + 1];
    let mut curr_row = vec![0; len2 + 1];
    
    // Initialize first row
    for j in 0..=len2 {
        prev_row[j] = j;
    }
    
    // Fill DP table with early termination
    for i in 1..=len1 {
        curr_row[0] = i;
        let mut min_in_row = i;
        
        for j in 1..=len2 {
            let cost = if seq1[i - 1].to_ascii_uppercase() == seq2[j - 1].to_ascii_uppercase() {
                0
            } else {
                1
            };
            
            curr_row[j] = min(
                min(
                    prev_row[j] + 1,
                    curr_row[j - 1] + 1
                ),
                prev_row[j - 1] + cost
            );
            
            min_in_row = min(min_in_row, curr_row[j]);
        }
        
        // Early termination: if minimum in row exceeds threshold, we can stop
        if min_in_row > max_distance {
            return None;
        }
        
        std::mem::swap(&mut prev_row, &mut curr_row);
    }
    
    let distance = prev_row[len2];
    if distance <= max_distance {
        Some(distance)
    } else {
        None
    }
}

/// Calculate sequence identity (percentage of matching bases)
pub fn sequence_identity(seq1: &[u8], seq2: &[u8]) -> f64 {
    if seq1.is_empty() || seq2.is_empty() {
        return 0.0;
    }
    
    let distance = edit_distance(seq1, seq2);
    let max_len = seq1.len().max(seq2.len());
    
    if max_len == 0 {
        return 100.0;
    }
    
    let identity = (max_len - distance) as f64 / max_len as f64;
    identity * 100.0
}

/// Calculate Hamming distance (only for sequences of equal length)
pub fn hamming_distance(seq1: &[u8], seq2: &[u8]) -> Option<usize> {
    if seq1.len() != seq2.len() {
        return None;
    }
    
    let distance = seq1
        .iter()
        .zip(seq2.iter())
        .filter(|(a, b)| a.to_ascii_uppercase() != b.to_ascii_uppercase())
        .count();
    
    Some(distance)
}

/// Alignment result with detailed information
#[derive(Debug, Clone)]
pub struct AlignmentResult {
    pub edit_distance: usize,
    pub identity: f64,
    pub query_length: usize,
    pub ref_length: usize,
}

impl AlignmentResult {
    pub fn new(seq1: &[u8], seq2: &[u8]) -> Self {
        let distance = edit_distance(seq1, seq2);
        let identity = sequence_identity(seq1, seq2);
        
        Self {
            edit_distance: distance,
            identity,
            query_length: seq1.len(),
            ref_length: seq2.len(),
        }
    }
    
    pub fn from_bounded(seq1: &[u8], seq2: &[u8], max_distance: usize) -> Option<Self> {
        let distance = edit_distance_bounded(seq1, seq2, max_distance)?;
        let identity = sequence_identity(seq1, seq2);
        
        Some(Self {
            edit_distance: distance,
            identity,
            query_length: seq1.len(),
            ref_length: seq2.len(),
        })
    }
}

/// Find best matching reference sequence based on edit distance
pub fn find_best_match<'a>(
    query: &[u8],
    references: &'a [Vec<u8>],
    max_distance: Option<usize>,
) -> Option<(usize, AlignmentResult)> {
    let mut best_idx = None;
    let mut best_distance = usize::MAX;
    let mut best_result = None;
    
    for (idx, reference) in references.iter().enumerate() {
        let distance = if let Some(max_dist) = max_distance {
            match edit_distance_bounded(query, reference, max_dist) {
                Some(d) => d,
                None => continue,
            }
        } else {
            edit_distance(query, reference)
        };
        
        if distance < best_distance {
            best_distance = distance;
            best_idx = Some(idx);
            best_result = Some(AlignmentResult::new(query, reference));
        }
    }
    
    best_idx.map(|idx| (idx, best_result.unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_edit_distance_identical() {
        let seq1 = b"ACGT";
        let seq2 = b"ACGT";
        assert_eq!(edit_distance(seq1, seq2), 0);
    }
    
    #[test]
    fn test_edit_distance_one_substitution() {
        let seq1 = b"ACGT";
        let seq2 = b"ACAT";
        assert_eq!(edit_distance(seq1, seq2), 1);
    }
    
    #[test]
    fn test_edit_distance_one_insertion() {
        let seq1 = b"ACGT";
        let seq2 = b"ACGAT";
        assert_eq!(edit_distance(seq1, seq2), 1);
    }
    
    #[test]
    fn test_edit_distance_one_deletion() {
        let seq1 = b"ACGAT";
        let seq2 = b"ACGT";
        assert_eq!(edit_distance(seq1, seq2), 1);
    }
    
    #[test]
    fn test_edit_distance_empty() {
        let seq1 = b"ACGT";
        let seq2 = b"";
        assert_eq!(edit_distance(seq1, seq2), 4);
        assert_eq!(edit_distance(seq2, seq1), 4);
    }
    
    #[test]
    fn test_edit_distance_bounded() {
        let seq1 = b"ACGT";
        let seq2 = b"TGCA";
        
        // Should succeed - distance is 4
        assert_eq!(edit_distance_bounded(seq1, seq2, 5), Some(4));
        
        // Should fail - distance exceeds threshold
        assert_eq!(edit_distance_bounded(seq1, seq2, 2), None);
    }
    
    #[test]
    fn test_sequence_identity() {
        let seq1 = b"ACGT";
        let seq2 = b"ACGT";
        assert!((sequence_identity(seq1, seq2) - 100.0).abs() < 0.01);
        
        let seq3 = b"ACAT";
        let identity = sequence_identity(seq1, seq3);
        assert!((identity - 75.0).abs() < 0.01);
    }
    
    #[test]
    fn test_hamming_distance() {
        let seq1 = b"ACGT";
        let seq2 = b"ACAT";
        assert_eq!(hamming_distance(seq1, seq2), Some(1));
        
        let seq3 = b"ACGTA";
        assert_eq!(hamming_distance(seq1, seq3), None); // Different lengths
    }
    
    #[test]
    fn test_alignment_result() {
        let seq1 = b"ACGT";
        let seq2 = b"ACAT";
        let result = AlignmentResult::new(seq1, seq2);
        
        assert_eq!(result.edit_distance, 1);
        assert_eq!(result.query_length, 4);
        assert_eq!(result.ref_length, 4);
        assert!(result.identity > 70.0);
    }
    
    #[test]
    fn test_find_best_match() {
        let query = b"ACGT";
        let references = vec![
            b"TGCA".to_vec(),
            b"ACGT".to_vec(),
            b"ACGA".to_vec(),
        ];
        
        let (best_idx, result) = find_best_match(query, &references, None).unwrap();
        assert_eq!(best_idx, 1); // Exact match
        assert_eq!(result.edit_distance, 0);
    }
    
    #[test]
    fn test_case_insensitive() {
        let seq1 = b"ACGT";
        let seq2 = b"acgt";
        assert_eq!(edit_distance(seq1, seq2), 0);
    }
}
