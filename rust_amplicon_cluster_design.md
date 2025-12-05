# AmpliClust - Universal Amplicon Clustering Tool

## Project Overview

**AmpliClust** is a reference-guided and de novo amplicon clustering tool designed to work with multiple sequencing platforms (PacBio HiFi, ONT, and Illumina). It provides sophisticated clustering algorithms, quality metrics, and flexible input/output options.

## Key Features

### 1. **Multi-Platform Support**
- PacBio HiFi reads (high quality, long reads)
- Oxford Nanopore (ONT) reads (variable quality, long reads)
- Illumina reads (high quality, short reads)

### 2. **Flexible Input Formats**
- FASTQ files (gzipped or uncompressed)
- BAM/SAM files (aligned or unaligned)
- FASTA files for guide sequences

### 3. **Clustering Modes**
- **Reference-guided**: Cluster reads based on similarity to reference loci
- **De novo**: Cluster reads without reference guidance
- **Hybrid**: Combine both approaches

### 4. **Comprehensive Metrics**
- Cluster frequency/abundance
- Read depth per cluster
- Consensus quality scores
- Cluster diversity metrics
- Chimera detection
- Platform-specific QC metrics

## Architecture Design

### Core Modules

```
ampliclust/
├── src/
│   ├── main.rs                    # Entry point and CLI
│   ├── lib.rs                     # Library exports
│   ├── io/
│   │   ├── mod.rs                 # I/O module exports
│   │   ├── fastq.rs               # FASTQ reader/writer
│   │   ├── bam.rs                 # BAM/SAM reader/writer
│   │   ├── fasta.rs               # FASTA reader/writer
│   │   └── formats.rs             # Format detection and conversion
│   ├── reads/
│   │   ├── mod.rs                 # Read handling exports
│   │   ├── sequence.rs            # Sequence representation
│   │   ├── quality.rs             # Quality score handling
│   │   ├── platform.rs            # Platform-specific processing
│   │   └── filtering.rs           # Quality filtering
│   ├── alignment/
│   │   ├── mod.rs                 # Alignment exports
│   │   ├── kmer.rs                # K-mer based alignment
│   │   ├── minimap.rs             # Minimap2 wrapper
│   │   ├── edlib.rs               # Edit distance alignment
│   │   └── placement.rs           # Read-to-reference placement
│   ├── clustering/
│   │   ├── mod.rs                 # Clustering exports
│   │   ├── kmeans.rs              # K-means clustering
│   │   ├── hierarchical.rs        # Hierarchical clustering
│   │   ├── dbscan.rs              # DBSCAN for variable cluster counts
│   │   ├── denovo.rs              # De novo clustering
│   │   └── reference_guided.rs    # Reference-guided clustering
│   ├── consensus/
│   │   ├── mod.rs                 # Consensus exports
│   │   ├── poa.rs                 # Partial Order Alignment
│   │   ├── spoa_wrapper.rs        # SPOA library wrapper
│   │   ├── polish.rs              # Consensus polishing
│   │   └── quality.rs             # Consensus quality calculation
│   ├── variants/
│   │   ├── mod.rs                 # Variant exports
│   │   ├── detection.rs           # Variant calling
│   │   ├── filtering.rs           # Variant filtering
│   │   └── graph.rs               # Variant graph construction
│   ├── metrics/
│   │   ├── mod.rs                 # Metrics exports
│   │   ├── cluster_stats.rs       # Cluster statistics
│   │   ├── diversity.rs           # Diversity metrics
│   │   ├── chimera.rs             # Chimera detection (UCHIME)
│   │   └── quality_control.rs     # QC metrics
│   ├── utils/
│   │   ├── mod.rs                 # Utility exports
│   │   ├── parallel.rs            # Parallelization helpers
│   │   ├── logging.rs             # Logging configuration
│   │   └── math.rs                # Mathematical utilities
│   └── config.rs                  # Configuration management
├── tests/
│   ├── integration_tests.rs
│   └── test_data/
├── benches/
│   └── benchmarks.rs
├── Cargo.toml
└── README.md
```

