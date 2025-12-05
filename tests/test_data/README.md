# Test Data for AmplicLust

This directory contains synthetic test data for validating Phase 1 & 2.

## Files

### References
- **references.fasta**: 7 reference sequences grouped into 3 loci
  - Locus_A: 3 alleles (Allele_A_01, Allele_A_02, Allele_A_03)
  - Locus_B: 2 alleles (Allele_B_01, Allele_B_02)
  - Locus_C: 2 alleles (Allele_C_01, Allele_C_02)

- **simple_refs.fasta**: 3 ungrouped references for basic testing

### Reads
- **reads.fastq**: 1000 reads from grouped references
  - 400 from Allele_A_01 (40%)
  - 300 from Allele_A_02 (30%)
  - 100 from Allele_A_03 (10%)
  - 150 from Allele_B_01 (15%)
  - 50 from Allele_B_02 (5%)
  - Each read has ~0.1% error rate from reference
  - Average quality: Q35

- **reads.fastq.gz**: Gzipped version of reads.fastq

- **varied_quality.fastq**: 500 reads with mixed quality
  - 80% high quality (Q30-Q45)
  - 20% low quality (Q5-Q20)
  - For testing quality filtering

## Usage Examples

### Test basic functionality:
```bash
./target/release/ampliclust cluster \
  --guide test_data/references.fasta \
  --input test_data/reads.fastq \
  --output-prefix test_basic \
  --platform pacbio \
  --threads 4
```

### Test gzipped input:
```bash
./target/release/ampliclust cluster \
  --guide test_data/references.fasta \
  --input test_data/reads.fastq.gz \
  --output-prefix test_gzipped \
  --platform pacbio
```

### Test quality filtering:
```bash
./target/release/ampliclust cluster \
  --guide test_data/references.fasta \
  --input test_data/varied_quality.fastq \
  --output-prefix test_filtered \
  --min-read-quality 25 \
  --platform pacbio
```

### Test reference grouping:
```bash
./target/release/ampliclust cluster \
  --guide test_data/references.fasta \
  --input test_data/reads.fastq \
  --output-prefix test_grouped \
  --platform pacbio
```

## Expected Results

With default parameters, you should see:
- **Placement rate**: >95% (most reads should place)
- **Avg confidence**: >0.9 (high confidence placements)
- **Reads per locus**:
  - Locus_A: ~800 reads (80%)
  - Locus_B: ~200 reads (20%)
  - Locus_C: 0 reads (no reads from this locus)

## Data Characteristics

- All sequences are synthetic/random
- References are 2.5-3.5 kb in length
- Reads match reference length (full amplicons)
- Mutation rate: ~0.1% from references
- Quality scores: Phred+33 encoding

Generated on: darwin
Random seed: 42 (reproducible)
