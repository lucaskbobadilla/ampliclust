# AmpliClust Development Checklist

## ✅ Completed

- [x] Project structure created
- [x] Cargo.toml with all dependencies
- [x] Core data structures defined
  - [x] SequenceRead
  - [x] Cluster
  - [x] ClusterMetrics
  - [x] Config
- [x] CLI interface with clap
  - [x] cluster command
  - [x] bampaint command
  - [x] stats command
- [x] Module skeleton created
- [x] Comprehensive documentation
  - [x] Architecture design
  - [x] Implementation guide
  - [x] Examples and test cases
  - [x] Quick reference
  - [x] Project summary

## 🔨 To Implement

### Phase 1: Core I/O (Priority: HIGH) ✅ COMPLETE
- [x] `src/io/fastq.rs`
  - [x] FastqReader struct
  - [x] Gzip compression support
  - [x] Streaming for large files
  - [x] Error handling
  - [x] Unit tests (3 tests passing)

- [x] `src/io/fasta.rs`
  - [x] Reference loading
  - [x] Parse name|group format
  - [x] Fasta writer
  - [x] Unit tests (6 tests passing)

- [x] `src/io/bam.rs`
  - [x] BAM reader with rust-htslib
  - [x] BAM writer for tagged output
  - [x] Handle aligned/unaligned reads
  - [x] Unit tests (2 tests passing)

- [x] `src/io/formats.rs`
  - [x] Format auto-detection
  - [x] FOFN support
  - [x] Validation
  - [x] Unit tests (5 tests passing)

### Phase 2: Alignment (Priority: HIGH)
- [ ] `src/alignment/kmer.rs`
  - [ ] K-mer encoding (2-bit)
  - [ ] K-mer index structure
  - [ ] Reference indexing
  - [ ] Query function
  - [ ] Minimizer support
  - [ ] Unit tests

- [ ] `src/alignment/placement.rs`
  - [ ] Read placement algorithm
  - [ ] Scoring function
  - [ ] Handle multi-mapping
  - [ ] Parallel processing
  - [ ] Unit tests

- [ ] `src/alignment/edlib.rs`
  - [ ] Edit distance wrapper
  - [ ] Batch processing
  - [ ] Unit tests

- [ ] `src/alignment/minimap.rs` (Optional)
  - [ ] Minimap2 bindings
  - [ ] Configuration
  - [ ] Unit tests

### Phase 3: Clustering (Priority: HIGH)
- [ ] `src/clustering/reference_guided.rs`
  - [ ] Distance matrix computation
  - [ ] Parallel pairwise distances
  - [ ] Memory optimization
  - [ ] Unit tests

- [ ] `src/clustering/kmeans.rs`
  - [ ] K-means implementation
  - [ ] Centroid calculation
  - [ ] Convergence detection
  - [ ] Initialization strategies
  - [ ] Unit tests

- [ ] `src/clustering/hierarchical.rs`
  - [ ] Hierarchical clustering
  - [ ] Dendrogram construction
  - [ ] Cutting strategies
  - [ ] Unit tests

- [ ] `src/clustering/dbscan.rs`
  - [ ] DBSCAN implementation
  - [ ] Epsilon estimation
  - [ ] Noise handling
  - [ ] Unit tests

- [ ] `src/clustering/denovo.rs`
  - [ ] De novo clustering pipeline
  - [ ] Automatic cluster count
  - [ ] Unit tests

### Phase 4: Consensus (Priority: MEDIUM)
- [ ] `src/consensus/poa.rs`
  - [ ] POA implementation
  - [ ] Graph construction
  - [ ] Consensus extraction
  - [ ] Unit tests

- [ ] `src/consensus/spoa_wrapper.rs` (Optional)
  - [ ] C++ SPOA bindings
  - [ ] FFI safety
  - [ ] Unit tests

- [ ] `src/consensus/polish.rs`
  - [ ] Error correction
  - [ ] Platform-specific polishing
  - [ ] Unit tests

