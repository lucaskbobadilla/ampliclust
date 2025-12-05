# AmplicLust Documentation

This folder contains all documentation for the AmplicLust project.

## 📖 Start Here

**New Users?** Start with these:
1. [`../README.md`](../README.md) - Main project README
2. [`QUICK_REFERENCE.md`](QUICK_REFERENCE.md) - Quick command reference
3. [`EXAMPLES.md`](EXAMPLES.md) - Usage examples

**Developers?** Read these:
1. [`rust_amplicon_cluster_design.md`](rust_amplicon_cluster_design.md) - Complete system design
2. [`ARCHITECTURE_DIAGRAMS.md`](ARCHITECTURE_DIAGRAMS.md) - Visual architecture
3. [`IMPLEMENTATION_GUIDE.md`](IMPLEMENTATION_GUIDE.md) - Implementation details
4. [`DEVELOPMENT_CHECKLIST.md`](DEVELOPMENT_CHECKLIST.md) - Progress tracking

**Testing?** Check these:
1. [`START_HERE_TESTING.md`](START_HERE_TESTING.md) - Quick testing guide
2. [`TESTING_GUIDE.md`](TESTING_GUIDE.md) - Comprehensive testing
3. [`PHASE_1_2_TESTING.md`](PHASE_1_2_TESTING.md) - Phase 1 & 2 validation
4. [`REAL_DATA_TEST_CHECKLIST.md`](REAL_DATA_TEST_CHECKLIST.md) - Real data checklist

---

## 📚 Documentation Index

### Overview & Quick References
| Document | Description |
|----------|-------------|
| [`INDEX.md`](INDEX.md) | Complete documentation hub |
| [`PROJECT_SUMMARY.md`](PROJECT_SUMMARY.md) | High-level project overview |
| [`QUICK_REFERENCE.md`](QUICK_REFERENCE.md) | Command cheat sheet |
| [`AMPLICLUST_README.md`](AMPLICLUST_README.md) | AmplicLust project description |

### Design & Architecture
| Document | Description |
|----------|-------------|
| [`rust_amplicon_cluster_design.md`](rust_amplicon_cluster_design.md) | **Complete system design** (start here for architecture) |
| [`ARCHITECTURE_DIAGRAMS.md`](ARCHITECTURE_DIAGRAMS.md) | Visual architecture diagrams |
| [`IMPLEMENTATION_GUIDE.md`](IMPLEMENTATION_GUIDE.md) | Developer implementation guide |
| [`DEVELOPMENT_CHECKLIST.md`](DEVELOPMENT_CHECKLIST.md) | Phase-by-phase progress tracker |

### Testing & Validation
| Document | Description |
|----------|-------------|
| [`START_HERE_TESTING.md`](START_HERE_TESTING.md) | Quick testing start guide |
| [`TESTING_GUIDE.md`](TESTING_GUIDE.md) | Comprehensive testing manual |
| [`PHASE_1_2_TESTING.md`](PHASE_1_2_TESTING.md) | Phase 1 & 2 specific tests |
| [`REAL_DATA_TEST_CHECKLIST.md`](REAL_DATA_TEST_CHECKLIST.md) | Real data validation checklist |
| [`PHASE1_COMPLETE.md`](PHASE1_COMPLETE.md) | Phase 1 completion summary |

### Usage & Examples
| Document | Description |
|----------|-------------|
| [`EXAMPLES.md`](EXAMPLES.md) | Usage examples and recipes |
| [`guide_reference.md`](guide_reference.md) | Additional reference material |

---

## 🎯 Quick Navigation

### I want to...

**...understand what AmplicLust does**
→ Read [`AMPLICLUST_README.md`](AMPLICLUST_README.md) and [`PROJECT_SUMMARY.md`](PROJECT_SUMMARY.md)

**...run my first analysis**
→ Check [`QUICK_REFERENCE.md`](QUICK_REFERENCE.md) and [`EXAMPLES.md`](EXAMPLES.md)

**...understand the system design**
→ Start with [`rust_amplicon_cluster_design.md`](rust_amplicon_cluster_design.md)

**...see how modules are organized**
→ Look at [`ARCHITECTURE_DIAGRAMS.md`](ARCHITECTURE_DIAGRAMS.md)

**...implement a new feature**
→ Follow [`IMPLEMENTATION_GUIDE.md`](IMPLEMENTATION_GUIDE.md) and update [`DEVELOPMENT_CHECKLIST.md`](DEVELOPMENT_CHECKLIST.md)

**...validate my changes**
→ Use [`TESTING_GUIDE.md`](TESTING_GUIDE.md) and run `../tests/run_tests.sh`

**...test with real data**
→ Follow [`REAL_DATA_TEST_CHECKLIST.md`](REAL_DATA_TEST_CHECKLIST.md)

**...see what's been completed**
→ Check [`DEVELOPMENT_CHECKLIST.md`](DEVELOPMENT_CHECKLIST.md) and [`PHASE1_COMPLETE.md`](PHASE1_COMPLETE.md)

---

## 📊 Current Status

### ✅ Phase 1: Core I/O - **COMPLETE**
- 21 unit tests passing
- FASTQ, FASTA, BAM support
- Format detection and validation

### ✅ Phase 2: Alignment - **COMPLETE**
- 30 unit tests passing
- K-mer indexing, read placement
- Minimap2-style alignment
- Edit distance calculations

### ✅ Phase 3: Clustering - **COMPLETE**
- 5 unit tests passing
- Reference-guided clustering
- Locus-based grouping
- Cluster frequency calculation

### ⏳ Phase 4: Consensus Generation - **NEXT**
- POA consensus generation
- Quality-weighted consensus
- Consensus polishing

### ⏳ Phase 4+: Advanced Features - **PLANNED**
- Consensus generation
- Variant calling
- Chimera detection

**Total Tests Passing**: 54/54 unit tests + 7/7 integration tests ✓

---

## 🔗 External Links

- **Main README**: [`../README.md`](../README.md)
- **Project Structure**: [`../PROJECT_STRUCTURE.md`](../PROJECT_STRUCTURE.md)
- **Source Code**: [`../src/`](../src/)
- **Tests**: [`../tests/`](../tests/)
- **Examples**: [`../examples/`](../examples/)

---

## 📝 Document Maintenance

When adding new documentation:
1. Add entry to this README
2. Update [`INDEX.md`](INDEX.md)
3. Cross-reference related documents
4. Update navigation sections

---

**Last Updated**: 2024 (Phase 1 & 2 Complete)
