# AmpliClust Quick Reference

## 🚀 Quick Start

```bash
# Build the project
cargo build --release

# Run reference-guided clustering
./target/release/ampliclust cluster \
  -g references.fasta \
  -i reads.fastq.gz \
  -o output \
  -p pacbio

# Run de novo clustering  
./target/release/ampliclust cluster \
  -i reads.fastq.gz \
  -o output \
  -m denovo
```

## 📋 Common Commands

```bash
# PacBio HiFi
ampliclust cluster -g refs.fasta -i reads.fq.gz -o out -p pacbio -j 8

# Oxford Nanopore
ampliclust cluster -g refs.fasta -i reads.fq.gz -o out -p ont -j 8

# Illumina
ampliclust cluster -g refs.fasta -i reads.fq.gz -o out -p illumina -j 8

# From BAM
ampliclust cluster -g refs.fasta -i aligned.bam -o out --from-bam

# BAM output with tags
ampliclust cluster -g refs.fasta -i reads.fq.gz -o out --output-bam

# Add tags to existing BAM
ampliclust bampaint -r out_read_info.txt -i input.bam -o tagged.bam

# Generate statistics
ampliclust stats -c out_passed.fasta -r out_read_info.txt -o stats.json
```

## ⚙️ Common Options

| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| `--guide` | `-g` | None | Reference FASTA |
| `--input` | `-i` | Required | Input FASTQ/BAM |
| `--output-prefix` | `-o` | Required | Output prefix |
| `--platform` | `-p` | auto | pacbio/ont/illumina |
| `--mode` | `-m` | reference-guided | denovo/hybrid |
| `--threads` | `-j` | 0 (auto) | Thread count |
| `--min-read-quality` | | 10 | Min Phred score |
| `--max-amplicon-size` | | 15000 | Max read length |
| `--kmer-size` | | 15 | K-mer size |
| `--min-cluster-frequency` | | 0.05 | Min cluster freq |
| `--min-cluster-reads` | | 5 | Min reads/cluster |
| `--max-chimera-score` | | 1.0 | Max UCHIME score |

## 📂 Output Files

```
{prefix}_passed_clusters.fasta    # High-quality clusters
{prefix}_failed_clusters.fasta    # Filtered clusters  
{prefix}_read_info.txt            # Read assignments
{prefix}_summary.json             # Analysis summary
{prefix}_tagged.bam               # BAM with cluster tags (optional)
```

## 🔧 Troubleshooting

| Problem | Solution |
|---------|----------|
| No clusters | Check `--log-level DEBUG`, verify references |
| Too many clusters | Increase `--min-cluster-frequency` |
| Low memory | Reduce `--max-reads-per-guide` |
| Slow performance | Increase `--threads`, reduce `--iterations` |
| Chimeras | Decrease `--max-chimera-score` |

## 📊 Platform Recommendations

### PacBio HiFi
```bash
--platform pacbio
--min-read-quality 20
--kmer-size 17
```

### Oxford Nanopore
```bash
--platform ont  
--min-read-quality 7
--kmer-size 15
--max-alignments-per-read 50
```

### Illumina
```bash
--platform illumina
--min-read-quality 20  
--kmer-size 21
```

## 🧬 File Formats

### Reference FASTA (with grouping)
```
>Allele1|LocusName
ACGTACGT...
>Allele2|LocusName  
ACGTCCGT...
```

### Read Info TSV
```
read_id  cluster_id  guide_name  score  strand  length  quality
read1    0           guide1      0.95   +       1000    42.3
```

### Summary JSON
```json
{
  "total_reads": 1000,
  "total_clusters": 10,
  "passed_clusters": 8,
  "platform": "PacBio"
}
```

## 🔬 Module Structure

```
ampliclust/
├── io/          # FASTQ, FASTA, BAM I/O
├── reads/       # SequenceRead, filtering
├── alignment/   # K-mer indexing, placement
├── clustering/  # K-means, DBSCAN, etc.
├── consensus/   # POA, SPOA
├── variants/    # Variant detection
├── metrics/     # Diversity, chimera, QC
└── utils/       # Parallel, logging, math
```

## 🎯 Key Data Structures

```rust
// Sequence read
pub struct SequenceRead {
    pub id: String,
    pub sequence: Vec<u8>,
    pub quality: Option<Vec<u8>>,
    pub platform: Platform,
}

// Cluster
pub struct Cluster {
    pub id: usize,
    pub guide_name: Option<String>,
    pub reads: Vec<String>,
    pub consensus: Option<ConsensusSequence>,
    pub metrics: ClusterMetrics,
}

// Metrics
pub struct ClusterMetrics {
    pub read_count: usize,
    pub frequency: f64,
    pub diversity: f64,
    pub avg_quality: f64,
    pub chimera_score: Option<f64>,
}
```

## 📚 Implementation Priority

1. ✅ Project setup
2. ⏳ FASTQ/FASTA I/O
3. ⏳ K-mer indexing
4. ⏳ Read placement
5. ⏳ Clustering algorithms
6. ⏳ Consensus generation
7. ⏳ Metrics calculation
8. ⏳ Output generation

## 💡 Tips

- Use `--log-level DEBUG` for troubleshooting
- Start with small test datasets
- Profile with `cargo flamegraph`
- Test each module independently
- Compare results with pbaa on demo data

## 📖 Documentation Links

- Design: `rust_amplicon_cluster_design.md`
- Implementation: `IMPLEMENTATION_GUIDE.md`
- Examples: `EXAMPLES.md`
- README: `AMPLICLUST_README.md`
- Summary: `PROJECT_SUMMARY.md`

## 🐛 Debugging

```bash
# Verbose logging
RUST_LOG=debug ampliclust cluster ...

# Backtrace on panic
RUST_BACKTRACE=1 ampliclust cluster ...

# Performance profiling
cargo install flamegraph
cargo flamegraph -- cluster ...
```