- [ ] `src/consensus/quality.rs`
  - [ ] Quality score calculation
  - [ ] Confidence metrics
  - [ ] Unit tests

### Phase 5: Variants (Priority: MEDIUM)
- [ ] `src/variants/detection.rs`
  - [ ] Pileup construction
  - [ ] SNV detection
  - [ ] Indel detection
  - [ ] Frequency filtering
  - [ ] Unit tests

- [ ] `src/variants/filtering.rs`
  - [ ] Quality filters
  - [ ] Coverage filters
  - [ ] Strand bias detection
  - [ ] Unit tests

- [ ] `src/variants/graph.rs`
  - [ ] Variant graph structure
  - [ ] Path enumeration
  - [ ] Unit tests

### Phase 6: Metrics (Priority: MEDIUM)
- [ ] `src/metrics/diversity.rs`
  - [ ] Shannon entropy
  - [ ] Simpson index
  - [ ] Pairwise differences
  - [ ] Unit tests

- [ ] `src/metrics/chimera.rs`
  - [ ] UCHIME implementation
  - [ ] Parent detection
  - [ ] Score calculation
  - [ ] Unit tests

- [ ] `src/metrics/cluster_stats.rs`
  - [ ] Coverage calculation
  - [ ] Frequency calculation
  - [ ] Summary statistics
  - [ ] Unit tests

- [ ] `src/metrics/quality_control.rs`
  - [ ] QC metrics
  - [ ] Platform-specific QC
  - [ ] Outlier detection
  - [ ] Unit tests

### Phase 7: Utilities (Priority: LOW)
- [ ] `src/utils/parallel.rs`
  - [ ] Thread pool helpers
  - [ ] Batch processing
  - [ ] Unit tests

- [ ] `src/utils/logging.rs`
  - [ ] Custom formatters
  - [ ] Progress bars
  - [ ] Unit tests

- [ ] `src/utils/math.rs`
  - [ ] Statistical functions
  - [ ] Distance metrics
  - [ ] Unit tests

### Phase 8: Pipeline Integration (Priority: HIGH)
- [ ] `src/main.rs` - Complete implementations
  - [ ] `run_clustering()`
    - [ ] Load references
    - [ ] Load reads
    - [ ] Filter reads
    - [ ] Place reads (if reference-guided)
    - [ ] Group reads
    - [ ] Cluster within groups
    - [ ] Generate consensus
    - [ ] Calculate metrics
    - [ ] Filter clusters
    - [ ] Write output
  - [ ] `run_bam_paint()`
    - [ ] Load read info
    - [ ] Read BAM
    - [ ] Add tags (HP, YC)
    - [ ] Write tagged BAM
  - [ ] `run_stats()`
    - [ ] Parse clusters
    - [ ] Calculate statistics
    - [ ] Generate report

- [ ] Output writers
  - [ ] FASTA writer with headers
  - [ ] TSV writer for read info
  - [ ] JSON writer for summary
  - [ ] BAM writer with tags

### Phase 9: Testing (Priority: HIGH)
- [ ] Unit tests for all modules
- [ ] Integration tests
  - [ ] End-to-end PacBio test
  - [ ] End-to-end ONT test
  - [ ] End-to-end Illumina test
  - [ ] De novo test
  - [ ] BAM input test
- [ ] Test data generation
  - [ ] Synthetic data generator
  - [ ] Known truth datasets
- [ ] Validation
  - [ ] Compare with pbaa
  - [ ] Accuracy metrics
  - [ ] Performance benchmarks

### Phase 10: Optimization (Priority: MEDIUM)
- [ ] Profile code
  - [ ] CPU profiling (flamegraph)
  - [ ] Memory profiling (heaptrack)
  - [ ] Identify bottlenecks
- [ ] Optimize hot paths
  - [ ] SIMD for distance calculations
  - [ ] Better data structures
  - [ ] Cache optimization
- [ ] Memory optimization
  - [ ] Streaming processing
  - [ ] Memory pools
  - [ ] Reduce allocations

