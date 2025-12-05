# Testing Guide for AmplicLust Phase 1 & 2

This guide provides step-by-step instructions for testing the implemented Phase 1 (I/O) and Phase 2 (Alignment) functionality using real data.

## ✅ What's Implemented

### Phase 1: Core I/O
- ✅ FASTQ reading (plain and gzipped)
- ✅ BAM reading
- ✅ FASTA reference loading
- ✅ Format auto-detection
- ✅ Quality filtering
- ✅ Length filtering
- ✅ Platform detection

### Phase 2: Alignment
- ✅ K-mer indexing (2-bit encoding)
- ✅ Read placement to references
- ✅ Confidence scoring
- ✅ Placement statistics
- ✅ Multi-threaded processing

## 🧪 Test Suite

### Test 1: FASTQ Input with Reference-Guided Alignment

**Purpose**: Verify FASTQ reading, reference loading, k-mer indexing, and read placement.

**Command**:
```bash
./target/release/ampliclust cluster \
  --guide references.fasta \
  --input reads.fastq \
  --output-prefix test1 \
  --platform pacbio \
  --kmer-size 15 \
  --min-read-quality 10 \
  --threads 4
```

**Expected Output**:
- `test1_placements.txt` - Tab-separated file with read placements
- Console output showing:
  - Number of reads loaded
  - Number of reads after filtering
  - Number of references loaded
  - K-mer index statistics
  - Placement statistics (placement rate, confidence, etc.)
  - Reads per reference

**What to Check**:
1. ✅ All reads are loaded successfully
2. ✅ Quality filtering works (check filtering percentage)
3. ✅ References are loaded and grouped correctly
4. ✅ K-mer index is built (check unique k-mers count)
5. ✅ Reads are placed with reasonable confidence (>0.5)
6. ✅ Placement rate is reasonable (>70% for good data)
7. ✅ Output file has correct format

---

### Test 2: Gzipped FASTQ Input

**Purpose**: Verify gzip decompression works correctly.

**Command**:
```bash
./target/release/ampliclust cluster \
  --guide references.fasta \
  --input reads.fastq.gz \
  --output-prefix test2_gz \
  --platform ont \
  --kmer-size 13 \
  --threads 4
```

**Expected Output**:
- Same as Test 1 but with gzipped input

**What to Check**:
1. ✅ Gzip format is auto-detected
2. ✅ Reads decompress correctly
3. ✅ Results match uncompressed version (same number of reads)

---

### Test 3: BAM Input

**Purpose**: Verify BAM reading functionality.

**Command**:
```bash
./target/release/ampliclust cluster \
  --guide references.fasta \
  --input reads.bam \
  --output-prefix test3_bam \
  --platform pacbio \
  --threads 4
```

**Expected Output**:
- Same placement output as FASTQ tests

**What to Check**:
1. ✅ BAM format is detected
2. ✅ Sequences are extracted correctly
3. ✅ Quality scores are preserved (if present)
4. ✅ Read names are preserved

---

### Test 4: Quality Filtering

**Purpose**: Test quality and length filtering thresholds.

**Command**:
```bash
# Strict filtering
./target/release/ampliclust cluster \
  --guide references.fasta \
  --input reads.fastq \
  --output-prefix test4_strict \
  --platform pacbio \
  --min-read-quality 30 \
  --max-amplicon-size 5000 \
  --threads 4
```

**Expected Output**:
- Fewer reads after filtering
- Higher quality reads retained

**What to Check**:
1. ✅ Filtering percentage changes appropriately
2. ✅ Low quality reads are removed
3. ✅ Reads longer than max size are removed
4. ✅ Console shows correct filtering stats

---

### Test 5: K-mer Size Optimization

**Purpose**: Test different k-mer sizes for different platforms.

