# AmplicLust Project Structure

## 📁 Root Directory Layout

```
pbAA-1.2.0/
├── Cargo.toml              # Rust project configuration and dependencies
├── Cargo.lock              # Locked dependency versions
├── README.md               # Main project README (start here!)
├── PROJECT_STRUCTURE.md    # This file - project organization guide
│
├── src/                    # Rust source code
│   ├── main.rs            # CLI entry point and orchestration
│   ├── lib.rs             # Library exports
│   ├── io/                # Input/Output modules
│   ├── alignment/         # Alignment algorithms
│   ├── clustering/        # Clustering (Phase 3 - upcoming)
│   ├── consensus/         # Consensus calling (Phase 4 - upcoming)
│   └── ...               # Other modules
│
├── docs/                   # Documentation (all design, guides, and references)
│   ├── INDEX.md                          # Documentation index
│   ├── PROJECT_SUMMARY.md                # High-level overview
│   ├── QUICK_REFERENCE.md                # Quick command reference
│   ├── rust_amplicon_cluster_design.md   # Original design document
│   ├── ARCHITECTURE_DIAGRAMS.md          # System architecture
│   ├── IMPLEMENTATION_GUIDE.md           # Developer implementation guide
│   ├── DEVELOPMENT_CHECKLIST.md          # Development progress tracker
│   ├── TESTING_GUIDE.md                  # Comprehensive testing guide
│   ├── EXAMPLES.md                       # Usage examples
│   ├── START_HERE_TESTING.md             # Quick testing start
│   ├── PHASE_1_2_TESTING.md              # Phase 1 & 2 specific tests
│   ├── REAL_DATA_TEST_CHECKLIST.md       # Real data validation
│   ├── PHASE1_COMPLETE.md                # Phase 1 completion notes
│   ├── AMPLICLUST_README.md              # AmplicLust description
│   └── guide_reference.md                # Additional reference material
│
├── tests/                  # Test infrastructure and data
│   ├── run_tests.sh       # Automated test suite (run this!)
│   ├── test_data/         # Synthetic test data
│   │   ├── references.fasta       # 7 reference sequences (3 loci)
│   │   ├── reads.fastq            # 1000 synthetic reads
│   │   ├── reads.fastq.gz         # Gzipped version
│   │   ├── varied_quality.fastq   # Quality filtering test
│   │   └── simple_refs.fasta      # Negative control references
│   └── results/           # Test output files
│       └── test*_placements.txt   # Placement results from tests
│
├── examples/               # Example scripts and utilities
│   └── generate_test_data.py  # Python script to create synthetic data
│
├── img/                    # Images and logos
│   ├── logo_pbaa.svg
│   ├── pbaa_logo.png
│   ├── workflow.png
│   └── ...
│
└── target/                 # Cargo build artifacts (gitignored)
    ├── debug/             # Debug builds
    └── release/           # Release builds (optimized)
        └── ampliclust     # Main executable binary
```

---

## 🚀 Quick Start

### For New Users
1. Read `README.md` - project overview and quick start
2. Check `docs/INDEX.md` - documentation hub
3. Run `tests/run_tests.sh` - validate installation

### For Developers
1. Read `docs/rust_amplicon_cluster_design.md` - full design
2. Review `docs/ARCHITECTURE_DIAGRAMS.md` - system architecture
3. Follow `docs/IMPLEMENTATION_GUIDE.md` - implementation details
4. Track progress in `docs/DEVELOPMENT_CHECKLIST.md`

### For Testers
1. Start with `docs/START_HERE_TESTING.md`
2. Run automated tests: `bash tests/run_tests.sh`
3. Follow `docs/TESTING_GUIDE.md` for comprehensive testing
4. Use `docs/REAL_DATA_TEST_CHECKLIST.md` for validation

---

## 📊 Implementation Status

### ✅ Phase 1: Core I/O (COMPLETE)
- **Module**: `src/io/`
- **Tests**: 21 passing
- **Features**: FASTQ, FASTA, BAM reading/writing, format detection
- **Documentation**: `docs/PHASE1_COMPLETE.md`

