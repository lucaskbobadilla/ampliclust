# AmpliClust Implementation Guide

This guide provides step-by-step instructions for implementing the core functionality of AmpliClust.

## Phase 1: Core I/O (Week 1-2)

### 1.1 FASTQ Reader

Implement in `src/io/fastq.rs`:

```rust
use seq_io::fastq::{Reader, Record};
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::{BufReader, Read};
use crate::reads::SequenceRead;
use crate::Platform;

pub struct FastqReader {
    reader: Box<dyn Read>,
}

impl FastqReader {
    pub fn new(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        let reader: Box<dyn Read> = if path.extension() == Some(OsStr::new("gz")) {
            Box::new(GzDecoder::new(file))
        } else {
            Box::new(file)
        };
        Ok(Self { reader })
    }

    pub fn read_sequences(&mut self, platform: Platform) -> Result<Vec<SequenceRead>> {
        let mut records = Vec::new();
        let reader = Reader::new(BufReader::new(&mut self.reader));
        
        for result in reader.records() {
            let record = result?;
            let seq_read = SequenceRead::new(
                String::from_utf8_lossy(record.id()).to_string(),
                record.seq().to_vec(),
                Some(record.qual().to_vec()),
                platform,
            );
            records.push(seq_read);
        }
        Ok(records)
    }
}
```

**Key Libraries**: `seq_io`, `flate2`

### 1.2 FASTA Reader

Implement in `src/io/fasta.rs`:

```rust
use bio::io::fasta;
use std::path::Path;

pub struct Reference {
    pub name: String,
    pub group: Option<String>,  // From header: name|group
    pub sequence: Vec<u8>,
}

pub fn read_references(path: &Path) -> Result<Vec<Reference>> {
    let reader = fasta::Reader::from_file(path)?;
    let mut references = Vec::new();
    
    for result in reader.records() {
        let record = result?;
        let id = record.id();
        
        // Parse name|group format
        let (name, group) = if id.contains('|') {
            let parts: Vec<&str> = id.split('|').collect();
            (parts[0].to_string(), Some(parts[1].to_string()))
        } else {
            (id.to_string(), None)
        };
        
        references.push(Reference {
            name,
            group,
            sequence: record.seq().to_vec(),
        });
    }
    Ok(references)
}
```

### 1.3 BAM Reader

Implement in `src/io/bam.rs`:

```rust
use rust_htslib::bam::{Read, Reader, Record};
use crate::reads::SequenceRead;

pub fn read_bam(path: &Path, platform: Platform) -> Result<Vec<SequenceRead>> {
    let mut bam = Reader::from_path(path)?;
    let mut reads = Vec::new();
    
    for result in bam.records() {
        let record = result?;
        if record.is_unmapped() {
            continue;
        }
        
        let seq_read = SequenceRead::new(
            String::from_utf8_lossy(record.qname()).to_string(),
            record.seq().as_bytes(),
            Some(record.qual().to_vec()),
            platform,
        );
        reads.push(seq_read);
    }
    Ok(reads)
}
```

## Phase 2: K-mer Indexing and Alignment (Week 3-4)

### 2.1 K-mer Index

Implement in `src/alignment/kmer.rs`:

```rust
use ahash::HashMap;

pub struct KmerIndex {
    k: usize,
    index: HashMap<u64, Vec<(usize, usize)>>,  // kmer -> [(ref_id, position)]
}

impl KmerIndex {
    pub fn new(k: usize) -> Self {
        Self {
            k,
            index: HashMap::default(),
        }
    }
    
    pub fn index_reference(&mut self, ref_id: usize, sequence: &[u8]) {
        for (pos, window) in sequence.windows(self.k).enumerate() {
            if let Some(kmer) = Self::encode_kmer(window) {
                self.index.entry(kmer)
                    .or_insert_with(Vec::new)
                    .push((ref_id, pos));
            }
        }
    }
    
    fn encode_kmer(kmer: &[u8]) -> Option<u64> {
        let mut encoded = 0u64;
        for &base in kmer {
            let bits = match base {
                b'A' | b'a' => 0,
                b'C' | b'c' => 1,
                b'G' | b'g' => 2,
                b'T' | b't' => 3,
                _ => return None,  // Skip kmers with N
            };
            encoded = (encoded << 2) | bits;
        }
        Some(encoded)
    }
    
    pub fn query(&self, sequence: &[u8]) -> Vec<(usize, usize)> {
        let mut hits = HashMap::default();
        
        for window in sequence.windows(self.k) {
            if let Some(kmer) = Self::encode_kmer(window) {
                if let Some(positions) = self.index.get(&kmer) {
                    for &(ref_id, _) in positions {
                        *hits.entry(ref_id).or_insert(0) += 1;
                    }
                }
            }
        }
        
        let mut sorted: Vec<_> = hits.into_iter().collect();
        sorted.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
        sorted
    }
}
```

