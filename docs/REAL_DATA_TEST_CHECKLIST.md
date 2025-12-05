# Real Data Testing Checklist for Phase 1 & 2

Use this checklist when testing AmplicLust with your actual amplicon sequencing data.

## 📋 Pre-Testing Setup

### ✅ Environment Setup
- [ ] Built release binary: `cargo build --release`
- [ ] Binary location: `./target/release/ampliclust`
- [ ] Test scripts are executable
- [ ] Have sufficient disk space (>10GB recommended)
- [ ] Have multiple CPU cores available

### ✅ Data Preparation
- [ ] Reference sequences in FASTA format
- [ ] References indexed (or will be auto-indexed)
- [ ] Read files in FASTQ or BAM format
- [ ] Know your sequencing platform (PacBio/ONT/Illumina)
- [ ] Have quality metrics for your data
- [ ] Backed up original data

### ✅ Quick Validation (Synthetic Data)
```bash
# Generate and test synthetic data first
./examples/generate_test_data.py
./run_tests.sh
```
- [ ] All 6 automated tests pass
- [ ] Placement files generated correctly
- [ ] Statistics look reasonable

---

## 🧪 Phase 1 Tests: Input/Output

### Test 1.1: FASTQ Reading
```bash
./target/release/ampliclust cluster \
  --guide YOUR_REFS.fasta \
  --input YOUR_READS.fastq \
  --output-prefix test1_fastq \
  --platform YOUR_PLATFORM \
  --threads 4
```

**Check:**
- [ ] Console shows: "Detected format: Fastq"
- [ ] Console shows: "Loaded N reads from FASTQ"
- [ ] Read count matches expected (use `grep "^@" reads.fastq | wc -l`)
- [ ] No error messages
- [ ] Process completes successfully

**Expected read count**: _____________
**Actual read count**: _____________

---

### Test 1.2: Gzipped FASTQ
```bash
./target/release/ampliclust cluster \
  --guide YOUR_REFS.fasta \
  --input YOUR_READS.fastq.gz \
  --output-prefix test2_gz \
  --platform YOUR_PLATFORM \
  --threads 4
```

**Check:**
- [ ] Console shows: "Detected format: FastqGz"
- [ ] Read count same as uncompressed
- [ ] No decompression errors
- [ ] Results match uncompressed version

**Gzipped read count**: _____________
**Matches uncompressed**: [ ] Yes [ ] No

---

### Test 1.3: BAM Input
```bash
./target/release/ampliclust cluster \
  --guide YOUR_REFS.fasta \
  --input YOUR_READS.bam \
  --output-prefix test3_bam \
  --platform YOUR_PLATFORM \
  --threads 4
```

**Check:**
- [ ] Console shows: "Detected format: Bam"
- [ ] Sequences extracted correctly
- [ ] Quality scores preserved (if present)
- [ ] Read names match original

**BAM read count**: _____________
**Quality scores present**: [ ] Yes [ ] No

---

### Test 1.4: Quality Filtering

#### High quality threshold (strict):
```bash
./target/release/ampliclust cluster \
  --guide YOUR_REFS.fasta \
  --input YOUR_READS.fastq \
  --output-prefix test4_strict \
  --min-read-quality 30 \
  --platform YOUR_PLATFORM
```

**Check:**
- [ ] Console shows filtering percentage
- [ ] Fewer reads retained than default
- [ ] Percentage makes sense for your data

**Input reads**: _____________
**Reads after filtering**: _____________
**Filtering rate**: _____________%

#### Low quality threshold (lenient):
```bash
./target/release/ampliclust cluster \
  --guide YOUR_REFS.fasta \
  --input YOUR_READS.fastq \
  --output-prefix test4_lenient \
  --min-read-quality 5 \
  --platform YOUR_PLATFORM
```

**Reads after filtering**: _____________
**Difference from strict**: _____________

---

### Test 1.5: Length Filtering
```bash
./target/release/ampliclust cluster \
  --guide YOUR_REFS.fasta \
  --input YOUR_READS.fastq \
  --output-prefix test5_length \
  --max-amplicon-size 5000 \
  --platform YOUR_PLATFORM
```

**Check:**
- [ ] Long reads are filtered out
- [ ] Reads within size retained
- [ ] Filtering statistics accurate

**Max amplicon size set**: _____________
**Reads after filtering**: _____________

---

### Test 1.6: Reference Loading
```bash
./target/release/ampliclust cluster \
  --guide YOUR_REFS.fasta \
  --input YOUR_READS.fastq \
  --output-prefix test6_refs \
  --platform YOUR_PLATFORM
```

