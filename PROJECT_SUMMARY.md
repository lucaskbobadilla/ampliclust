# AmpliClust Project Summary

## 📊 Project Overview

**AmpliClust** is a universal amplicon clustering tool designed in Rust to overcome the limitations of platform-specific tools like pbaa. It provides:

✅ **Multi-platform support** - PacBio HiFi, Oxford Nanopore, and Illumina
✅ **Flexible input formats** - FASTQ and BAM files  
✅ **Reference-guided AND de novo clustering**
✅ **Comprehensive cluster metrics** - frequency, diversity, quality, chimera detection
✅ **High performance** - Parallelized Rust implementation

## 📁 Files Created

### Core Project Structure

1. **`Cargo.toml`** - Rust project configuration with all dependencies
2. **`src/main.rs`** - CLI implementation with clap
3. **`src/lib.rs`** - Library exports
4. **`src/config.rs`** - Configuration management

### Core Modules

5. **`src/reads/`** - Read handling and filtering
   - `mod.rs` - SequenceRead data structure
   - `quality.rs` - Quality score conversion
   - `platform.rs` - Platform-specific handling
   - `filtering.rs` - Read filtering logic

6. **`src/io/`** - File I/O (FASTQ, FASTA, BAM)
   - Placeholder files for implementation

7. **`src/alignment/`** - K-mer indexing and read placement
   - K-mer encoding and indexing
   - Minimap2 wrapper support
   - Read-to-reference placement

8. **`src/clustering/`** - Clustering algorithms
   - `mod.rs` - Cluster data structures
   - K-means, hierarchical, DBSCAN support
   - Reference-guided and de novo modes

9. **`src/consensus/`** - Consensus sequence generation
   - POA (Partial Order Alignment)
   - SPOA wrapper support
   - Consensus polishing

10. **`src/variants/`** - Variant detection and graph
    - Variant calling from pileups
    - Variant graph construction

11. **`src/metrics/`** - Quality metrics
    - `mod.rs` - ClusterMetrics data structure
    - Diversity calculation
    - UCHIME chimera detection
    - Quality control

12. **`src/utils/`** - Utility functions
    - Parallelization helpers
    - Logging configuration
    - Mathematical utilities

### Documentation

13. **`rust_amplicon_cluster_design.md`** - Complete architecture design
14. **`AMPLICLUST_README.md`** - User-facing README
15. **`IMPLEMENTATION_GUIDE.md`** - Step-by-step implementation guide
16. **`EXAMPLES.md`** - Usage examples and test cases
17. **`PROJECT_SUMMARY.md`** (this file) - Project summary

## 🎯 Key Advantages Over pbaa

| Feature | pbaa | AmpliClust |
|---------|------|------------|
| **Platform Support** | PacBio only | PacBio + ONT + Illumina |
| **Input Formats** | FASTQ only | FASTQ + BAM |
| **Clustering Mode** | Reference-guided | Reference-guided + De novo + Hybrid |
| **BAM Output** | Via separate tool | Built-in with tags |
| **Language** | C++ | Rust (memory-safe, modern) |
| **Parallelization** | OpenMP | Rayon (work-stealing) |
| **Error Handling** | C-style | Result types with anyhow |

## 🏗️ Architecture Highlights

### Data Flow

```
Input (FASTQ/BAM)
    ↓
Read Loading & QC Filtering
    ↓
[Reference-Guided Mode]          [De Novo Mode]
    ↓                                 ↓
K-mer Indexing                   Pairwise Distances
    ↓                                 ↓
Read Placement                   Distance Matrix
    ↓                                 ↓
Variant Detection                     ↓
    ↓                                 ↓
    └────────── Clustering ──────────┘
                  ↓
         Consensus Generation
                  ↓
         Metrics Calculation
                  ↓
         Filtering (Frequency, Chimera, Quality)
                  ↓
         Output (FASTA, TSV, JSON, BAM)
```

### Key Algorithms

1. **K-mer Indexing** - Fast read placement using 2-bit encoding
2. **Variant Graph** - Represent allelic diversity efficiently  
3. **K-means Clustering** - Iterative refinement of cluster assignments
4. **POA/SPOA** - Partial Order Alignment for consensus
5. **UCHIME** - Chimera detection comparing to putative parents

## 🚀 Implementation Roadmap

### Phase 1: Core I/O (2 weeks)
- [x] Project structure
- [ ] FASTQ reader/writer with gzip support
- [ ] FASTA reader for references
- [ ] BAM reader/writer with rust-htslib

