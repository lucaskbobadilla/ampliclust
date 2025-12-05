# AmpliClust Examples and Test Cases

## Example Workflows

### Example 1: HLA Typing with PacBio HiFi

```bash
# Download HLA reference alleles
wget https://github.com/ANHIG/IMGTHLA/raw/Latest/fasta/A_gen.fasta

# Run clustering
ampliclust cluster \
  --guide A_gen.fasta \
  --input patient_sample.fastq.gz \
  --output-prefix patient_hla_a \
  --platform pacbio \
  --min-cluster-frequency 0.1 \
  --min-cluster-reads 10 \
  --threads 8

# Expected output:
# - patient_hla_a_passed_clusters.fasta (2 clusters for diploid sample)
# - patient_hla_a_failed_clusters.fasta
# - patient_hla_a_read_info.txt
# - patient_hla_a_summary.json
```

### Example 2: Viral Quasispecies with ONT

```bash
# Viral amplicon with high diversity
ampliclust cluster \
  --guide hiv_env.fasta \
  --input ont_viral_reads.fastq.gz \
  --output-prefix hiv_quasispecies \
  --platform ont \
  --min-read-quality 7 \
  --min-cluster-frequency 0.01 \
  --min-cluster-reads 3 \
  --max-chimera-score 0.5 \
  --threads 16

# Will detect multiple viral variants in a single sample
```

### Example 3: 16S Amplicon Sequencing (De Novo)

```bash
# Microbial community profiling without references
ampliclust cluster \
  --input 16s_amplicons.fastq.gz \
  --output-prefix microbiome \
  --mode denovo \
  --platform illumina \
  --min-cluster-frequency 0.005 \
  --iterations 20 \
  --threads 8

# Generates OTUs/ASVs de novo
```

### Example 4: Pooled Samples with BAM Input

```bash
# Pre-aligned BAM from multiple amplicons
ampliclust cluster \
  --guide amplicon_panel.fasta \
  --input pooled_samples.bam \
  --output-prefix pooled_analysis \
  --from-bam \
  --platform ont \
  --max-reads-per-guide 500 \
  --output-bam \
  --threads 12

# Also generates tagged BAM for IGV visualization
```

## Test Data Generation

### Create Synthetic Test Data

```bash
# Python script to generate test FASTQ
python3 << 'EOF'
import random
import gzip

def generate_read(seq, error_rate=0.01, read_id="read"):
    """Generate a read with random errors"""
    bases = list(seq)
    for i in range(len(bases)):
        if random.random() < error_rate:
            bases[i] = random.choice(['A', 'C', 'G', 'T'])
    return ''.join(bases)

def write_fastq(reads, filename):
    """Write reads to FASTQ.GZ"""
    with gzip.open(filename, 'wt') as f:
        for i, (read_id, seq) in enumerate(reads):
            qual = 'I' * len(seq)  # Quality score ~40
            f.write(f"@{read_id}\n{seq}\n+\n{qual}\n")

# Two alleles for diploid sample
allele1 = "ACGTACGTACGT" * 100  # 1200 bp
allele2 = "ACGTACGTCCGT" * 100  # Same but with SNP

# Generate 100 reads per allele
reads = []
for i in range(100):
    reads.append((f"read_a1_{i}", generate_read(allele1, 0.001)))
    reads.append((f"read_a2_{i}", generate_read(allele2, 0.001)))

write_fastq(reads, "test_diploid.fastq.gz")
print("Generated test_diploid.fastq.gz with 200 reads")
EOF
```

### Create Reference FASTA

```bash
cat > test_reference.fasta << 'EOF'
>Allele1|TestLocus
ACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGT
>Allele2|TestLocus
ACGTACGTCCGTACGTACGTACGTACGTACGTACGTACGTACGT
>OffTarget|OffTarget
GGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGG
EOF
```

## Expected Output Examples

### Passed Clusters FASTA

```
>cluster_0_guide-Allele1_reads-98 freq:0.490 diversity:0.012 quality:42.3 chimera:-1.000 length:1200
ACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGT...

>cluster_1_guide-Allele2_reads-102 freq:0.510 diversity:0.015 quality:41.8 chimera:-1.000 length:1200
ACGTACGTCCGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGT...
```

### Read Info TSV

```
read_id         cluster_id  guide_name  alignment_score  strand  length  quality
read_a1_0       0           Allele1     0.985            +       1200    42.5
read_a1_1       0           Allele1     0.982            +       1198    41.9
read_a2_0       1           Allele2     0.987            +       1201    42.1
read_a2_1       1           Allele2     0.984            +       1200    41.7
```

### Summary JSON

```json
{
  "total_reads": 200,
  "clustered_reads": 200,
  "unclustered_reads": 0,
  "total_clusters": 2,
  "passed_clusters": 2,
  "failed_clusters": 0,
  "platform": "PacBio",
  "mode": "reference-guided",
  "processing_time_seconds": 12.5,
  "clusters": [
    {
      "id": 0,
      "guide_name": "Allele1",
      "read_count": 98,
      "frequency": 0.49,
      "diversity": 0.012,
      "avg_quality": 42.3,
      "chimera_score": null,
      "length": 1200,
      "status": "passed"
    },
    {
      "id": 1,
      "guide_name": "Allele2",
      "read_count": 102,
      "frequency": 0.51,
      "diversity": 0.015,
      "avg_quality": 41.8,
      "chimera_score": null,
      "length": 1200,
      "status": "passed"
    }
  ]
}
```

