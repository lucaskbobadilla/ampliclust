# AmpliClust Documentation Index

Welcome to the AmpliClust project documentation! This index will help you navigate all the documentation files.

## 📖 Getting Started

Start here if you're new to the project:

1. **[PROJECT_SUMMARY.md](PROJECT_SUMMARY.md)** - High-level project overview
   - What is AmpliClust?
   - Key features and advantages
   - Comparison with pbaa
   - Success metrics

2. **[AMPLICLUST_README.md](AMPLICLUST_README.md)** - User documentation
   - Installation instructions
   - Quick start guide
   - Usage examples
   - Output format description

3. **[QUICK_REFERENCE.md](QUICK_REFERENCE.md)** - Command cheat sheet
   - Common commands
   - Option reference
   - Troubleshooting tips
   - Platform recommendations

## Architecture & Design

Understand the system design:

4. **[rust_amplicon_cluster_design.md](rust_amplicon_cluster_design.md)** - Complete design document
   - Architecture overview
   - Module structure
   - Data structures
   - Algorithm descriptions
   - Dependencies

5. **[ARCHITECTURE_DIAGRAMS.md](ARCHITECTURE_DIAGRAMS.md)** - Visual diagrams
   - System architecture
   - Data flow diagrams
   - Module dependencies
   - Parallelization strategy

## 💻 Implementation

For developers implementing the system:

6. **[IMPLEMENTATION_GUIDE.md](IMPLEMENTATION_GUIDE.md)** - Step-by-step implementation
   - Phase 1: Core I/O
   - Phase 2: K-mer indexing
   - Phase 3: Clustering
   - Phase 4: Consensus
   - Phase 5: Metrics
   - Phase 6: Pipeline integration
   - Code examples for each phase

7. **[DEVELOPMENT_CHECKLIST.md](DEVELOPMENT_CHECKLIST.md)** - Task tracking
   - Completed items
   - To-do items organized by phase
   - Progress tracking
   - Milestones
   - Timeline estimates

## 🧪 Examples & Testing

Practical examples and test cases:

8. **[EXAMPLES.md](EXAMPLES.md)** - Usage examples
   - Real-world workflows
   - Test data generation
   - Expected outputs
   - Benchmarking procedures
   - Integration examples

## 📂 Code Structure

```
ampliclust/
│
├── Documentation (You are here!)
│   ├── PROJECT_SUMMARY.md          ← Start here
│   ├── AMPLICLUST_README.md        ← User guide
│   ├── QUICK_REFERENCE.md          ← Command reference
│   ├── rust_amplicon_cluster_design.md  ← Full design
│   ├── ARCHITECTURE_DIAGRAMS.md    ← Visual diagrams
│   ├── IMPLEMENTATION_GUIDE.md     ← How to implement
│   ├── DEVELOPMENT_CHECKLIST.md    ← Task tracking
│   ├── EXAMPLES.md                 ← Usage examples
│   └── INDEX.md                    ← This file
│
├── Source Code
│   ├── Cargo.toml                  ← Dependencies & config
│   ├── src/
│   │   ├── main.rs                 ← CLI entry point
│   │   ├── lib.rs                  ← Library exports
│   │   ├── config.rs               ← Configuration
│   │   ├── io/                     ← File I/O
│   │   ├── reads/                  ← Read handling
│   │   ├── alignment/              ← K-mer & alignment
│   │   ├── clustering/             ← Clustering algorithms
│   │   ├── consensus/              ← Consensus generation
│   │   ├── variants/               ← Variant detection
│   │   ├── metrics/                ← Quality metrics
│   │   └── utils/                  ← Utilities
│   │
│   └── tests/                      ← Test files
│
└── Original pbaa Documentation
    ├── README.md                   ← Original pbaa README
    ├── README_BETA.md              ← Beta version docs
    └── guide_reference.md          ← Guide setup info
```

## 📚 Reading Order

### For Users
1. PROJECT_SUMMARY.md (5 min)
2. AMPLICLUST_README.md (15 min)
3. QUICK_REFERENCE.md (5 min)
4. EXAMPLES.md (20 min)

**Total: ~45 minutes**

### For Developers
1. PROJECT_SUMMARY.md (5 min)
2. rust_amplicon_cluster_design.md (30 min)
3. ARCHITECTURE_DIAGRAMS.md (15 min)
4. IMPLEMENTATION_GUIDE.md (60 min)
5. DEVELOPMENT_CHECKLIST.md (10 min)

**Total: ~2 hours**