## Data Structures

### Read Representation
```rust
pub struct SequenceRead {
    pub id: String,
    pub sequence: Vec<u8>,
    pub quality: Option<Vec<u8>>,
    pub platform: Platform,
    pub length: usize,
    pub avg_quality: Option<f64>,
    pub metadata: HashMap<String, String>,
}

pub enum Platform {
    PacBio,
    ONT,
    Illumina,
    Unknown,
}
```

### Cluster Representation
```rust
pub struct Cluster {
    pub id: usize,
    pub guide_name: Option<String>,
    pub reads: Vec<String>, // Read IDs
    pub consensus: Option<ConsensusSequence>,
    pub metrics: ClusterMetrics,
}

pub struct ClusterMetrics {
    pub read_count: usize,
    pub frequency: f64,
    pub diversity: f64,
    pub avg_quality: f64,
    pub chimera_score: Option<f64>,
    pub chimera_parents: Option<(String, String)>,
    pub coverage_depth: f64,
    pub strand_bias: f64,
}
```

## Algorithm Workflow

### 1. Input Processing
- Detect input format (FASTQ/BAM/SAM)
- Parse reads with platform-specific handling
- Extract quality scores and metadata
- Apply initial quality filters

### 2. Read Placement (Reference-Guided Mode)
- Index reference sequences with k-mers
- Align reads to references using minimap2 or k-mer matching
- Calculate alignment scores
- Assign reads to reference groups
- Handle multi-mapping with probabilistic assignment

### 3. Variant Detection
- Build pileup for each reference group
- Detect SNVs and indels
- Filter variants by frequency and quality
- Construct variant graph

### 4. Clustering
- **Reference-guided**: 
  - Cluster reads within each reference group
  - Use variant profiles for distance calculation
  - K-means or hierarchical clustering
  
- **De novo**:
  - All-vs-all similarity calculation (optimized)
  - DBSCAN or hierarchical clustering
  - Automatic cluster count detection

### 5. Consensus Generation
- Use SPOA (SIMD Partial Order Alignment) for speed
- Platform-specific error correction
- Calculate consensus quality scores
- Polish consensus sequences

### 6. Filtering and QC
- Apply cluster frequency filters
- Detect chimeras using UCHIME algorithm
- Calculate diversity metrics
- Filter by minimum read count

### 7. Output Generation
- FASTA files with passed/failed clusters
- Detailed metrics report
- Read assignment table
- Optional BAM output with cluster tags

## Dependencies (Cargo.toml)

```toml
[dependencies]
# I/O and parsing
rust-htslib = "0.47"          # BAM/SAM/VCF handling
noodles = "0.70"              # Modern bioinformatics formats
bio = "1.6"                   # General bioinformatics
seq_io = "0.3"                # Fast FASTQ/FASTA parsing
flate2 = "1.0"                # Gzip compression

# Alignment
minimap2 = "0.1"              # Minimap2 bindings
edlib-rs = "0.1"              # Edit distance alignment
rust-bio-tools = "0.41"       # Additional bio tools

# Clustering and statistics
ndarray = "0.15"              # N-dimensional arrays
linfa = "0.7"                 # Machine learning (k-means)
linfa-clustering = "0.7"      # Clustering algorithms
statrs = "0.16"               # Statistical functions

# Parallelization
rayon = "1.8"                 # Data parallelism
crossbeam = "0.8"             # Concurrent data structures

# CLI and utilities
clap = { version = "4.5", features = ["derive"] }
anyhow = "1.0"                # Error handling
thiserror = "1.0"             # Error types
log = "0.4"                   # Logging facade
env_logger = "0.11"           # Logging implementation
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"            # JSON serialization
toml = "0.8"                  # TOML configuration

# Performance
ahash = "0.8"                 # Fast hashing
parking_lot = "0.12"          # Better synchronization primitives
```