## Benchmarking

### Performance Test

```bash
# Generate larger test dataset
for i in {1..10}; do
  cat test_diploid.fastq.gz >> large_test.fastq.gz
done

# Benchmark
time ampliclust cluster \
  --guide test_reference.fasta \
  --input large_test.fastq.gz \
  --output-prefix benchmark \
  --threads 1

# Compare with multithreading
time ampliclust cluster \
  --guide test_reference.fasta \
  --input large_test.fastq.gz \
  --output-prefix benchmark_parallel \
  --threads 8
```

### Memory Profiling

```bash
# Using /usr/bin/time on Linux
/usr/bin/time -v ampliclust cluster \
  --guide test_reference.fasta \
  --input large_test.fastq.gz \
  --output-prefix memtest \
  --threads 4

# Or use valgrind massif
valgrind --tool=massif ampliclust cluster ...
ms_print massif.out.* | less
```

## Troubleshooting Examples

### Problem: No clusters found

```bash
# Check read placement
ampliclust cluster \
  --guide references.fasta \
  --input reads.fastq.gz \
  --output-prefix debug \
  --log-level DEBUG

# Look for placement issues in log
grep "placement" debug.log
```

### Problem: Too many low-frequency clusters

```bash
# Increase filtering thresholds
ampliclust cluster \
  --guide references.fasta \
  --input reads.fastq.gz \
  --output-prefix filtered \
  --min-cluster-frequency 0.10 \
  --min-cluster-reads 20
```

### Problem: Chimeric sequences

```bash
# Tighten chimera detection
ampliclust cluster \
  --guide references.fasta \
  --input reads.fastq.gz \
  --output-prefix chimera_filtered \
  --max-chimera-score 0.3

# Check failed clusters
grep "chimera" chimera_filtered_failed_clusters.fasta
```

## Integration with Downstream Tools

### Convert to OTU Table

```bash
# Extract cluster abundances
cat passed_clusters.fasta | grep "^>" | \
  awk '{print $1, $4}' | \
  sed 's/>//' | sed 's/freq://' > otu_table.txt
```

### Align Consensus to Database

```bash
# BLAST consensus sequences
makeblastdb -in database.fasta -dbtype nucl
blastn -query passed_clusters.fasta \
  -db database.fasta \
  -outfmt 6 \
  -out cluster_identifications.txt
```

### Phylogenetic Analysis

```bash
# Align clusters
mafft passed_clusters.fasta > aligned_clusters.fasta

# Build tree
iqtree -s aligned_clusters.fasta -m GTR+G -bb 1000
```

## Continuous Integration Tests

### Basic Functionality Test

```yaml
# .github/workflows/test.yml
name: Test AmpliClust

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Build
        run: cargo build --release
      
      - name: Run unit tests
        run: cargo test
      
      - name: Generate test data
        run: python3 scripts/generate_test_data.py
      
      - name: Test clustering
        run: |
          ./target/release/ampliclust cluster \
            --guide tests/data/reference.fasta \
            --input tests/data/reads.fastq.gz \
            --output-prefix ci_test \
            --threads 2
      
      - name: Verify output
        run: |
          test -f ci_test_passed_clusters.fasta
          test -f ci_test_read_info.txt
          test -f ci_test_summary.json
```

## Real-World Dataset Tests

### PacBio HLA Dataset

```bash
# Download public HLA dataset
wget https://downloads.pacbcloud.com/public/dataset/pbAmpliconAnalysis_HLA/m64012_200712_164638.fastq.gz
wget https://downloads.pacbcloud.com/public/dataset/pbAmpliconAnalysis_HLA/hla_references.fasta

# Run AmpliClust
ampliclust cluster \
  --guide hla_references.fasta \
  --input m64012_200712_164638.fastq.gz \
  --output-prefix hla_validation \
  --platform pacbio \
  --threads 8

# Compare with pbaa results
diff <(grep "^>" hla_validation_passed_clusters.fasta | sort) \
     <(grep "^>" pbaa_output.fasta | sort)
```

## Performance Targets

| Dataset Size | Platform | Expected Time | Expected Memory |
|-------------|----------|---------------|-----------------|
| 1K reads | PacBio | < 10s | < 500 MB |
| 10K reads | PacBio | < 2 min | < 2 GB |
| 100K reads | ONT | < 20 min | < 8 GB |
| 1M reads | Illumina | < 2 hrs | < 16 GB |

## Validation Metrics

Expected accuracy on test datasets:

- **Cluster Purity**: > 95% (reads in cluster are from same source)
- **Cluster Completeness**: > 90% (all reads from source in one cluster)
- **Consensus Accuracy**: > 99.9% identity to true sequence
- **Chimera Detection**: > 95% sensitivity, < 5% false positive rate