**Check:**
- [ ] Console shows: "Loaded N reference sequences"
- [ ] Reference count matches file
- [ ] Grouping detected (if using pipe delimiter)
- [ ] All loci shown with correct counts

**Expected references**: _____________
**Loaded references**: _____________
**Number of loci/groups**: _____________

---

## 🎯 Phase 2 Tests: Alignment

### Test 2.1: K-mer Index Building
```bash
./target/release/ampliclust cluster \
  --guide YOUR_REFS.fasta \
  --input YOUR_READS.fastq \
  --output-prefix test7_kmer \
  --kmer-size 15 \
  --platform YOUR_PLATFORM
```

**Check k-mer statistics:**
- [ ] Console shows: "Building k-mer index"
- [ ] Unique k-mers shown
- [ ] Total k-mers shown
- [ ] Average occurrences calculated

**K-mer size**: _____________
**Unique k-mers**: _____________
**Total k-mers**: _____________
**Avg occurrences**: _____________

---

### Test 2.2: Read Placement
```bash
./target/release/ampliclust cluster \
  --guide YOUR_REFS.fasta \
  --input YOUR_READS.fastq \
  --output-prefix test8_placement \
  --platform YOUR_PLATFORM \
  --threads 8
```

**Check placement statistics:**
- [ ] Placement rate shown
- [ ] Average confidence calculated
- [ ] Average k-mer hits shown
- [ ] Reads per reference displayed

**Total reads**: _____________
**Placed reads**: _____________
**Unplaced reads**: _____________
**Placement rate**: _____________%
**Avg confidence**: _____________
**Avg k-mer hits**: _____________

**Success criteria:**
- [ ] Placement rate >70% (good data)
- [ ] Avg confidence >0.5
- [ ] Reads distribute to expected references

---

### Test 2.3: Placement Confidence
```bash
# Analyze confidence scores from placement file
tail -n +2 test8_placement_placements.txt | awk '
{
    if ($4 >= 0.9) high++
    else if ($4 >= 0.7) medium++
    else if ($4 >= 0.5) low++
    else very_low++
    total++
}
END {
    print "High (≥0.9):", high, "(" 100*high/total "%)"
    print "Medium (≥0.7):", medium, "(" 100*medium/total "%)"
    print "Low (≥0.5):", low, "(" 100*low/total "%)"
    print "Very low (<0.5):", very_low, "(" 100*very_low/total "%)"
}'
```

**Confidence distribution:**
- High (≥0.9): _____________
- Medium (≥0.7): _____________
- Low (≥0.5): _____________
- Very low (<0.5): _____________

**Check:**
- [ ] Most reads have high confidence (≥0.7)
- [ ] Few reads with very low confidence
- [ ] Distribution makes sense for data quality

---

### Test 2.4: Read Distribution
```bash
# Check reads per reference
tail -n +2 test8_placement_placements.txt | \
  cut -f3 | sort | uniq -c | sort -rn
```

**Top 5 references by read count:**
1. ________________________: _______ reads
2. ________________________: _______ reads
3. ________________________: _______ reads
4. ________________________: _______ reads
5. ________________________: _______ reads

**Check:**
- [ ] Distribution matches expected biology
- [ ] No unexpected reference getting many reads
- [ ] Allele ratios make sense (if known)

---

### Test 2.5: K-mer Size Optimization

Test different k-mer sizes:
```bash
for k in 11 13 15 17 19; do
  ./target/release/ampliclust cluster \
    --guide YOUR_REFS.fasta \
    --input YOUR_READS.fastq \
    --output-prefix test9_k${k} \
    --kmer-size $k \
    --platform YOUR_PLATFORM \
    --threads 4
  
  # Extract stats
  echo "k=$k:"
  grep "Placement rate:" test9_k${k}.log
  grep "Avg confidence:" test9_k${k}.log
done
```

**Results:**

| K-mer | Placement Rate | Avg Confidence | Notes |
|-------|---------------|----------------|-------|
| 11    | _____________% | _____________ |       |
| 13    | _____________% | _____________ |       |
| 15    | _____________% | _____________ |       |
| 17    | _____________% | _____________ |       |
| 19    | _____________% | _____________ |       |

**Best k-mer size for your data**: _____________

**Check:**
- [ ] Optimal k-mer identified
- [ ] Larger k gives higher confidence (usually)
- [ ] Smaller k gives better placement for noisy data

---