### For Contributors
1. All of the above
2. Review code in `src/`
3. Check tests in `tests/`
4. Read CONTRIBUTING.md (to be created)

## 🔍 Quick Lookup

### Need to find...

**A specific command?**
→ [QUICK_REFERENCE.md](QUICK_REFERENCE.md)

**How to implement a module?**
→ [IMPLEMENTATION_GUIDE.md](IMPLEMENTATION_GUIDE.md)

**Understanding the architecture?**
→ [ARCHITECTURE_DIAGRAMS.md](ARCHITECTURE_DIAGRAMS.md)

**Usage examples?**
→ [EXAMPLES.md](EXAMPLES.md)

**What's been done?**
→ [DEVELOPMENT_CHECKLIST.md](DEVELOPMENT_CHECKLIST.md)

**Full design details?**
→ [rust_amplicon_cluster_design.md](rust_amplicon_cluster_design.md)

**Project overview?**
→ [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md)

## 📊 Document Status

| Document | Status | Last Updated |
|----------|--------|--------------|
| PROJECT_SUMMARY.md | ✅ Complete | 2024-12-05 |
| AMPLICLUST_README.md | ✅ Complete | 2024-12-05 |
| QUICK_REFERENCE.md | ✅ Complete | 2024-12-05 |
| rust_amplicon_cluster_design.md | ✅ Complete | 2024-12-05 |
| ARCHITECTURE_DIAGRAMS.md | ✅ Complete | 2024-12-05 |
| IMPLEMENTATION_GUIDE.md | ✅ Complete | 2024-12-05 |
| DEVELOPMENT_CHECKLIST.md | ✅ Complete | 2024-12-05 |
| EXAMPLES.md | ✅ Complete | 2024-12-05 |
| INDEX.md | ✅ Complete | 2024-12-05 |

## 🎯 Key Concepts

### For Understanding the System

**Reference-Guided Clustering**
- Uses known reference sequences
- K-mer based read placement
- Clusters within reference groups
- Best for known loci (HLA, specific genes)

**De Novo Clustering**
- No reference required
- All-vs-all distance calculation
- Automatic cluster detection
- Best for unknown diversity (16S, viral quasispecies)

**K-mer Indexing**
- Convert DNA to 2-bit integers
- Build hash table of k-mer positions
- Fast read placement
- Memory efficient with minimizers

**Consensus Generation**
- Partial Order Alignment (POA)
- Create graph from multiple alignments
- Extract heaviest path
- Calculate quality scores

**Chimera Detection**
- UCHIME algorithm
- Detect PCR chimeras
- Compare to potential parents
- Score based on breakpoint detection

## 🚀 Next Steps

### If you're starting implementation:

1. ✅ Read PROJECT_SUMMARY.md
2. ✅ Review rust_amplicon_cluster_design.md
3. ✅ Study IMPLEMENTATION_GUIDE.md
4. ⏳ Start with Phase 1 (I/O)
5. ⏳ Follow DEVELOPMENT_CHECKLIST.md

### If you're a user:

1. ✅ Read AMPLICLUST_README.md
2. ✅ Try examples from EXAMPLES.md
3. ✅ Keep QUICK_REFERENCE.md handy
4. ⏳ Wait for implementation completion

## 📞 Support & Contact

- **Issues**: Create GitHub issue (when repository is public)
- **Questions**: Check QUICK_REFERENCE.md troubleshooting section
- **Contributions**: Follow DEVELOPMENT_CHECKLIST.md
- **Design questions**: Reference rust_amplicon_cluster_design.md

## 🔄 Version History

- **v0.1.0** (2024-12-05): Initial design and documentation
- Project structure created
- All documentation completed
- Ready for implementation Phase 1

## 📜 License

See LICENSE-MIT and LICENSE-APACHE in the project root.

## 🙏 Acknowledgments

This project was inspired by:
- **pbaa** - PacBio Amplicon Analysis tool
- The Rust bioinformatics community
- Open-source bioinformatics tools

## 🎓 Additional Resources

### Rust Learning
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)

### Bioinformatics
- [Rust-Bio](https://rust-bio.github.io/)
- [Bioinformatics Algorithms (Compeau & Pevzner)](http://bioinformaticsalgorithms.com/)

### Algorithms
- pbaa paper (when published)
- UCHIME paper: Edgar et al. 2011
- minimap2 paper: Li 2018

---

**Welcome to AmpliClust!** 🧬🦀

This documentation should provide everything you need to understand, implement, and use AmpliClust. Start with PROJECT_SUMMARY.md and follow your learning path from there.

Happy coding! 🚀