### 2.2 Read Placement

Implement in `src/alignment/placement.rs`:

```rust
use super::kmer::KmerIndex;
use crate::reads::SequenceRead;

pub struct Placement {
    pub read_id: String,
    pub reference_id: usize,
    pub score: f64,
}

pub fn place_reads(
    reads: &[SequenceRead],
    references: &[Reference],
    kmer_size: usize,
) -> Result<Vec<Placement>> {
    // Build k-mer index
    let mut index = KmerIndex::new(kmer_size);
    for (ref_id, reference) in references.iter().enumerate() {
        index.index_reference(ref_id, &reference.sequence);
    }
    
    // Place reads in parallel
    use rayon::prelude::*;
    let placements: Vec<_> = reads.par_iter()
        .filter_map(|read| {
            let hits = index.query(&read.sequence);
            hits.first().map(|&(ref_id, count)| {
                let score = count as f64 / read.length as f64;
                Placement {
                    read_id: read.id.clone(),
                    reference_id: ref_id,
                    score,
                }
            })
        })
        .collect();
    
    Ok(placements)
}
```

## Phase 3: Clustering (Week 5-6)

### 3.1 Distance Matrix

Implement in `src/clustering/reference_guided.rs`:

```rust
use ndarray::{Array2, s};
use rayon::prelude::*;

pub fn build_distance_matrix(reads: &[SequenceRead]) -> Array2<f64> {
    let n = reads.len();
    let mut distances = Array2::zeros((n, n));
    
    // Compute upper triangle in parallel
    let pairs: Vec<_> = (0..n).flat_map(|i| (i+1..n).map(move |j| (i, j))).collect();
    
    let dists: Vec<_> = pairs.par_iter()
        .map(|&(i, j)| {
            let dist = edit_distance(&reads[i].sequence, &reads[j].sequence);
            (i, j, dist as f64 / reads[i].length.max(reads[j].length) as f64)
        })
        .collect();
    
    for (i, j, dist) in dists {
        distances[[i, j]] = dist;
        distances[[j, i]] = dist;
    }
    
    distances
}

fn edit_distance(seq1: &[u8], seq2: &[u8]) -> usize {
    // Implement edit distance (can use edlib-rs for production)
    // Simple implementation here for demonstration
    let m = seq1.len();
    let n = seq2.len();
    let mut dp = vec![vec![0; n + 1]; m + 1];
    
    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }
    
    for i in 1..=m {
        for j in 1..=n {
            let cost = if seq1[i-1] == seq2[j-1] { 0 } else { 1 };
            dp[i][j] = (dp[i-1][j] + 1)
                .min(dp[i][j-1] + 1)
                .min(dp[i-1][j-1] + cost);
        }
    }
    
    dp[m][n]
}
```

### 3.2 K-means Clustering

Implement in `src/clustering/kmeans.rs`:

```rust
use ndarray::Array2;
use rand::Rng;

pub struct KMeans {
    k: usize,
    max_iterations: usize,
    seed: u64,
}

impl KMeans {
    pub fn new(k: usize, max_iterations: usize, seed: u64) -> Self {
        Self { k, max_iterations, seed }
    }
    
    pub fn fit(&self, distances: &Array2<f64>) -> Vec<usize> {
        let n = distances.nrows();
        let mut rng = rand::rngs::StdRng::seed_from_u64(self.seed);
        
        // Initialize centroids randomly
        let mut assignments = vec![0; n];
        for i in 0..n {
            assignments[i] = rng.gen_range(0..self.k);
        }
        
        // Iterate
        for _ in 0..self.max_iterations {
            let mut changed = false;
            
            // Assign each point to nearest centroid
            for i in 0..n {
                let mut min_dist = f64::MAX;
                let mut best_cluster = 0;
                
                for cluster in 0..self.k {
                    // Calculate distance to cluster centroid (medoid)
                    let members: Vec<_> = assignments.iter()
                        .enumerate()
                        .filter(|(_, &c)| c == cluster)
                        .map(|(idx, _)| idx)
                        .collect();
                    
                    if members.is_empty() {
                        continue;
                    }
                    
                    let avg_dist: f64 = members.iter()
                        .map(|&j| distances[[i, j]])
                        .sum::<f64>() / members.len() as f64;
                    
                    if avg_dist < min_dist {
                        min_dist = avg_dist;
                        best_cluster = cluster;
                    }
                }
                
                if assignments[i] != best_cluster {
                    assignments[i] = best_cluster;
                    changed = true;
                }
            }
            
            if !changed {
                break;
            }
        }
        
        assignments
    }
}
```

