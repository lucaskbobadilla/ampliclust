# AmpliClust - Universal Amplicon Clustering Tool

<p align="center">
  <img src="img/logo_pbaa.svg" alt="AmpliClust logo" width="250px"/>
</p>

A high-performance, multi-platform amplicon clustering tool written in Rust. AmpliClust supports PacBio HiFi, Oxford Nanopore (ONT), and Illumina sequencing data with both reference-guided and de novo clustering capabilities.

## 🚀 Features

- **Multi-Platform Support**: PacBio HiFi, ONT, and Illumina reads
- **Flexible Input**: FASTQ (gzipped or uncompressed) and BAM/SAM files
- **Dual Clustering Modes**: Reference-guided and de novo clustering
- **Comprehensive Metrics**: Frequency, diversity, quality, and chimera detection
- **High Performance**: Parallelized Rust implementation with SIMD optimizations
- **Robust Filtering**: Multiple quality control and filtering options

## 📋 Requirements

- Rust 1.70+ 
- Cargo (comes with Rust)

## 🔧 Installation

### From Source

```bash
git clone https://github.com/yourusername/ampliclust.git
cd ampliclust
cargo build --release
```

The binary will be in `target/release/ampliclust`.

### Using Cargo

```bash
cargo install ampliclust
```

## 📖 Quick Start

### Reference-Guided Clustering

```bash
ampliclust cluster \
  --guide references.fasta \
  --input reads.fastq.gz \
  --output-prefix sample1 \
  --platform pacbio \
  --threads 8
```

### De Novo Clustering

```bash
ampliclust cluster \
  --input reads.fastq.gz \
  --output-prefix sample1 \
  --mode denovo \
  --threads 8
```

### BAM Input

```bash
ampliclust cluster \
  --guide references.fasta \
  --input aligned_reads.bam \
  --output-prefix sample1 \
  --from-bam \
  --platform ont
```

## 📝 Usage

### Main Clustering Command

```
ampliclust cluster [OPTIONS] --input <INPUT> --output-prefix <PREFIX>

Options:
  -g, --guide <FILE>              Guide/reference sequences (FASTA)
  -i, --input <FILE>              Input reads (FASTQ/BAM)
  -o, --output-prefix <PREFIX>    Output file prefix
  -p, --platform <PLATFORM>       Sequencing platform [auto, pacbio, ont, illumina]
  -m, --mode <MODE>               Clustering mode [reference-guided, denovo, hybrid]
  -j, --threads <N>               Number of threads (0 = auto) [default: 0]

Quality Filtering:
  --min-read-quality <QV>         Minimum read quality (Phred) [default: 10]
  --max-amplicon-size <BP>        Maximum amplicon size [default: 15000]

Alignment Options:
  --kmer-size <K>                 K-mer size for placement [default: 15]
  --max-reads-per-guide <N>       Max reads per guide/locus [default: 1000]
  --max-alignments-per-read <N>   Max alignments per read [default: 100]

Clustering Options:
  --iterations <N>                Number of iterations [default: 10]
  --seed <N>                      Random seed [default: 42]

Consensus Options:
  --consensus-algorithm <ALG>     Algorithm [spoa, poa, simple] [default: spoa]
  --max-consensus-reads <N>       Max reads for consensus [default: 100]

Filtering Options:
  --min-cluster-frequency <F>     Min cluster frequency [default: 0.05]
  --min-cluster-reads <N>         Min reads per cluster [default: 5]
  --max-chimera-score <S>         Max UCHIME score [default: 1.0]
  --skip-chimera                  Skip chimera detection

Output Options:
  --output-bam                    Generate BAM with cluster tags
```

### BAM Painting

Add cluster tags to existing BAM files:

```bash
ampliclust bampaint \
  --read-info sample1_read_info.txt \
  --input-bam input.bam \
  --output-bam output_tagged.bam
```

### Generate Statistics

```bash
ampliclust stats \
  --clusters sample1_passed_clusters.fasta \
  --read-info sample1_read_info.txt \
  --output summary.json \
  --format json
```

## 📊 Output Files

AmpliClust generates several output files:

1. **`{prefix}_passed_clusters.fasta`** - High-quality cluster consensus sequences
2. **`{prefix}_failed_clusters.fasta`** - Filtered-out cluster sequences
3. **`{prefix}_read_info.txt`** - Read-to-cluster assignments
4. **`{prefix}_summary.json`** - Analysis summary and statistics
5. **`{prefix}_tagged.bam`** (optional) - BAM with cluster tags

### FASTA Header Format

```
>cluster_1_guide-HLA-A_reads-45 freq:0.225 diversity:0.15 quality:42.5 chimera:0.02 length:3152
```

### Read Info Format (TSV)

```
read_id    cluster_id    guide_name    alignment_score    strand    length    quality
read001    1             HLA-A         0.95               +         3150      42.3
read002    1             HLA-A         0.93               +         3148      41.8
```

## 🔬 Algorithm Overview

### Reference-Guided Mode

1. **Read Placement**: K-mer based alignment to reference sequences
2. **Variant Detection**: Build variant graph from aligned reads
3. **Clustering**: K-means or hierarchical clustering within guide groups
4. **Consensus Generation**: SPOA-based consensus with quality scores
5. **Filtering**: Apply frequency, quality, and chimera filters

### De Novo Mode

1. **Pairwise Similarity**: Calculate read-to-read distances
2. **Clustering**: DBSCAN or hierarchical clustering
3. **Consensus Generation**: Generate consensus for each cluster
4. **Filtering**: Quality-based filtering

## 🧬 Platform-Specific Considerations

### PacBio HiFi
- High accuracy (>QV20)
- Focus on homopolymer error correction
- Optimal k-mer size: 15-21

### Oxford Nanopore
- Variable quality
- More aggressive error correction
- Optimal k-mer size: 13-17
- Consider quality filtering

### Illumina
- Short reads
- High accuracy
- May require paired-end handling
- Optimal k-mer size: 21-31

## ⚡ Performance Tips

1. **Threading**: Use `-j 0` for automatic thread detection
2. **Memory**: For large datasets, reduce `--max-reads-per-guide`
3. **Speed**: Skip chimera detection with `--skip-chimera` if not needed
4. **Accuracy**: Increase `--iterations` for better clustering

## 🔍 Troubleshooting

### Too Many Clusters
- Increase `--min-cluster-frequency`
- Increase `--min-cluster-reads`
- Check for chimeric reads

### Missing Clusters
- Decrease `--min-cluster-frequency`
- Check read quality filters
- Verify guide sequences are appropriate

### Low Quality Consensus
- Increase `--max-consensus-reads`
- Check input read quality
- Adjust platform-specific parameters

## 📚 Comparison with pbaa

| Feature | pbaa | AmpliClust |
|---------|------|------------|
| PacBio HiFi | ✅ | ✅ |
| ONT | ❌ | ✅ |
| Illumina | ❌ | ✅ |
| BAM Input | ❌ | ✅ |
| De novo Clustering | ❌ | ✅ |
| Reference-guided | ✅ | ✅ |
| Chimera Detection | ✅ | ✅ |
| Language | C++ | Rust |
| Performance | Fast | Very Fast |

## 🤝 Contributing

Contributions are welcome! Please see `CONTRIBUTING.md` for guidelines.

## 📄 License

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

## 📖 Citation

If you use AmpliClust in your research, please cite:

```bibtex
@software{ampliclust2024,
  title = {AmpliClust: Universal Amplicon Clustering Tool},
  author = {Your Name},
  year = {2024},
  url = {https://github.com/yourusername/ampliclust}
}
```

## 🔗 Related Projects

- [pbaa](https://github.com/PacificBiosciences/pbbioconda) - PacBio Amplicon Analysis
- [IsoSeq3](https://github.com/PacificBiosciences/IsoSeq) - PacBio Isoform Sequencing
- [medaka](https://github.com/nanoporetech/medaka) - ONT consensus caller

## 📞 Support

- 📧 Email: your.email@example.com
- 🐛 Issues: [GitHub Issues](https://github.com/yourusername/ampliclust/issues)
- 💬 Discussions: [GitHub Discussions](https://github.com/yourusername/ampliclust/discussions)

## 🙏 Acknowledgments

Inspired by pbaa and designed to extend amplicon analysis capabilities across multiple sequencing platforms.
