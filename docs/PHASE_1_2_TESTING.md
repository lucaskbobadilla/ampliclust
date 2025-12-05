# Phase 1 & 2 Testing Summary

## 🎯 What You Can Test Now

AmplicLust now has **full Phase 1 (I/O) and Phase 2 (Alignment)** functionality integrated into the CLI. You can test real amplicon sequencing data!

## 🚀 Quick Start

### 1. Generate Test Data
```bash
./examples/generate_test_data.py
```

This creates synthetic test data in `test_data/`:
- `references.fasta` - 7 reference sequences grouped into 3 loci
- `reads.fastq` - 1000 synthetic reads
- `reads.fastq.gz` - Gzipped version
- `varied_quality.fastq` - Reads with mixed quality

### 2. Run Automated Tests
```bash
./run_tests.sh
```

This runs 6 comprehensive tests covering:
- Basic FASTQ input
- Gzipped input
- Quality filtering
- K-mer size variations
- Multi-threading
- Simple references

### 3. Test with Your Real Data
```bash
./target/release/ampliclust cluster \
  --guide YOUR_REFERENCES.fasta \
  --input YOUR_READS.fastq \
  --output-prefix my_test \
  --platform pacbio \  # or 'ont' or 'illumina'
  --threads 8
```

## 📊 What to Expect

### Console Output
The program will show detailed progress:
```
[INFO] === Phase 1: Loading Input Data ===
[INFO] Loading reads from: "reads.fastq"
[INFO] Detected format: Fastq
[INFO] Loaded 1000 reads from FASTQ
[INFO] After filtering: 950 reads retained (95.0%)
[INFO] === Phase 2: Reference-Guided Alignment ===
[INFO] Loading references from: "references.fasta"
[INFO] Loaded 7 reference sequences
[INFO] Grouped into 3 loci
[INFO]   Locus_A: 3 sequences
[INFO]   Locus_B: 2 sequences
[INFO]   Locus_C: 2 sequences
[INFO] Building k-mer index (k=15)
[INFO] K-mer index stats:
[INFO]   Unique k-mers: 15234
[INFO]   Total k-mers: 45678
[INFO]   Avg occurrences: 3.00
[INFO] Placing reads against references...
[INFO] Placement results:
[INFO]   Total reads: 950
[INFO]   Placed reads: 920
[INFO]   Unplaced reads: 30
[INFO]   Placement rate: 96.8%
[INFO]   Avg confidence: 0.923
[INFO]   Avg hits: 245.3
[INFO] Reads per reference:
[INFO]   Allele_A_01|Locus_A: 380 reads
[INFO]   Allele_A_02|Locus_A: 285 reads
[INFO]   Allele_A_03|Locus_A: 95 reads
[INFO]   Allele_B_01|Locus_B: 145 reads
[INFO]   Allele_B_02|Locus_B: 15 reads
[INFO] Writing placement results to: test_placements.txt
```

### Output Files

**`{prefix}_placements.txt`** - Tab-separated placement results:
```
read_id	ref_id	ref_name	confidence	hits	read_length
read_00001_from_Allele_A_01	0	Allele_A_01|Locus_A	0.9234	245	3125
read_00002_from_Allele_A_01	0	Allele_A_01|Locus_A	0.8901	223	3087
read_00003_from_Allele_A_02	1	Allele_A_02|Locus_A	0.9456	256	3142
```

## 🧪 Comprehensive Test List

### ✅ Automated Tests (run_tests.sh)
1. **Basic FASTQ** - Standard input
2. **Gzipped FASTQ** - Compressed files
3. **Quality filtering** - Filter low-quality reads
4. **K-mer variations** - Test k=11, 15, 19
5. **Multi-threading** - Parallel processing
6. **Simple references** - Ungrouped references

### 📋 Manual Tests for Real Data

#### Test 1: PacBio HiFi Data
```bash
./target/release/ampliclust cluster \
  --guide hla_references.fasta \
  --input hifi_amplicons.fastq \
  --output-prefix pacbio_test \
  --platform pacbio \
  --kmer-size 15 \
  --min-read-quality 20 \
  --threads 8
```

**Expected**:
- Placement rate: >90%
- Avg confidence: >0.85
- High k-mer hit counts

#### Test 2: ONT Data
```bash
./target/release/ampliclust cluster \
  --guide references.fasta \
  --input ont_amplicons.fastq \
  --output-prefix ont_test \
  --platform ont \
  --kmer-size 13 \
  --min-read-quality 10 \
  --threads 8
```

**Expected**:
- Placement rate: >75% (higher error rate)
- Avg confidence: >0.70
- May need smaller k-mer for better placement

#### Test 3: Illumina Data
```bash
./target/release/ampliclust cluster \
  --guide references.fasta \
  --input illumina_amplicons.fastq \
  --output-prefix illumina_test \
  --platform illumina \
  --kmer-size 11 \
  --min-read-quality 25 \
  --threads 8
```

**Expected**:
- Placement rate: >85%
- Works best with smaller k-mers
- Short read length limits

#### Test 4: BAM Input
```bash
./target/release/ampliclust cluster \
  --guide references.fasta \
  --input aligned_reads.bam \
  --output-prefix bam_test \
  --platform pacbio \
  --threads 8
```

**Expected**:
- BAM format auto-detected
- Sequences extracted correctly
- Quality scores preserved

#### Test 5: Multi-Allelic Loci (HLA)
```bash
./target/release/ampliclust cluster \
  --guide hla_alleles.fasta \
  --input hla_amplicons.fastq \
  --output-prefix hla_test \
  --platform pacbio \
  --kmer-size 17 \
  --threads 8
```