## Phase 4: Consensus Generation (Week 7-8)

### 4.1 Simple Consensus

Implement in `src/consensus/poa.rs`:

```rust
use crate::reads::SequenceRead;
use crate::clustering::ConsensusSequence;

pub fn generate_consensus(reads: &[SequenceRead]) -> Result<ConsensusSequence> {
    if reads.is_empty() {
        return Err(anyhow::anyhow!("No reads for consensus"));
    }
    
    // Simple majority-vote consensus
    let max_len = reads.iter().map(|r| r.length).max().unwrap();
    let mut consensus = Vec::new();
    let mut quality = Vec::new();
    
    for pos in 0..max_len {
        let mut bases = [0u32; 5];  // A, C, G, T, N
        let mut quals = Vec::new();
        
        for read in reads {
            if pos < read.length {
                let base = read.sequence[pos];
                let idx = match base {
                    b'A' | b'a' => 0,
                    b'C' | b'c' => 1,
                    b'G' | b'g' => 2,
                    b'T' | b't' => 3,
                    _ => 4,
                };
                bases[idx] += 1;
                
                if let Some(ref q) = read.quality {
                    quals.push(q[pos]);
                }
            }
        }
        
        // Find majority base
        let (max_idx, _) = bases.iter()
            .enumerate()
            .max_by_key(|(_, &count)| count)
            .unwrap();
        
        let consensus_base = match max_idx {
            0 => b'A',
            1 => b'C',
            2 => b'G',
            3 => b'T',
            _ => b'N',
        };
        
        consensus.push(consensus_base);
        
        // Average quality
        let avg_q = if !quals.is_empty() {
            (quals.iter().map(|&q| q as u32).sum::<u32>() / quals.len() as u32) as u8
        } else {
            30
        };
        quality.push(avg_q);
    }
    
    Ok(ConsensusSequence {
        sequence: consensus.clone(),
        quality: quality.clone(),
        length: consensus.len(),
    })
}
```

## Phase 5: Metrics and Filtering (Week 9-10)

### 5.1 Diversity Calculation

Implement in `src/metrics/diversity.rs`:

```rust
use crate::reads::SequenceRead;

/// Calculate Shannon entropy as diversity metric
pub fn calculate_diversity(reads: &[SequenceRead]) -> f64 {
    if reads.len() <= 1 {
        return 0.0;
    }
    
    // Calculate pairwise differences
    let mut total_diff = 0.0;
    let mut comparisons = 0;
    
    for i in 0..reads.len() {
        for j in (i+1)..reads.len() {
            let diff = hamming_distance(&reads[i].sequence, &reads[j].sequence);
            total_diff += diff as f64;
            comparisons += 1;
        }
    }
    
    if comparisons == 0 {
        return 0.0;
    }
    
    total_diff / comparisons as f64
}

fn hamming_distance(seq1: &[u8], seq2: &[u8]) -> usize {
    seq1.iter()
        .zip(seq2.iter())
        .filter(|(a, b)| a != b)
        .count()
}
```

### 5.2 UCHIME Chimera Detection

Implement in `src/metrics/chimera.rs`:

```rust
pub fn calculate_uchime_score(
    query: &[u8],
    potential_parents: &[&[u8]],
) -> Option<(f64, usize, usize)> {
    // Simplified UCHIME implementation
    // Returns (score, left_parent_idx, right_parent_idx)
    
    if potential_parents.len() < 2 {
        return None;
    }
    
    let window_size = 100;
    let mut best_score = 0.0;
    let mut best_parents = (0, 1);
    
    for i in 0..potential_parents.len() {
        for j in (i+1)..potential_parents.len() {
            let score = compute_chimera_score(
                query,
                potential_parents[i],
                potential_parents[j],
                window_size,
            );
            
            if score > best_score {
                best_score = score;
                best_parents = (i, j);
            }
        }
    }
    
    Some((best_score, best_parents.0, best_parents.1))
}

fn compute_chimera_score(
    query: &[u8],
    parent_a: &[u8],
    parent_b: &[u8],
    window_size: usize,
) -> f64 {
    let len = query.len().min(parent_a.len()).min(parent_b.len());
    if len < window_size * 2 {
        return 0.0;
    }
    
    let mid = len / 2;
    
    // Similarity to parent A in left half
    let left_a = similarity(&query[..mid], &parent_a[..mid]);
    // Similarity to parent B in right half
    let right_b = similarity(&query[mid..], &parent_b[mid..]);
    
    // Chimera score: high if left matches A and right matches B
    (left_a + right_b) / 2.0
}

fn similarity(seq1: &[u8], seq2: &[u8]) -> f64 {
    let matches = seq1.iter()
        .zip(seq2.iter())
        .filter(|(a, b)| a == b)
        .count();
    matches as f64 / seq1.len() as f64
}
```