### Test 2.6: Multi-threading Performance
```bash
# Test with different thread counts
for t in 1 2 4 8; do
  echo "Testing with $t threads..."
  time ./target/release/ampliclust cluster \
    --guide YOUR_REFS.fasta \
    --input YOUR_READS.fastq \
    --output-prefix test10_t${t} \
    --threads $t \
    --platform YOUR_PLATFORM
done
```

**Performance results:**

| Threads | Runtime | Speedup | CPU Usage |
|---------|---------|---------|-----------|
| 1       | _______s | 1.0x   | ___%     |
| 2       | _______s | ___x   | ___%     |
| 4       | _______s | ___x   | ___%     |
| 8       | _______s | ___x   | ___%     |

**Check:**
- [ ] Multi-threading improves performance
- [ ] Speedup scales reasonably
- [ ] Results are identical across runs

---

### Test 2.7: Reproducibility
```bash
# Run same analysis twice
./target/release/ampliclust cluster \
  --guide YOUR_REFS.fasta \
  --input YOUR_READS.fastq \
  --output-prefix run1 \
  --platform YOUR_PLATFORM \
  --threads 4

./target/release/ampliclust cluster \
  --guide YOUR_REFS.fasta \
  --input YOUR_READS.fastq \
  --output-prefix run2 \
  --platform YOUR_PLATFORM \
  --threads 4

# Compare outputs
diff run1_placements.txt run2_placements.txt
```

**Check:**
- [ ] No differences in placement files
- [ ] Statistics are identical
- [ ] Results are reproducible

---

## 📊 Overall Validation

### Data Quality Assessment
- [ ] Read quality distribution checked
- [ ] Read length distribution examined
- [ ] Coverage per amplicon calculated
- [ ] Any quality issues noted

### Phase 1 (I/O) - PASS/FAIL
- [ ] All file formats work correctly
- [ ] Quality filtering functions as expected
- [ ] Length filtering is accurate
- [ ] No data loss or corruption
- [ ] Read counts are correct

**Phase 1 Status**: [ ] PASS [ ] FAIL

**Issues found**:
_________________________________________________________________
_________________________________________________________________

### Phase 2 (Alignment) - PASS/FAIL
- [ ] References load correctly
- [ ] K-mer indexing works
- [ ] Read placement succeeds
- [ ] Confidence scores are reasonable
- [ ] Distribution matches expectations
- [ ] Performance is acceptable
- [ ] Results are reproducible

**Phase 2 Status**: [ ] PASS [ ] FAIL

**Issues found**:
_________________________________________________________________
_________________________________________________________________

---

## 🎯 Platform-Specific Validation

### PacBio HiFi Data
- [ ] Placement rate >90%
- [ ] Avg confidence >0.85
- [ ] K-mer size 15-19 works well
- [ ] Min quality 20 appropriate

### ONT Data
- [ ] Placement rate >75%
- [ ] Avg confidence >0.70
- [ ] K-mer size 11-15 works well
- [ ] Min quality 10 appropriate
- [ ] Higher error rate handled

### Illumina Data
- [ ] Placement rate >85%
- [ ] Avg confidence >0.75
- [ ] K-mer size 11-13 works well
- [ ] Min quality 25 appropriate
- [ ] Short reads handled correctly

**Your platform**: _____________
**Platform-specific tests**: [ ] PASS [ ] FAIL

---

## 🐛 Issues Encountered

### Issue 1
**Description**: _________________________________________________
**Test**: _______________________________________________________
**Error message**: ______________________________________________
**Resolution**: _________________________________________________

### Issue 2
**Description**: _________________________________________________
**Test**: _______________________________________________________
**Error message**: ______________________________________________
**Resolution**: _________________________________________________

### Issue 3
**Description**: _________________________________________________
**Test**: _______________________________________________________
**Error message**: ______________________________________________
**Resolution**: _________________________________________________

---

## ✅ Final Sign-Off

### Ready for Production Use?
- [ ] All tests passed
- [ ] Performance acceptable
- [ ] Results validated
- [ ] No critical issues
- [ ] Documentation reviewed

**Tester name**: _____________
**Date**: _____________
**Overall status**: [ ] APPROVED [ ] NEEDS WORK

**Next steps**:
- [ ] Move to Phase 3 (Clustering) implementation
- [ ] Test with additional datasets
- [ ] Document any workarounds needed
- [ ] Share results with team

---

## 📝 Notes

Use this space for additional observations, insights, or recommendations:

_________________________________________________________________
_________________________________________________________________
_________________________________________________________________
_________________________________________________________________
_________________________________________________________________
_________________________________________________________________

---

**Save this completed checklist for your records and share with the development team!**