## Command-Line Interface

```bash
# Reference-guided clustering
ampliclust cluster \
  --guide references.fasta \
  --input reads.fastq.gz \
  --output-prefix sample1 \
  --threads 8 \
  --platform pacbio

# De novo clustering
ampliclust cluster \
  --input reads.fastq.gz \
  --output-prefix sample1 \
  --mode denovo \
  --threads 8

# BAM input
ampliclust cluster \
  --guide references.fasta \
  --input aligned_reads.bam \
  --output-prefix sample1 \
  --from-bam

# Advanced options
ampliclust cluster \
  --guide references.fasta \
  --input reads.fastq.gz \
  --output-prefix sample1 \
  --min-read-quality 10 \
  --min-cluster-frequency 0.05 \
  --min-cluster-reads 3 \
  --max-chimera-score 1.0 \
  --kmer-size 15 \
  --max-reads-per-guide 1000 \
  --consensus-algorithm spoa
```

## Key Algorithms

### 1. K-mer Based Read Placement
- Fast initial placement of reads to references
- Uses minimizers for memory efficiency
- Similar to pbaa's approach

### 2. Variant Graph Construction
- Build graph of variant positions
- Use for clustering distance calculation
- Handles SNVs and indels

### 3. UCHIME Chimera Detection
- Adapted from vsearch implementation
- Detects chimeric consensus sequences
- Reports parent sequences

### 4. Adaptive Clustering
- Automatically determine optimal cluster count
- Use silhouette score or gap statistic
- Handle variable ploidy

### 5. Platform-Specific Error Correction
- **PacBio**: Focus on homopolymer errors
- **ONT**: Handle systematic errors
- **Illumina**: Quality score recalibration

## Output Files

### 1. Consensus Sequences
```
{prefix}_passed_clusters.fasta
{prefix}_failed_clusters.fasta
```

### 2. Cluster Metrics (TSV/JSON)
```
cluster_id  guide_name  read_count  frequency  diversity  avg_quality  chimera_score  status
```

### 3. Read Assignments (TSV)
```
read_id  cluster_id  guide_name  alignment_score  strand  read_length  read_quality
```

### 4. Summary Report (JSON)
```json
{
  "total_reads": 10000,
  "clustered_reads": 9500,
  "unclustered_reads": 500,
  "total_clusters": 45,
  "passed_clusters": 40,
  "failed_clusters": 5,
  "platform": "ONT",
  "processing_time": "120s"
}
```

## Performance Optimizations

1. **Memory Efficiency**
   - Stream processing of large files
   - Chunked read processing
   - Memory-mapped files for references

2. **Parallelization**
   - Thread-pool for read processing
   - Parallel clustering
   - Parallel consensus generation

3. **Algorithm Optimizations**
   - K-mer indexing with minimizers
   - SIMD operations where possible
   - Efficient distance calculations

## Testing Strategy

1. **Unit Tests**: Each module
2. **Integration Tests**: Full workflows
3. **Benchmark Tests**: Performance regression
4. **Test Data**: Multiple platforms, known outcomes

## Future Extensions

1. Real-time analysis (streaming mode)
2. GPU acceleration for alignment
3. Machine learning for cluster validation
4. Interactive visualization
5. Cloud deployment support

## Comparison with pbaa

| Feature | pbaa | AmpliClust |
|---------|------|------------|
| PacBio HiFi | ✓ | ✓ |
| ONT | ✗ | ✓ |
| Illumina | ✗ | ✓ |
| BAM Input | ✗ | ✓ |
| De novo | ✗ | ✓ |
| Reference-guided | ✓ | ✓ |
| Chimera detection | ✓ | ✓ |
| Language | C++ | Rust |

## Getting Started

See `IMPLEMENTATION_GUIDE.md` for step-by-step implementation instructions.
