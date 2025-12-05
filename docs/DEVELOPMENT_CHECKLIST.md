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

### Phase 2: Alignment (Priority: HIGH) ✅ COMPLETE
- [x] `src/alignment/kmer.rs`
  - [x] K-mer encoding (2-bit)
  - [x] K-mer index structure
  - [x] Reference indexing
  - [x] Query function
  - [x] Similarity scoring
  - [x] Unit tests (8 tests passing)

- [x] `src/alignment/placement.rs`
  - [x] Read placement algorithm
  - [x] Confidence scoring
  - [x] Handle multi-mapping
  - [x] Parallel processing with rayon
  - [x] Placement statistics
  - [x] Unit tests (5 tests passing)

- [x] `src/alignment/edlib.rs`
  - [x] Edit distance implementation (Wagner-Fischer)
  - [x] Bounded edit distance
  - [x] Sequence identity calculation
  - [x] Hamming distance
  - [x] Unit tests (11 tests passing)

- [x] `src/alignment/minimap.rs`
  - [x] Minimap2-style alignment
  - [x] Minimizer extraction
  - [x] Anchor chaining
  - [x] Platform presets (PacBio/ONT/Illumina)
  - [x] Unit tests (6 tests passing)

### Phase 3: Clustering (Priority: HIGH) ✅ COMPLETE
- [x] `src/clustering/mod.rs`
  - [x] Cluster data structure
  - [x] ConsensusSequence structure
  - [x] ClusteringResult structure
  - [x] Frequency calculation
  - [x] Filtering methods
  - [x] Unit tests (2 tests passing)

- [x] `src/clustering/reference_guided.rs`
  - [x] Cluster by placement (reference-based)
  - [x] Cluster by locus (hierarchical grouping)
  - [x] Subcluster by similarity
  - [x] Confidence threshold filtering
  - [x] Locus-aware grouping
  - [x] Unit tests (3 tests passing)

- [ ] `src/clustering/kmeans.rs`
  - [ ] K-means implementation
  - [ ] Centroid calculation
  - [ ] Convergence detection
  - [ ] Initialization strategies
  - [ ] Unit tests (STUB - for future phases)

- [ ] `src/clustering/hierarchical.rs`
  - [ ] Hierarchical clustering
  - [ ] Dendrogram construction
  - [ ] Cutting strategies
  - [ ] Unit tests (STUB - for future phases)

- [ ] `src/clustering/dbscan.rs`
  - [ ] DBSCAN implementation
  - [ ] Epsilon estimation
  - [ ] Noise handling
  - [ ] Unit tests (STUB - for future phases)

- [ ] `src/clustering/denovo.rs`
  - [ ] De novo clustering pipeline
  - [ ] Automatic cluster count
  - [ ] Unit tests (STUB - for future phases)

### Phase 4: Consensus (Priority: MEDIUM) ✅ COMPLETE
- [x] `src/consensus/mod.rs`
  - [x] Main consensus interface
  - [x] ConsensusMethod enum
  - [x] generate_consensus() function
  - [x] Unit tests (1 test passing)

- [x] `src/consensus/simple.rs`
  - [x] Simple majority-vote consensus
  - [x] Quality-weighted consensus
  - [x] Phred score conversion helpers
  - [x] Unit tests (6 tests passing)

- [ ] `src/consensus/poa.rs`
  - [ ] POA implementation
  - [ ] Graph construction
  - [ ] Consensus extraction
  - [ ] Unit tests (STUB - for future phases)

- [ ] `src/consensus/spoa_wrapper.rs` (Optional)
  - [ ] C++ SPOA bindings
  - [ ] FFI safety
  - [ ] Unit tests (STUB - for future phases)

- [ ] `src/consensus/polish.rs`
  - [ ] Error correction
  - [ ] Platform-specific polishing
  - [ ] Unit tests (STUB - for future phases)

- [ ] `src/consensus/quality.rs`
  - [ ] Quality score calculation
  - [ ] Confidence metrics
  - [ ] Unit tests (STUB - for future phases)

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

### Phase 8: Pipeline Integration (Priority: HIGH) ⏳ IN PROGRESS
- [x] `src/main.rs` - Partial implementation
  - [x] `run_clustering()` - Phases 1-3
    - [x] Load references
    - [x] Load reads (FASTQ/BAM/FASTA)
    - [x] Filter reads by quality/length
    - [x] Place reads (reference-guided)
    - [x] Group reads by reference/locus
    - [x] Cluster by placement
    - [ ] Generate consensus (Phase 4)
    - [ ] Calculate metrics (Phase 5-6)
    - [ ] Filter clusters (Phase 6)
    - [x] Write placement output
    - [x] Write cluster output
  - [ ] `run_bam_paint()` (STUB)
    - [ ] Load read info
    - [ ] Read BAM
    - [ ] Add tags (HP, YC)
    - [ ] Write tagged BAM
  - [ ] `run_stats()` (STUB)
    - [ ] Parse clusters
    - [ ] Calculate statistics
    - [ ] Generate report

- [x] Output writers (Partial)
  - [x] Placement TSV writer
  - [x] Cluster summary TSV writer
  - [x] Read-to-cluster mapping TSV writer
  - [ ] FASTA writer with headers (Phase 4)
  - [ ] JSON writer for summary (Phase 6)
  - [ ] BAM writer with tags (Phase 8)

### Phase 9: Testing (Priority: HIGH) ⏳ IN PROGRESS
- [x] Unit tests for Phase 1-3 modules (54 tests passing)
  - [x] I/O module tests (21 tests)
  - [x] Alignment module tests (30 tests)
  - [x] Clustering module tests (5 tests)
  - [x] Reads module tests (3 tests)
- [x] Integration tests
  - [x] End-to-end PacBio test (7 tests)
  - [x] FASTQ uncompressed test
  - [x] FASTQ gzipped test
  - [x] BAM input test
  - [x] Quality filtering test
  - [x] K-mer size variation test (k=11,15,19)
  - [x] Multi-threading test
  - [x] Negative control test
  - [ ] End-to-end ONT test
  - [ ] End-to-end Illumina test
  - [ ] De novo test (Phase 3 denovo module)
- [x] Test data generation
  - [x] Synthetic data generator (Python script)
  - [x] Known truth datasets (7 references, 1000 reads, 3 loci)
  - [x] BAM test data generation
  - [x] Quality-varied test data
- [x] Automated test suite
  - [x] Bash script for automated testing
  - [x] Test result validation
  - [x] Performance benchmarking
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
- Alignment: ✅ 100% (30 tests passing)
- Clustering: ✅ 60% (5 tests passing) - Reference-guided complete, denovo/kmeans/hierarchical/dbscan pending
- Consensus: ⏳ 0%
- Metrics: ⏳ 0%
- Integration: ✅ 40% (Phases 1-3 integrated)
- Testing: ✅ 70% (54 unit tests + 7 integration tests)
- Documentation: ✅ 95%

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

### Milestone 1: MVP (Week 8) ✅ ACHIEVED & EXCEEDED
- [x] Read FASTQ files (including gzipped)
- [x] Read BAM files
- [x] K-mer placement (with confidence scoring)
- [x] Reference-guided clustering (by placement)
- [x] Simple consensus (Phase 4 - COMPLETE)
- [x] Quality-weighted consensus (Phase 4 - COMPLETE)
- [x] Placement and cluster output (TSV format)
- [x] Consensus FASTA output (NEW)
- [x] Works on test datasets (1000+ reads)
- [x] 61 tests passing (54 unit + 7 integration)

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