**Commands**:
```bash
# Small k-mer (for higher error rates)
./target/release/ampliclust cluster \
  --guide refs.fasta \
  --input ont_reads.fastq \
  --output-prefix test5_k11 \
  --kmer-size 11 \
  --platform ont

# Medium k-mer (default)
./target/release/ampliclust cluster \
  --guide refs.fasta \
  --input hifi_reads.fastq \
  --output-prefix test5_k15 \
  --kmer-size 15 \
  --platform pacbio

# Large k-mer (for high accuracy)
./target/release/ampliclust cluster \
  --guide refs.fasta \
  --input hifi_reads.fastq \
  --output-prefix test5_k19 \
  --kmer-size 19 \
  --platform pacbio
```

**What to Check**:
1. ✅ Smaller k-mers give higher placement rates for noisy data
2. ✅ Larger k-mers give higher confidence for accurate data
3. ✅ K-mer index size changes appropriately

---

### Test 6: Multi-threading Performance

**Purpose**: Test parallel processing scalability.

**Commands**:
```bash
# Single thread
time ./target/release/ampliclust cluster \
  --guide refs.fasta \
  --input reads.fastq \
  --output-prefix test6_t1 \
  --threads 1

# Multiple threads
time ./target/release/ampliclust cluster \
  --guide refs.fasta \
  --input reads.fastq \
  --output-prefix test6_t8 \
  --threads 8
```

**What to Check**:
1. ✅ Multi-threaded version is faster
2. ✅ Results are identical (deterministic)
3. ✅ CPU utilization increases with threads

---

### Test 7: Reference Grouping

**Purpose**: Test reference grouping with pipe delimiter.

**Create test reference file** (`grouped_refs.fasta`):
```fasta
>Allele_A_1|Locus_A
ACGTACGTACGTACGT
>Allele_A_2|Locus_A
ACGTACGTACGTAGGT
>Allele_B_1|Locus_B
TGCATGCATGCATGCA
>Allele_B_2|Locus_B
TGCATGCATGCTTGCA
```

**Command**:
```bash
./target/release/ampliclust cluster \
  --guide grouped_refs.fasta \
  --input reads.fastq \
  --output-prefix test7_grouped \
  --platform pacbio
```

**What to Check**:
1. ✅ Console shows "Grouped into 2 loci"
2. ✅ Each locus shows correct number of sequences
3. ✅ Reads are placed to specific alleles but grouped by locus

---

### Test 8: Edge Cases

**Purpose**: Test error handling and edge cases.

#### 8a. Empty input file
```bash
touch empty.fastq
./target/release/ampliclust cluster \
  --guide refs.fasta \
  --input empty.fastq \
  --output-prefix test8a
```
**Expected**: Error message about no reads passing filters

#### 8b. Missing reference file
```bash
./target/release/ampliclust cluster \
  --guide nonexistent.fasta \
  --input reads.fastq \
  --output-prefix test8b
```
**Expected**: Clear error about missing file

#### 8c. Invalid format
```bash
echo "not a fastq" > invalid.fastq
./target/release/ampliclust cluster \
  --guide refs.fasta \
  --input invalid.fastq \
  --output-prefix test8c
```
**Expected**: Format detection or parsing error

#### 8d. Very short reads
```bash
./target/release/ampliclust cluster \
  --guide refs.fasta \
  --input short_reads.fastq \
  --output-prefix test8d \
  --max-amplicon-size 50
```
**Expected**: Most/all reads filtered out

---

## 📊 Interpreting Output

### Console Output Structure