## Phase 6: Main Pipeline (Week 11-12)

### 6.1 Pipeline Implementation

Update `src/main.rs`:

```rust
fn run_clustering(config: ampliclust::Config) -> Result<()> {
    info!("Starting clustering pipeline...");
    
    // 1. Load references
    let references = if let Some(guide_path) = &config.guide {
        info!("Loading reference sequences from {:?}", guide_path);
        ampliclust::io::fasta::read_references(guide_path)?
    } else {
        vec![]
    };
    
    // 2. Load reads
    info!("Loading reads from {:?}", config.input);
    let reads = if config.from_bam {
        ampliclust::io::bam::read_bam(&config.input, config.platform)?
    } else {
        let mut reader = ampliclust::io::fastq::FastqReader::new(&config.input)?;
        reader.read_sequences(config.platform)?
    };
    info!("Loaded {} reads", reads.len());
    
    // 3. Filter reads
    info!("Filtering reads...");
    let filter = ampliclust::reads::filtering::ReadFilter::new(
        config.min_read_quality,
        config.max_amplicon_size,
        0,
    );
    let filtered_reads: Vec<_> = reads.iter()
        .filter(|r| filter.passes(r))
        .cloned()
        .collect();
    info!("Retained {} reads after filtering", filtered_reads.len());
    
    // 4. Place reads (if reference-guided)
    let mut result = ampliclust::clustering::ClusteringResult::new();
    
    if config.mode == "reference-guided" && !references.is_empty() {
        info!("Placing reads to references...");
        let placements = ampliclust::alignment::placement::place_reads(
            &filtered_reads,
            &references,
            config.kmer_size,
        )?;
        
        // Group reads by reference
        let mut groups: HashMap<usize, Vec<&SequenceRead>> = HashMap::new();
        for placement in placements {
            if let Some(read) = filtered_reads.iter().find(|r| r.id == placement.read_id) {
                groups.entry(placement.reference_id)
                    .or_insert_with(Vec::new)
                    .push(read);
            }
        }
        
        // 5. Cluster within each group
        info!("Clustering within reference groups...");
        for (ref_id, group_reads) in groups {
            let clusters = cluster_group(group_reads, &config)?;
            for mut cluster in clusters {
                cluster.guide_name = Some(references[ref_id].name.clone());
                result.add_cluster(cluster);
            }
        }
    } else {
        // De novo clustering
        info!("Performing de novo clustering...");
        let clusters = cluster_denovo(&filtered_reads, &config)?;
        for cluster in clusters {
            result.add_cluster(cluster);
        }
    }
    
    // 6. Calculate frequencies
    result.calculate_frequencies();
    
    // 7. Generate consensus
    info!("Generating consensus sequences...");
    for cluster in &mut result.clusters {
        let cluster_reads: Vec<_> = filtered_reads.iter()
            .filter(|r| cluster.reads.contains(&r.id))
            .cloned()
            .collect();
        
        if !cluster_reads.is_empty() {
            let consensus = ampliclust::consensus::poa::generate_consensus(&cluster_reads)?;
            cluster.set_consensus(consensus);
            
            // Calculate metrics
            cluster.metrics.diversity = ampliclust::metrics::diversity::calculate_diversity(&cluster_reads);
            cluster.metrics.avg_quality = cluster_reads.iter()
                .filter_map(|r| r.avg_quality)
                .sum::<f64>() / cluster_reads.len() as f64;
        }
    }
    
    // 8. Filter and write output
    info!("Writing output files...");
    write_output(&result, &config)?;
    
    info!("Pipeline complete!");
    Ok(())
}
```

## Testing Strategy

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_kmer_encoding() { ... }
    
    #[test]
    fn test_consensus_generation() { ... }
    
    #[test]
    fn test_clustering() { ... }
}
```

### Integration Tests
```rust
// tests/integration_tests.rs
#[test]
fn test_full_pipeline_pacbio() { ... }

#[test]
fn test_full_pipeline_ont() { ... }
```

## Next Steps

1. Implement each phase sequentially
2. Add comprehensive error handling
3. Add progress bars with `indicatif`
4. Optimize hot paths with profiling
5. Add comprehensive documentation
6. Create example datasets
7. Benchmark against pbaa

## Resources

- [Rust Bio](https://rust-bio.github.io/)
- [ndarray](https://docs.rs/ndarray/)
- [rayon](https://docs.rs/rayon/)
- [minimap2](https://github.com/lh3/minimap2)
