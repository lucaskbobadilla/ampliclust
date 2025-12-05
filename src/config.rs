use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    // Input/Output
    pub guide: Option<PathBuf>,
    pub input: PathBuf,
    pub output_prefix: String,
    pub from_bam: bool,
    pub output_bam: bool,

    // Platform and mode
    pub platform: Platform,
    pub mode: String,

    // Quality filtering
    pub min_read_quality: f64,
    pub max_amplicon_size: usize,

    // Alignment
    pub kmer_size: usize,
    pub max_reads_per_guide: usize,
    pub max_alignments_per_read: usize,

    // Clustering
    pub iterations: usize,
    pub seed: u64,

    // Consensus
    pub consensus_algorithm: String,
    pub max_consensus_reads: usize,

    // Filtering
    pub min_cluster_frequency: f64,
    pub min_cluster_reads: usize,
    pub max_chimera_score: f64,
    pub skip_chimera: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Platform {
    PacBio,
    ONT,
    Illumina,
    Unknown,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            guide: None,
            input: PathBuf::from("input.fastq"),
            output_prefix: String::from("output"),
            from_bam: false,
            output_bam: false,
            platform: Platform::Unknown,
            mode: String::from("reference-guided"),
            min_read_quality: 10.0,
            max_amplicon_size: 15000,
            kmer_size: 15,
            max_reads_per_guide: 1000,
            max_alignments_per_read: 100,
            iterations: 10,
            seed: 42,
            consensus_algorithm: String::from("spoa"),
            max_consensus_reads: 100,
            min_cluster_frequency: 0.05,
            min_cluster_reads: 5,
            max_chimera_score: 1.0,
            skip_chimera: false,
        }
    }
}