```
[2024-12-05 10:00:00 INFO ampliclust] AmpliClust v0.1.0
[2024-12-05 10:00:00 INFO ampliclust] Using 8 threads
[2024-12-05 10:00:00 INFO ampliclust] Running clustering analysis...
[2024-12-05 10:00:00 INFO ampliclust] === Phase 1: Loading Input Data ===
[2024-12-05 10:00:00 INFO ampliclust] Loading reads from: "reads.fastq"
[2024-12-05 10:00:00 INFO ampliclust] Detected format: Fastq
[2024-12-05 10:00:01 INFO ampliclust] Loaded 1000 reads from FASTQ
[2024-12-05 10:00:01 INFO ampliclust] Filtering reads (min_quality=10, max_length=15000)
[2024-12-05 10:00:01 INFO ampliclust] After filtering: 950 reads retained (95.0%)
[2024-12-05 10:00:01 INFO ampliclust] === Phase 2: Reference-Guided Alignment ===
[2024-12-05 10:00:01 INFO ampliclust] Loading references from: "references.fasta"
[2024-12-05 10:00:01 INFO ampliclust] Loaded 10 reference sequences
[2024-12-05 10:00:01 INFO ampliclust] Grouped into 3 loci
[2024-12-05 10:00:01 INFO ampliclust]   HLA-A: 4 sequences
[2024-12-05 10:00:01 INFO ampliclust]   HLA-B: 3 sequences
[2024-12-05 10:00:01 INFO ampliclust]   HLA-C: 3 sequences
[2024-12-05 10:00:01 INFO ampliclust] Building k-mer index (k=15)
[2024-12-05 10:00:02 INFO ampliclust] K-mer index stats:
[2024-12-05 10:00:02 INFO ampliclust]   Unique k-mers: 15234
[2024-12-05 10:00:02 INFO ampliclust]   Total k-mers: 45678
[2024-12-05 10:00:02 INFO ampliclust]   Avg occurrences: 3.00
[2024-12-05 10:00:02 INFO ampliclust] Placing reads against references...
[2024-12-05 10:00:03 INFO ampliclust] Placement results:
[2024-12-05 10:00:03 INFO ampliclust]   Total reads: 950
[2024-12-05 10:00:03 INFO ampliclust]   Placed reads: 920
[2024-12-05 10:00:03 INFO ampliclust]   Unplaced reads: 30
[2024-12-05 10:00:03 INFO ampliclust]   Placement rate: 96.8%
[2024-12-05 10:00:03 INFO ampliclust]   Avg confidence: 0.923
[2024-12-05 10:00:03 INFO ampliclust]   Avg hits: 245.3
[2024-12-05 10:00:03 INFO ampliclust] Reads per reference:
[2024-12-05 10:00:03 INFO ampliclust]   HLA-A*01:01: 320 reads
[2024-12-05 10:00:03 INFO ampliclust]   HLA-A*02:01: 280 reads
[2024-12-05 10:00:03 INFO ampliclust]   HLA-B*07:02: 220 reads
[2024-12-05 10:00:03 INFO ampliclust]   HLA-B*08:01: 100 reads
[2024-12-05 10:00:03 INFO ampliclust] Writing placement results to: test_placements.txt
[2024-12-05 10:00:03 INFO ampliclust] Phase 1 & 2 complete! Next: implement clustering (Phase 3)
[2024-12-05 10:00:03 INFO ampliclust] Analysis complete!
```

### Placement Output File Format

The `*_placements.txt` file is tab-separated with columns:

```
read_id	ref_id	ref_name	confidence	hits	read_length
m64012_200712_164638/72090819/ccs	0	HLA-A*01:01	0.9234	245	3125
m64012_200712_164638/72090820/ccs	0	HLA-A*01:01	0.8901	223	3087
m64012_200712_164638/72090821/ccs	1	HLA-A*02:01	0.9456	256	3142
```

**Columns**:
- `read_id`: Original read identifier
- `ref_id`: Numeric reference ID (0-indexed)
- `ref_name`: Reference sequence name
- `confidence`: Placement confidence (0.0-1.0)
- `hits`: Number of k-mer hits
- `read_length`: Length of the read in bases

---

## 🎯 Success Criteria

### Phase 1 (I/O) is working if:
- ✅ FASTQ files load without errors
- ✅ Gzipped files decompress correctly
- ✅ BAM files are read successfully
- ✅ Quality filtering removes appropriate reads
- ✅ Length filtering works as expected
- ✅ Read counts are accurate

### Phase 2 (Alignment) is working if:
- ✅ References load and group correctly
- ✅ K-mer index builds successfully
- ✅ Placement rate >70% for good quality data
- ✅ Confidence scores are reasonable (>0.5 average)
- ✅ Reads distribute to correct references
- ✅ Multi-threading improves performance
- ✅ Results are reproducible

---

## 📈 Performance Benchmarks

Expected performance on typical hardware (8 cores, 16GB RAM):