### Phase 2: Alignment (2 weeks)
- [ ] K-mer encoding and indexing
- [ ] Read placement to references
- [ ] Minimap2 wrapper (optional)
- [ ] Edit distance calculations

### Phase 3: Clustering (2 weeks)
- [ ] Distance matrix computation
- [ ] K-means implementation
- [ ] DBSCAN for de novo
- [ ] Cluster assignment logic

### Phase 4: Consensus (2 weeks)
- [ ] Simple majority-vote consensus
- [ ] POA implementation
- [ ] SPOA wrapper (C++ binding)
- [ ] Quality score calculation

### Phase 5: Metrics (2 weeks)
- [ ] Diversity metrics (Shannon entropy)
- [ ] UCHIME chimera detection
- [ ] Frequency calculations
- [ ] Quality control metrics

### Phase 6: Integration (2 weeks)
- [ ] Full pipeline integration
- [ ] Output file generation
- [ ] BAM tagging functionality
- [ ] Progress bars and logging

### Phase 7: Testing & Optimization (2 weeks)
- [ ] Unit tests for all modules
- [ ] Integration tests
- [ ] Benchmarking vs pbaa
- [ ] Performance optimization
- [ ] Documentation

**Total Estimated Time**: 3-4 months for full implementation

## 📦 Dependencies

### Core Dependencies
- `clap` - CLI parsing
- `anyhow` / `thiserror` - Error handling
- `serde` / `serde_json` - Serialization
- `log` / `env_logger` - Logging

### Bioinformatics
- `rust-htslib` - BAM/SAM/VCF handling
- `bio` - General bioinformatics algorithms
- `seq_io` - Fast FASTQ parsing
- `minimap2` - Alignment (optional)

### Performance
- `rayon` - Data parallelism
- `ndarray` - N-dimensional arrays
- `ahash` - Fast hashing
- `parking_lot` - Better sync primitives

### Clustering & Stats
- `linfa` / `linfa-clustering` - ML algorithms
- `statrs` - Statistical functions

## 💻 Command Examples

### Basic Usage
```bash
ampliclust cluster \
  --guide refs.fasta \
  --input reads.fastq.gz \
  --output-prefix sample1 \
  --platform ont
```

### Advanced Usage
```bash
ampliclust cluster \
  --guide hla_alleles.fasta \
  --input patient.bam \
  --output-prefix patient_hla \
  --from-bam \
  --platform pacbio \
  --min-cluster-frequency 0.05 \
  --max-reads-per-guide 2000 \
  --skip-chimera \
  --output-bam \
  --threads 16
```

## 🎓 Learning Resources

### Rust Bioinformatics
- [Rust-Bio Book](https://rust-bio.github.io/)
- [Bioinformatics Algorithms in Rust](https://github.com/rust-bio/rust-bio)

### Algorithms
- pbaa source: [GitHub](https://github.com/PacificBiosciences/pbbioconda)
- UCHIME paper: Edgar et al. 2011, Bioinformatics
- minimap2 paper: Li, 2018, Bioinformatics

### Rust Performance
- [The Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Rayon Documentation](https://docs.rs/rayon/)

## 🔄 Next Steps

1. **Set up development environment**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   cargo new ampliclust
   # Copy files from this project
   ```

2. **Start with Phase 1** (I/O)
   - Implement FASTQ reader
   - Add tests with synthetic data
   - Verify memory efficiency

3. **Iterative development**
   - Complete one phase at a time
   - Add tests as you go
   - Profile performance regularly

4. **Validation**
   - Test on pbaa demo data
   - Compare results with pbaa
   - Optimize bottlenecks

## 📞 Getting Help

- Rust questions: [Rust Users Forum](https://users.rust-lang.org/)
- Bioinformatics: [Biostars](https://www.biostars.org/)
- Project issues: GitHub Issues (when public)

## 📈 Success Metrics

### Performance Goals
- **Speed**: Match or exceed pbaa performance
- **Memory**: Scale to millions of reads
- **Accuracy**: >99% cluster purity

### Code Quality Goals
- **Test Coverage**: >80%
- **Documentation**: All public APIs documented
- **Safety**: Zero unsafe code in core logic

## 🎉 Conclusion

You now have a complete design and starter code for **AmpliClust**, a modern, universal amplicon clustering tool. The project structure is set up, core data structures are defined, and you have detailed implementation guides.

**Start with Phase 1 (I/O)** and build incrementally. The modular design allows you to test each component independently before integrating the full pipeline.

Good luck with your implementation! 🚀