**Check**:
- Reads group by locus
- Multiple alleles per locus detected
- Even distribution if heterozygous

#### Test 6: Quality Filtering Thresholds
```bash
# Strict filtering
./target/release/ampliclust cluster \
  --guide refs.fasta \
  --input reads.fastq \
  --output-prefix strict \
  --min-read-quality 30 \
  --max-amplicon-size 5000 \
  --threads 8

# Lenient filtering
./target/release/ampliclust cluster \
  --guide refs.fasta \
  --input reads.fastq \
  --output-prefix lenient \
  --min-read-quality 5 \
  --max-amplicon-size 20000 \
  --threads 8
```

**Compare**:
- Number of reads retained
- Placement rates
- Confidence scores

#### Test 7: K-mer Size Optimization
```bash
for k in 11 13 15 17 19; do
  ./target/release/ampliclust cluster \
    --guide refs.fasta \
    --input reads.fastq \
    --output-prefix kmer_$k \
    --kmer-size $k \
    --platform pacbio \
    --threads 8
done

# Compare placement rates and confidence
grep "Placement rate" kmer_*_placements.txt
```

#### Test 8: Large Dataset Performance
```bash
# Test with 10K+ reads
time ./target/release/ampliclust cluster \
  --guide refs.fasta \
  --input large_dataset.fastq \
  --output-prefix large_test \
  --threads 16 \
  --platform pacbio
```

**Monitor**:
- Runtime
- Memory usage
- CPU utilization
- Placement statistics

## ✅ Success Criteria

### Phase 1 (I/O) Validation
- [ ] FASTQ files load without errors
- [ ] Gzipped files decompress correctly
- [ ] BAM files parse successfully
- [ ] Format auto-detection works
- [ ] Quality filtering removes appropriate reads
- [ ] Length filtering works as expected
- [ ] Read counts are accurate
- [ ] No memory leaks

### Phase 2 (Alignment) Validation
- [ ] References load correctly
- [ ] Reference grouping works (pipe delimiter)
- [ ] K-mer index builds successfully
- [ ] K-mer statistics are reasonable
- [ ] Placement rate >70% for good data
- [ ] Confidence scores >0.5 average
- [ ] K-mer hits correlate with confidence
- [ ] Reads distribute to correct references
- [ ] Multi-threading improves performance
- [ ] Results are reproducible (same input = same output)

## 📈 Interpreting Results

### Good Results
```
Placement rate: 95.0%         ← Most reads placed successfully
Avg confidence: 0.923         ← High confidence placements
Avg hits: 245.3               ← Many k-mer matches
```

### Needs Tuning
```
Placement rate: 45.0%         ← Low placement - adjust k-mer size
Avg confidence: 0.412         ← Low confidence - check references
Avg hits: 23.1                ← Few matches - k-mer too large?
```

### Platform-Specific Expectations

| Platform | Placement Rate | Avg Confidence | Recommended k |
|----------|---------------|----------------|---------------|
| PacBio HiFi | >90% | >0.85 | 15-19 |
| ONT | >75% | >0.70 | 11-15 |
| Illumina | >85% | >0.75 | 11-13 |

## 🐛 Troubleshooting

### Low placement rate
- Try smaller k-mer size
- Check platform setting matches data
- Verify references match your amplicons
- Lower quality threshold

### All reads filtered
- Lower `--min-read-quality`
- Increase `--max-amplicon-size`
- Check input file quality encoding

### Slow performance
- Increase `--threads`
- Use SSD for data files
- Process in batches for very large datasets

### Unexpected read distribution
- Verify reference sequences
- Check for PCR bias in library prep
- Review placement confidence scores

## 📝 Validation Checklist for Real Data

Before using with production data, validate:

- [ ] **Input loading**: All reads load correctly
- [ ] **Format detection**: Correct format detected
- [ ] **Quality metrics**: Filtering works as expected
- [ ] **Reference loading**: All references loaded
- [ ] **Grouping**: Loci grouped correctly
- [ ] **K-mer index**: Reasonable statistics
- [ ] **Placement**: Good placement rate
- [ ] **Confidence**: High average confidence
- [ ] **Distribution**: Reads distribute correctly
- [ ] **Performance**: Acceptable runtime
- [ ] **Reproducibility**: Same results on re-run
- [ ] **Output format**: Valid placement file

## 🎉 Next Steps

Once Phase 1 & 2 are validated:

### Phase 3: Clustering (Next Priority)
- Implement graph-based clustering
- Use placement results to group reads
- Support both reference-guided and de novo modes
- Generate initial cluster assignments

### Phase 4: Consensus Generation
- Create consensus sequences for each cluster
- Calculate quality scores
- Implement POA/SPOA alignment

### Phase 5: Variant Detection
- Call SNVs and indels
- Track variant frequencies
- Support multi-allelic sites

### Phase 6-12: Advanced Features
- Chimera detection
- Statistical models
- Visualization
- Full CLI integration

## 📚 Documentation

- **TESTING_GUIDE.md** - Comprehensive testing procedures
- **AMPLICLUST_README.md** - User documentation
- **IMPLEMENTATION_GUIDE.md** - Developer guide
- **test_data/README.md** - Test data description

## 🤝 Reporting Issues

When reporting test results or issues, include:

1. **Command used**
2. **Console output** (full log)
3. **Input file info** (format, size, platform)
4. **Expected vs actual results**
5. **System specs** (OS, CPU, RAM)

Happy testing! 🧪✨
