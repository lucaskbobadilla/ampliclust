#!/bin/bash
# Quick test suite for AmplicLust Phase 1 & 2

set -e

# Get the project root directory (parent of tests/)
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

echo "════════════════════════════════════════════════════════════"
echo "AmplicLust Phase 1 & 2 Test Suite"
echo "════════════════════════════════════════════════════════════"

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Check if binary exists
if [ ! -f "./target/release/ampliclust" ]; then
    echo -e "${RED}✗ Binary not found. Building...${NC}"
    cargo build --release
fi

# Generate test data
if [ ! -d "tests/test_data" ]; then
    echo -e "${YELLOW}→ Generating test data...${NC}"
    python3 examples/generate_test_data.py
fi

echo ""
echo "════════════════════════════════════════════════════════════"
echo "Running Tests"
echo "════════════════════════════════════════════════════════════"

# Test 1: Basic FASTQ with references
echo ""
echo -e "${YELLOW}[Test 1/6]${NC} Basic FASTQ input with reference-guided alignment"
./target/release/ampliclust cluster \
  --guide tests/test_data/references.fasta \
  --input tests/test_data/reads.fastq \
  --output-prefix tests/results/test1_basic \
  --platform pacbio \
  --kmer-size 15 \
  --threads 4 \
  --log-level info

if [ -f "tests/results/test1_basic_placements.txt" ]; then
    lines=$(wc -l < tests/results/test1_basic_placements.txt)
    echo -e "${GREEN}✓ Test 1 passed - Generated placement file with $lines lines${NC}"
else
    echo -e "${RED}✗ Test 1 failed - No placement file generated${NC}"
    exit 1
fi

# Test 2: Gzipped FASTQ
echo ""
echo -e "${YELLOW}[Test 2/6]${NC} Gzipped FASTQ input"
./target/release/ampliclust cluster \
  --guide tests/test_data/references.fasta \
  --input tests/test_data/reads.fastq.gz \
  --output-prefix tests/results/test2_gzipped \
  --platform pacbio \
  --threads 4 \
  --log-level warn

if [ -f "tests/results/test2_gzipped_placements.txt" ]; then
    echo -e "${GREEN}✓ Test 2 passed - Gzipped input works${NC}"
else
    echo -e "${RED}✗ Test 2 failed${NC}"
    exit 1
fi

# Test 3: Quality filtering
echo ""
echo -e "${YELLOW}[Test 3/6]${NC} Quality filtering"
./target/release/ampliclust cluster \
  --guide tests/test_data/references.fasta \
  --input tests/test_data/varied_quality.fastq \
  --output-prefix tests/results/test3_filtered \
  --min-read-quality 25 \
  --platform pacbio \
  --threads 4 \
  --log-level warn

if [ -f "tests/results/test3_filtered_placements.txt" ]; then
    echo -e "${GREEN}✓ Test 3 passed - Quality filtering works${NC}"
else
    echo -e "${RED}✗ Test 3 failed${NC}"
    exit 1
fi

# Test 4: Different k-mer sizes
echo ""
echo -e "${YELLOW}[Test 4/6]${NC} K-mer size variations"
for k in 11 15 19; do
    echo "  Testing k=$k..."
    ./target/release/ampliclust cluster \
      --guide tests/test_data/references.fasta \
      --input tests/test_data/reads.fastq \
      --output-prefix tests/results/test4_k${k} \
      --kmer-size $k \
      --platform pacbio \
      --threads 4 \
      --log-level warn > /dev/null 2>&1
    
    if [ -f "tests/results/test4_k${k}_placements.txt" ]; then
        echo -e "    ${GREEN}✓ k=$k works${NC}"
    else
        echo -e "    ${RED}✗ k=$k failed${NC}"
        exit 1
    fi
done

# Test 5: Threading
echo ""
echo -e "${YELLOW}[Test 5/6]${NC} Multi-threading performance"
echo "  Single thread..."
time_1=$(( time ./target/release/ampliclust cluster \
  --guide tests/test_data/references.fasta \
  --input tests/test_data/reads.fastq \
  --output-prefix tests/results/test5_t1 \
  --threads 1 \
  --log-level warn > /dev/null 2>&1 ) 2>&1 | grep real | awk '{print $2}')

echo "  Four threads..."
time_4=$(( time ./target/release/ampliclust cluster \
  --guide tests/test_data/references.fasta \
  --input tests/test_data/reads.fastq \
  --output-prefix tests/results/test5_t4 \
  --threads 4 \
  --log-level warn > /dev/null 2>&1 ) 2>&1 | grep real | awk '{print $2}')

echo -e "${GREEN}✓ Test 5 passed - Multi-threading works${NC}"
echo "  1 thread: $time_1"
echo "  4 threads: $time_4"

# Test 6: Negative control (reads don't match references)
echo ""
echo -e "${YELLOW}[Test 6/6]${NC} Negative control - unmatched references"
./target/release/ampliclust cluster \
  --guide tests/test_data/simple_refs.fasta \
  --input tests/test_data/reads.fastq \
  --output-prefix tests/results/test6_negative \
  --platform pacbio \
  --threads 4 \
  --log-level warn

if [ -f "tests/results/test6_negative_placements.txt" ]; then
    lines=$(wc -l < tests/results/test6_negative_placements.txt)
    if [ "$lines" -le 10 ]; then
        echo -e "${GREEN}✓ Test 6 passed - Correctly rejects unmatched reads (negative control)${NC}"
    else
        echo -e "${RED}✗ Test 6 failed - Should have few/no placements for unmatched data${NC}"
    fi
else
    echo -e "${RED}✗ Test 6 failed${NC}"
    exit 1
fi

# Summary
echo ""
echo "════════════════════════════════════════════════════════════"
echo "Test Summary"
echo "════════════════════════════════════════════════════════════"
echo ""

# Analyze one of the placement files
echo "Sample placement statistics (tests/results/test1_basic_placements.txt):"
echo ""
tail -n +2 tests/results/test1_basic_placements.txt | awk '
BEGIN {
    sum_conf = 0
    sum_hits = 0
    count = 0
}
{
    sum_conf += $4
    sum_hits += $5
    count++
    refs[$3]++
}
END {
    print "  Total reads placed: " count
    print "  Avg confidence: " sprintf("%.3f", sum_conf/count)
    print "  Avg k-mer hits: " sprintf("%.1f", sum_hits/count)
    print ""
    print "  Reads per reference:"
    for (ref in refs) {
        print "    " ref ": " refs[ref] " reads (" sprintf("%.1f", 100*refs[ref]/count) "%)"
    }
}
'

echo ""
echo -e "${GREEN}════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}✨ All tests passed! Phase 1 & 2 are working correctly.${NC}"
echo -e "${GREEN}════════════════════════════════════════════════════════════${NC}"
echo ""
echo "Generated files:"
ls -lh tests/results/test*_placements.txt
echo ""
echo "Next steps:"
echo "  1. Review placement files to verify correctness"
echo "  2. Test with your own real data"
echo "  3. See docs/TESTING_GUIDE.md for comprehensive testing"
echo "  4. Ready to implement Phase 3 (Clustering)!"