| Dataset Size | Format | Time (threads=1) | Time (threads=8) | Memory |
|-------------|--------|------------------|------------------|---------|
| 1K reads    | FASTQ  | ~1 sec          | ~0.5 sec         | ~100 MB |
| 10K reads   | FASTQ  | ~5 sec          | ~1 sec           | ~500 MB |
| 100K reads  | FASTQ  | ~45 sec         | ~8 sec           | ~2 GB   |
| 1K reads    | BAM    | ~2 sec          | ~0.8 sec         | ~150 MB |

---

## 🐛 Common Issues and Solutions

### Issue: Low placement rate (<50%)

**Possible causes**:
1. K-mer size too large for data quality
2. Wrong platform specified
3. References don't match reads
4. Quality filtering too strict

**Solutions**:
- Try smaller k-mer size (11-13 for ONT, 13-15 for PacBio)
- Verify `--platform` matches your data
- Check references are correct for your experiment
- Lower `--min-read-quality`

### Issue: All reads filtered out

**Possible causes**:
1. Quality threshold too high
2. Length filter too restrictive
3. Wrong quality encoding

**Solutions**:
- Lower `--min-read-quality`
- Increase `--max-amplicon-size`
- Check if quality scores are in correct format

### Issue: Very low confidence scores

**Possible causes**:
1. K-mer size mismatch
2. High error rate data
3. References divergent from reads

**Solutions**:
- Adjust k-mer size for your data
- Verify data quality
- Ensure references are appropriate

### Issue: Slow performance

**Possible causes**:
1. Not using enough threads
2. Very large dataset
3. Disk I/O bottleneck

**Solutions**:
- Increase `--threads` to match CPU cores
- Process in batches
- Use SSD for data files

---

## 🔬 Real Data Test Checklist

Use this checklist when testing with your actual amplicon data:

### Pre-test Setup
- [ ] Install ampliclust: `cargo build --release`
- [ ] Prepare reference FASTA file
- [ ] Prepare read files (FASTQ or BAM)
- [ ] Check file formats are correct
- [ ] Note your sequencing platform

### Basic Functionality Tests
- [ ] Test 1: Load FASTQ successfully
- [ ] Test 2: Load references successfully
- [ ] Test 3: Build k-mer index
- [ ] Test 4: Place reads to references
- [ ] Test 5: Generate output file
- [ ] Test 6: Check placement statistics

### Quality Tests
- [ ] Placement rate >70%
- [ ] Average confidence >0.5
- [ ] Reads distribute to expected references
- [ ] No crashes or errors
- [ ] Output file is valid

### Performance Tests
- [ ] Test with 1 thread
- [ ] Test with max threads
- [ ] Measure speedup
- [ ] Check memory usage
- [ ] Verify results are identical

### Platform-Specific Tests
- [ ] Test with PacBio HiFi data (k=15-19)
- [ ] Test with ONT data (k=13-15)
- [ ] Test with Illumina data (k=11-13)
- [ ] Verify appropriate k-mer sizes

---

## 📝 Reporting Results

When reporting test results, please include:

1. **System Info**:
   - OS and version
   - CPU (cores/threads)
   - RAM available
   - Disk type (HDD/SSD)

2. **Data Info**:
   - Sequencing platform
   - Number of reads
   - Read length distribution
   - File format and size
   - Number of references

3. **Test Results**:
   - Command used
   - Console output (full log)
   - Placement statistics
   - Runtime
   - Memory usage
   - Output file samples

4. **Issues Encountered**:
   - Error messages
   - Unexpected behavior
   - Performance problems
   - Output discrepancies

---

## 🚀 Next Steps

After validating Phase 1 & 2:

1. **Implement Phase 3: Clustering**
   - Graph-based clustering
   - K-means clustering
   - De novo mode

2. **Implement Phase 4: Consensus**
   - Consensus sequence generation
   - Quality scoring
   - POA/SPOA integration

3. **Implement Phase 5: Variant Detection**
   - SNV calling
   - Indel detection
   - Variant filtering

Continue testing each phase as it's implemented!
