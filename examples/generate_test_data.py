#!/usr/bin/env python3
"""
Generate synthetic test data for AmplicLust Phase 1 & 2 testing.

This script creates:
1. Reference sequences (FASTA)
2. Simulated reads (FASTQ) with variants
3. Grouped references for testing grouping functionality
"""

import random
import gzip
import sys
from pathlib import Path

# DNA alphabet
DNA = ['A', 'C', 'G', 'T']

def random_sequence(length):
    """Generate random DNA sequence."""
    return ''.join(random.choices(DNA, k=length))

def mutate_sequence(seq, mutation_rate=0.01):
    """Introduce random mutations into sequence."""
    seq_list = list(seq)
    for i in range(len(seq_list)):
        if random.random() < mutation_rate:
            # Mutation type
            mut_type = random.choice(['snv', 'ins', 'del'])
            if mut_type == 'snv':
                seq_list[i] = random.choice([b for b in DNA if b != seq_list[i]])
            elif mut_type == 'ins' and i < len(seq_list) - 1:
                seq_list.insert(i+1, random.choice(DNA))
            elif mut_type == 'del':
                seq_list[i] = ''
    return ''.join(seq_list)

def generate_quality_string(length, avg_qual=30):
    """Generate Phred quality string."""
    qualities = []
    for _ in range(length):
        qual = int(random.gauss(avg_qual, 5))
        qual = max(0, min(60, qual))  # Clamp to 0-60
        qualities.append(chr(qual + 33))  # Phred+33 encoding
    return ''.join(qualities)

def write_fasta(filename, sequences):
    """Write sequences to FASTA file."""
    with open(filename, 'w') as f:
        for name, seq in sequences:
            f.write(f">{name}\n")
            # Write in 80-character lines
            for i in range(0, len(seq), 80):
                f.write(seq[i:i+80] + '\n')
    print(f"✓ Created {filename} with {len(sequences)} sequences")

def write_fastq(filename, reads, compress=False):
    """Write reads to FASTQ file (optionally gzipped)."""
    opener = gzip.open if compress else open
    mode = 'wt' if compress else 'w'
    
    with opener(filename, mode) as f:
        for read_id, seq, qual in reads:
            f.write(f"@{read_id}\n")
            f.write(f"{seq}\n")
            f.write("+\n")
            f.write(f"{qual}\n")
    
    print(f"✓ Created {filename} with {len(reads)} reads")

def main():
    """Generate test data."""
    print("=" * 60)
    print("AmplicLust Test Data Generator")
    print("=" * 60)
    
    output_dir = Path("test_data")
    output_dir.mkdir(exist_ok=True)
    
    random.seed(42)  # Reproducible
    
    # 1. Generate reference sequences
    print("\n[1/5] Generating reference sequences...")
    
    references = []
    
    # Locus A - 3 alleles
    base_a = random_sequence(3000)
    references.append(("Allele_A_01|Locus_A", base_a))
    references.append(("Allele_A_02|Locus_A", mutate_sequence(base_a, 0.005)))
    references.append(("Allele_A_03|Locus_A", mutate_sequence(base_a, 0.01)))
    
    # Locus B - 2 alleles
    base_b = random_sequence(2500)
    references.append(("Allele_B_01|Locus_B", base_b))
    references.append(("Allele_B_02|Locus_B", mutate_sequence(base_b, 0.008)))
    
    # Locus C - 2 alleles  
    base_c = random_sequence(3500)
    references.append(("Allele_C_01|Locus_C", base_c))
    references.append(("Allele_C_02|Locus_C", mutate_sequence(base_c, 0.006)))
    
    write_fasta(output_dir / "references.fasta", references)
    
    # 2. Generate simple references (no grouping)
    print("\n[2/5] Generating simple references...")
    simple_refs = [
        ("Reference_1", random_sequence(3000)),
        ("Reference_2", random_sequence(2800)),
        ("Reference_3", random_sequence(3200)),
    ]
    write_fasta(output_dir / "simple_refs.fasta", simple_refs)
    
    # 3. Generate reads from grouped references
    print("\n[3/5] Generating reads from references...")
    
    reads = []
    read_count = 0
    
    # Distribution: 40% A_01, 30% A_02, 10% A_03, 15% B_01, 5% B_02
    distributions = [
        (references[0], 400),  # A_01
        (references[1], 300),  # A_02
        (references[2], 100),  # A_03
        (references[3], 150),  # B_01
        (references[4], 50),   # B_02
    ]
    
    for (ref_name, ref_seq), count in distributions:
        for i in range(count):
            read_count += 1
            # Simulate read with variants
            read_seq = mutate_sequence(ref_seq, mutation_rate=0.001)
            qual = generate_quality_string(len(read_seq), avg_qual=35)
            read_id = f"read_{read_count:05d}_from_{ref_name.split('|')[0]}"
            reads.append((read_id, read_seq, qual))
    
    # Shuffle reads
    random.shuffle(reads)
    
    write_fastq(output_dir / "reads.fastq", reads)
    
    # 4. Generate gzipped version
    print("\n[4/5] Generating gzipped FASTQ...")
    write_fastq(output_dir / "reads.fastq.gz", reads, compress=True)
    
    # 5. Generate reads with varying quality
    print("\n[5/5] Generating reads with quality variation...")
    
    varied_reads = []
    for i in range(500):
        # 80% high quality, 20% low quality
        if random.random() < 0.8:
            avg_qual = random.randint(30, 45)  # High quality
        else:
            avg_qual = random.randint(5, 20)   # Low quality
        
        ref = random.choice(references)
        seq = mutate_sequence(ref[1], mutation_rate=0.002)
        qual = generate_quality_string(len(seq), avg_qual=avg_qual)
        varied_reads.append((f"varied_read_{i:05d}", seq, qual))
    
    write_fastq(output_dir / "varied_quality.fastq", varied_reads)
    
    # Create a README
    print("\n[6/6] Creating README...")
    readme_content = f"""# Test Data for AmplicLust

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
./target/release/ampliclust cluster \\
  --guide test_data/references.fasta \\
  --input test_data/reads.fastq \\
  --output-prefix test_basic \\
  --platform pacbio \\
  --threads 4
```

### Test gzipped input:
```bash
./target/release/ampliclust cluster \\
  --guide test_data/references.fasta \\
  --input test_data/reads.fastq.gz \\
  --output-prefix test_gzipped \\
  --platform pacbio
```

### Test quality filtering:
```bash
./target/release/ampliclust cluster \\
  --guide test_data/references.fasta \\
  --input test_data/varied_quality.fastq \\
  --output-prefix test_filtered \\
  --min-read-quality 25 \\
  --platform pacbio
```

### Test reference grouping:
```bash
./target/release/ampliclust cluster \\
  --guide test_data/references.fasta \\
  --input test_data/reads.fastq \\
  --output-prefix test_grouped \\
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

Generated on: {sys.platform}
Random seed: 42 (reproducible)
"""
    
    with open(output_dir / "README.md", 'w') as f:
        f.write(readme_content)
    
    print(f"✓ Created {output_dir / 'README.md'}")
    
    print("\n" + "=" * 60)
    print("✨ Test data generation complete!")
    print("=" * 60)
    print(f"\nTest data location: {output_dir.absolute()}")
    print("\nNext steps:")
    print("1. Review TESTING_GUIDE.md for test procedures")
    print("2. Run test commands from test_data/README.md")
    print("3. Validate output files and statistics")
    print("\nHappy testing! 🧪")

if __name__ == "__main__":
    main()