### ✅ Phase 2: Alignment (COMPLETE)
- **Module**: `src/alignment/`
- **Tests**: 30 passing
- **Features**: K-mer indexing, read placement, minimap2-style alignment, edit distance
- **Documentation**: `docs/PHASE_1_2_TESTING.md`

### ⏳ Phase 3: Clustering (UPCOMING)
- **Module**: `src/clustering/`
- **Features**: Graph-based clustering, k-means, de novo clustering

### ⏳ Phase 4: Consensus Generation (UPCOMING)
- **Module**: `src/consensus/`
- **Features**: Multi-sequence alignment, quality-weighted consensus

### ⏳ Phase 5+: Advanced Features (PLANNED)
- Chimera detection
- Variant calling
- Performance optimization
- BAM painting for visualization

---

## 🧪 Testing Infrastructure

### Automated Test Suite
**Location**: `tests/run_tests.sh`

**Tests**:
1. ✅ Basic FASTQ with references (1000 reads → 1000 placements)
2. ✅ Gzipped FASTQ support
3. ✅ Quality filtering (500/1000 reads retained at Q≥25)
4. ✅ K-mer size variations (k=11, 15, 19)
5. ✅ Multi-threading performance
6. ✅ Negative control (unmatched references → 0 placements)

**Status**: All 6 tests passing ✓

### Unit Tests
**Location**: Throughout `src/` modules

**Command**: `cargo test`

**Status**: 51/51 tests passing ✓

---

## 📝 Documentation Categories

### User Documentation
- `README.md` - Quick start and overview
- `docs/QUICK_REFERENCE.md` - Command reference
- `docs/EXAMPLES.md` - Usage examples

### Developer Documentation
- `docs/rust_amplicon_cluster_design.md` - System design
- `docs/ARCHITECTURE_DIAGRAMS.md` - Architecture
- `docs/IMPLEMENTATION_GUIDE.md` - Implementation details
- `docs/DEVELOPMENT_CHECKLIST.md` - Progress tracking

### Testing Documentation
- `docs/START_HERE_TESTING.md` - Testing quick start
- `docs/TESTING_GUIDE.md` - Comprehensive testing guide
- `docs/PHASE_1_2_TESTING.md` - Phase 1 & 2 validation
- `docs/REAL_DATA_TEST_CHECKLIST.md` - Real data validation

### Reference Documentation
- `docs/INDEX.md` - Documentation hub
- `docs/PROJECT_SUMMARY.md` - Project summary
- `docs/AMPLICLUST_README.md` - AmplicLust description
- `docs/guide_reference.md` - Additional references

---

## 🔧 Building and Running

### Build Commands
```bash
# Debug build (fast compile, slower runtime)
cargo build

# Release build (slow compile, fast runtime)
cargo build --release

# Run unit tests
cargo test

# Run automated test suite
bash tests/run_tests.sh
```

### Binary Locations
- **Debug**: `./target/debug/ampliclust`
- **Release**: `./target/release/ampliclust` (use this for real data)

### Common Commands
```bash
# Reference-guided clustering
./target/release/ampliclust cluster \
  --guide references.fasta \
  --input reads.fastq \
  --output-prefix results \
  --platform pacbio

# See all options
./target/release/ampliclust cluster --help
```

---

## 📚 Additional Resources

### External Documentation
- [Rust Book](https://doc.rust-lang.org/book/)
- [rust-htslib docs](https://docs.rs/rust-htslib/)
- [bio crate docs](https://docs.rs/bio/)

### Related Tools
- [pbaa](https://github.com/PacificBiosciences/pbAA) - Original PacBio amplicon analysis
- [minimap2](https://github.com/lh3/minimap2) - Alignment inspiration
- [vsearch](https://github.com/torognes/vsearch) - Clustering reference

---

## 🤝 Contributing

AmplicLust is designed to be modular and extensible. When adding features:

1. Follow the existing module structure
2. Add unit tests (`#[cfg(test)]` modules)
3. Update documentation
4. Run the test suite before committing
5. Update `docs/DEVELOPMENT_CHECKLIST.md`

---

## 📄 License

See LICENSE file in repository root.

---

**Last Updated**: 2024 (Phase 1 & 2 Complete, Phase 3 In Planning)