### Phase 11: Documentation (Priority: MEDIUM)
- [ ] API documentation
  - [ ] Doc comments for all public items
  - [ ] Examples in doc comments
  - [ ] Generate rustdoc
- [ ] User guide
  - [ ] Installation instructions
  - [ ] Tutorial
  - [ ] Best practices
- [ ] Developer guide
  - [ ] Architecture overview
  - [ ] Contributing guidelines
  - [ ] Code style guide

### Phase 12: Release Preparation (Priority: LOW)
- [ ] CI/CD setup
  - [ ] GitHub Actions workflow
  - [ ] Automated testing
  - [ ] Automated builds
- [ ] Packaging
  - [ ] Cargo publish preparation
  - [ ] Binary releases
  - [ ] Conda package
  - [ ] Docker image
- [ ] Release artifacts
  - [ ] CHANGELOG.md
  - [ ] Version tagging
  - [ ] Release notes

## 📊 Progress Tracking

### Overall Progress
- Core structure: ✅ 100%
- I/O implementation: ✅ 100% (21 tests passing)
- Alignment: ⏳ 0%
- Clustering: ⏳ 0%
- Consensus: ⏳ 0%
- Metrics: ⏳ 0%
- Integration: ⏳ 0%
- Testing: ✅ 25%
- Documentation: ✅ 80%

### Estimated Timeline
- **Phase 1-2**: 4 weeks (I/O + Alignment)
- **Phase 3**: 2 weeks (Clustering)
- **Phase 4**: 2 weeks (Consensus)
- **Phase 5-6**: 3 weeks (Variants + Metrics)
- **Phase 7-8**: 2 weeks (Utils + Integration)
- **Phase 9**: 2 weeks (Testing)
- **Phase 10**: 1 week (Optimization)
- **Phase 11-12**: 1 week (Documentation + Release)

**Total: ~17 weeks (~4 months)**

## 🎯 Milestones

### Milestone 1: MVP (Week 8)
- [ ] Read FASTQ files
- [ ] Simple k-mer placement
- [ ] Basic k-means clustering
- [ ] Simple consensus
- [ ] FASTA output
- [ ] Works on small test dataset

### Milestone 2: Feature Complete (Week 12)
- [ ] All input formats (FASTQ, BAM, FASTA)
- [ ] Reference-guided and de novo
- [ ] All clustering algorithms
- [ ] Chimera detection
- [ ] All output formats
- [ ] Comprehensive metrics

### Milestone 3: Production Ready (Week 17)
- [ ] Fully tested (>80% coverage)
- [ ] Optimized performance
- [ ] Complete documentation
- [ ] Validated against pbaa
- [ ] CI/CD pipeline
- [ ] Ready for release

## 📝 Notes

### Design Decisions to Make
- [ ] Choose POA vs SPOA (or both?)
- [ ] Decide on default clustering algorithm
- [ ] Determine optimal k-mer size per platform
- [ ] Choose quality scoring method

### Known Challenges
- [ ] Memory efficiency for large datasets
- [ ] Accurate chimera detection
- [ ] Handling mixed-platform data
- [ ] Optimal parameter selection

### Future Enhancements
- [ ] GPU acceleration
- [ ] Real-time streaming analysis
- [ ] Machine learning for classification
- [ ] Web interface
- [ ] Cloud deployment

## 🔄 Development Workflow

1. **Pick a task** from this checklist
2. **Create a branch**: `git checkout -b feature/task-name`
3. **Implement** the feature with tests
4. **Run tests**: `cargo test`
5. **Check lints**: `cargo clippy`
6. **Format code**: `cargo fmt`
7. **Commit** with descriptive message
8. **Update** this checklist
9. **Merge** to main branch

## 📞 Need Help?

- Rust questions: [users.rust-lang.org](https://users.rust-lang.org)
- Algorithm questions: [Biostars](https://biostars.org)
- Design questions: Review `rust_amplicon_cluster_design.md`
- Implementation questions: Review `IMPLEMENTATION_GUIDE.md`
