<h1 align="center"><i>AmplicLust</i></h1>
<p align="center">Universal Amplicon Clustering Tool</p>

***

**AmplicLust** is a universal amplicon clustering tool designed to work with multiple sequencing platforms: **PacBio HiFi**, **Oxford Nanopore (ONT)**, and **Illumina**. It separates complex mixtures of amplicon targets from genomic samples and generates high-quality consensus sequences.

AmplicLust supports both **reference-guided** and **de novo** clustering approaches, making it flexible for various experimental designs. The tool accepts both FASTQ and BAM file inputs and provides comprehensive cluster frequency metrics.

## Key Features

- **Multi-platform support**: PacBio HiFi, ONT, and Illumina data
- **Flexible input**: FASTQ and BAM files
- **Dual clustering modes**: Reference-guided and de novo
- **Fast alignment**: K-mer indexing with minimap2-style chaining
- **Accurate refinement**: Edit distance calculations for precision
- **Parallel processing**: Multi-threaded for performance
- **Comprehensive metrics**: Cluster frequencies, quality scores, and statistics
- **IGV visualization**: BAM painting with cluster tags for visualization

## Use Cases

Typical use cases involve multi-allelic samples where the sample-specific ploidy or copy number is unknown. AmplicLust can effectively separate alleles with one to many variants, including SNVs and large indels contained within the target region. The tool has been optimized for datasets with moderate to high cluster counts.

## Workflow

AmplicLust follows a streamlined workflow:

1. **Input Detection**: Automatically detects FASTQ/BAM format and platform
2. **Quality Filtering**: Filters reads by quality and length thresholds
3. **Alignment**: K-mer indexing and minimap2-style placement
4. **Clustering**: Graph-based or k-means clustering
5. **Consensus**: High-quality consensus sequence generation
6. **Output**: FASTA sequences with comprehensive statistics

## Installation

### From Source (Rust)

```bash
# Clone the repository
git clone https://github.com/yourusername/ampliclust.git
cd ampliclust

# Build with cargo
cargo build --release

# Binary will be in target/release/ampliclust
./target/release/ampliclust --version
```

### Prerequisites

- Rust 1.83+ (for compilation)
- Samtools 1.9+ (for indexing input files)

## Usage

AmplicLust has three main commands: `cluster`, `bampaint`, and `stats`.

```bash
ampliclust - Universal Amplicon Clustering Tool

Usage:
  ampliclust <COMMAND>

Commands:
  cluster    Run clustering on amplicon reads
  bampaint   Paint BAM files with cluster assignments
  stats      Generate statistics from clustering results
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Main clustering tool

This tool runs the alignment, clustering, and consensus algorithms.

```bash
ampliclust cluster - Run clustering on amplicon reads

Usage:
  ampliclust cluster [OPTIONS] <REFERENCES> <READS> <OUTPUT>

Arguments:
  <REFERENCES>  Reference sequences in FASTA format
  <READS>       Input reads (FASTQ or BAM format)
  <OUTPUT>      Output file prefix

Options:
  -p, --platform <PLATFORM>
          Sequencing platform [possible values: pacbio, ont, illumina, auto]
  
  -m, --mode <MODE>
          Clustering mode [default: reference] [possible values: reference, denovo]
  
  -k, --kmer-size <KMER_SIZE>
          K-mer size for indexing [default: 15]
  
  -t, --threads <THREADS>
          Number of threads [default: 4]
  
      --min-quality <MIN_QUALITY>
          Minimum average read quality [default: 20]
  
      --min-length <MIN_LENGTH>
          Minimum read length [default: 100]
  
      --max-length <MAX_LENGTH>
          Maximum read length [default: 50000]
  
      --min-cluster-size <MIN_CLUSTER_SIZE>
          Minimum cluster size [default: 5]
  
      --min-cluster-frequency <MIN_CLUSTER_FREQUENCY>
          Minimum cluster frequency [default: 0.01]
  
  -h, --help
          Print help
```

### BAM Painting Tool

Add IGV visualization tags to BAM files based on cluster assignments.

```bash
ampliclust bampaint - Paint BAM files with cluster assignments

Usage:
  ampliclust bampaint [OPTIONS] <CLUSTERS> <INPUT_BAM> <OUTPUT_BAM>

Arguments:
  <CLUSTERS>    Cluster information file from ampliclust cluster
  <INPUT_BAM>   Input BAM file
  <OUTPUT_BAM>  Output BAM file with cluster tags

Options:
  -t, --threads <THREADS>  Number of threads [default: 4]
  -h, --help              Print help
```

### Statistics Tool

Generate comprehensive statistics from clustering results.

```bash
ampliclust stats - Generate statistics from clustering results

Usage:
  ampliclust stats [OPTIONS] <CLUSTERS>

Arguments:
  <CLUSTERS>  Cluster information file

Options:
  -o, --output <OUTPUT>  Output statistics file (JSON format)
  -h, --help            Print help
```

## Quick Start

### Reference-guided clustering (PacBio HiFi)

```bash
ampliclust cluster \
  --platform pacbio \
  --mode reference \
  --threads 8 \
  references.fasta \
  hifi_reads.fastq \
  output_prefix
```

### De novo clustering (ONT)

```bash
ampliclust cluster \
  --platform ont \
  --mode denovo \
  --min-cluster-size 10 \
  --threads 8 \
  ont_reads.fastq \
  output_prefix
```

### Paint BAM file with clusters

```bash
ampliclust bampaint \
  output_prefix_clusters.txt \
  aligned_reads.bam \
  painted_reads.bam
```

## Input

AmplicLust accepts the following inputs:

### Read Files
- **FASTQ**: De-multiplexed reads (can be gzip compressed)
- **BAM/SAM**: Aligned or unaligned reads
- **FOFN**: File of file names for batch processing

### Reference Files (for reference-guided mode)
- **FASTA**: Reference/guide sequences
- Guide sequences should contain the amplified region, but not much more
- Can use FOFN format for multiple reference files

AmplicLust automatically detects file formats and handles compressed files. For FOFN format, provide one file path per line.

## Customizing Reference Sequences

Reference sequence choice affects read grouping and placement in reference-guided mode. Follow these guidelines:

### Reference Selection
- Choose sufficiently divergent references to distinguish between loci
- Too many similar sequences reduce informative k-mers and increase un-placed reads
- Too few references can cause cluster dropout

### Grouping References

Group related sequences by locus using the pipe delimiter (`|`) in FASTA headers:

```fasta
>Allele_1|HLA-A
ACGTACGT...
>Allele_2|HLA-A
ACGTACGT...
>Allele_1|HLA-B
TGCATGCA...
```

Reads placed to any allele in a group (e.g., HLA-A) will be clustered together at the locus level. This is useful for:
- Multi-allelic loci (HLA typing)
- Gene families
- Isoform analysis

For more details, see `guide_reference.md`


## Output

_pbaa_ will generate three output files.

1. {prefix}_passed_cluster_sequences.fasta
2. {prefix}_failed_cluster_sequences.fasta
3. {prefix}_read_info.txt


### consensus sequence output

The headers entries in the consensus sequence output contain statistics about the clusters. These statistics are used for filtering (pass/fail criterion).

example of a passing sequence:

```
>sample-bc1001--bc1001_guide-HLA-A_cluster-1_ReadCount-22 uchime_score:-1 uchime_left_parent:N/A uchime_right_parent:N/A cluster_freq:0.44 diversity:0.188552 avg_quality:53.5878 duplicate_parent:N/A seq_length:3152 filters:none
```

example of a failing sequence:

```
>sample-bc1011--bc1011_guide-HLA-DRB1_cluster-3_ReadCount-6 uchime_score:0.03125 uchime_left_parent:bc1011--bc1011_HLA-DRB1_0 uchime_right_parent:bc1011--bc1011_HLA-DRB1_1 cluster_freq:0.12 diversity:0 avg_quality:42.2187 duplicate_parent:N/A seq_length:3706 filters:fail-low-frequency
```

The fields in the header are:

1. **uchime_score** The UCHIME score flags chimeric consensus sequences. The higher the score the more likely the sequence is chimeric. For more details see: Edgar, Robert C., et al. “UCHIME improves sensitivity and speed of chimera detection.” Bioinformatics 27.16 (2011): 2194-2200.

2. **uchime_left_parent/uchime_right_parent** The parent sequences of a chimeric sequence.

3. **cluster_freq** A measures of the clusters’ frequencies. The frequency is calculated by reads counts within groupings.

4. **diversity** A measure of the variability of variants within a cluster. Clusters with homogenous reads will have low diversity. A negative value indicates this metric was not calculated.

5. **avg_quality** The average PHRED quality of the reads within the cluster.

6. **seq_length** The sequence length of the cluster.

6. **filters** This is a space separated field enumerating the possible reasons a cluster was placed in the fail category.

### Read Information Output File
One row per read, columns as follows:
1. SeqName
2. GuideName
3. strand
4. SecondBestGuideName
5. Score
6. FirstHighest/SecondHighest/UniqueHitSum
7. Sample, input fastq
8. Sequence length
9. Average read quality
10. Cluster ID
11. Cluster Size

**_Example:_**
```
m64012_200712_164638/72090819/ccs HLA-DRB5 - HLA00622_DQB1_02-01-01_7480_bp|HLA-DQB1 0.714286 f:5/s:2/sum:8 /pbi/dept/appslab/projects/old/2020/jh_hla/2020-07-13_HGgendx/fastq_sqII/demultiplex.bc1099--bc1099.fastq 3125 58.6149 1 1
```

## Platform-Specific Recommendations

### PacBio HiFi
- Use `--platform pacbio`
- Default k-mer size (15) works well
- Minimum quality: 20 (Q20)
- Best for: High accuracy, long amplicons

### Oxford Nanopore (ONT)
- Use `--platform ont`
- Consider smaller k-mer size (13) for higher error rates
- Minimum quality: 10-15
- Best for: Long amplicons, rapid turnaround

### Illumina
- Use `--platform illumina`
- Smaller k-mer size (11-13) for short reads
- Minimum quality: 20-30
- Best for: High throughput, short amplicons

## Best Practices

### 1. Start with Defaults
Default parameters are optimized for most use cases. Only adjust parameters when needed for your specific dataset.

### 2. Choose Appropriate K-mer Size
- **PacBio HiFi**: k=15-19 (high accuracy)
- **ONT**: k=13-15 (moderate error rate)
- **Illumina**: k=11-13 (short reads)
- K-mer size must be odd and ≤31

### 3. Quality Filtering
- Set minimum quality based on your platform
- Filter out very short or very long reads
- Check quality distributions before clustering

### 4. Reference Selection (Reference-Guided Mode)
- Use divergent references to distinguish loci
- Group related alleles using `|` delimiter
- Include all expected variants
- Avoid whole chromosomes or very long sequences

### 5. Clustering Parameters
- Increase `--min-cluster-size` for high-coverage datasets
- Adjust `--min-cluster-frequency` to filter rare clusters
- Use more threads (`--threads`) for large datasets

### 6. Validation
- Inspect `_failed_clusters.fasta` for filtered sequences
- Check statistics in JSON output
- Visualize BAM painting results in IGV

## Advanced Options

### K-mer Size Optimization
The k-mer size affects both sensitivity and specificity:
- Larger k: More specific, fewer spurious matches
- Smaller k: More sensitive, works with higher error rates
- Must be odd number (for canonical k-mers)
- Maximum: 31 (due to 64-bit encoding)

### Threading and Performance
- Use `--threads` to match your CPU cores
- Parallel processing for alignment and clustering
- Memory usage scales with number of reads and references

### Filtering Thresholds
Fine-tune filtering for your application:
```bash
--min-quality 25 \           # Stricter quality filter
--min-cluster-size 10 \      # Larger minimum cluster size
--min-cluster-frequency 0.02 # 2% minimum frequency
```

## FAQ

### Q: Which clustering mode should I use?

**Reference-guided** when you have:
- Known reference sequences for your amplicons
- Multi-allelic loci (e.g., HLA typing)
- Need to assign reads to specific genes/alleles

**De novo** when you:
- Don't have reference sequences
- Are discovering novel variants
- Have unknown amplicon diversity

### Q: How do I choose the right k-mer size?

Match k-mer size to your data characteristics:
- **High accuracy (PacBio HiFi)**: k=15-19
- **Moderate accuracy (ONT)**: k=13-15
- **Short reads (Illumina)**: k=11-13
- **Rule of thumb**: Higher accuracy → larger k-mer

### Q: Why are some reads not clustering?

Common reasons:
1. **Low quality**: Check `--min-quality` threshold
2. **Length filters**: Adjust `--min-length` and `--max-length`
3. **Low coverage**: Clusters below `--min-cluster-size` are filtered
4. **Off-target**: Reads don't match any reference
5. **Chimeric**: Potential PCR artifacts

Check `_failed_clusters.fasta` and statistics output for details.

### Q: How much coverage do I need?

Recommended minimum coverage per allele:
- **PacBio HiFi**: 20-30x
- **ONT**: 30-50x (higher error rate)
- **Illumina**: 50-100x (shorter reads)

Higher coverage improves consensus quality and cluster detection.

### Q: Can I process multiple samples at once?

Yes! Use FOFN (File Of File Names) format:
```bash
# create file list
ls /path/to/samples/*.fastq > samples.fofn

# run clustering
ampliclust cluster references.fasta samples.fofn output_prefix
```

### Q: How do I visualize results?

Use the `bampaint` command to add cluster tags to BAM files:
```bash
ampliclust bampaint clusters.txt input.bam painted.bam
```

Then open `painted.bam` in IGV. Reads will be colored and grouped by cluster.

### Q: Performance tips?

1. Use `--threads` to match your CPU cores
2. Filter low-quality reads upfront with `--min-quality`
3. Use appropriate length filters
4. For very large datasets, consider downsampling first

## Troubleshooting

### Too many clusters (false positives)
- Increase `--min-cluster-size`
- Increase `--min-cluster-frequency`
- Check for chimeric reads
- Verify reference sequences are appropriate

### Missing expected clusters (false negatives)
- Check `_failed_clusters.fasta`
- Decrease `--min-cluster-size`
- Decrease `--min-cluster-frequency`
- Verify sufficient coverage
- Check reference sequences include expected variants

### Slow performance
- Reduce `--threads` if memory-limited
- Use length filters to remove outliers
- Consider smaller k-mer size
- Process samples separately

### Memory issues
- Reduce number of threads
- Process smaller batches
- Use reference-guided mode (more efficient than de novo)

## Support

For bug reports, feature requests, or questions:
- GitHub Issues: [https://github.com/yourusername/ampliclust/issues](https://github.com/lucaskbobadilla/ampliclust/issues)
- Documentation: See `AMPLICLUST_README.md` and `IMPLEMENTATION_GUIDE.md`

## License

AmplicLust is dual-licensed under MIT OR Apache-2.0.

## Citation

If you use AmplicLust in your research, please cite:

```
AmplicLust: Universal Amplicon Clustering Tool
```

## Acknowledgments

AmplicLust was inspired by:
- **pbaa** - PacBio Amplicon Analysis tool
- **minimap2** - Fast sequence alignment
- **UCHIME** - Chimera detection algorithm

## Related Tools

- **pbaa**: Original PacBio-specific amplicon tool
- **minimap2**: General-purpose sequence aligner
- **DADA2**: Amplicon sequence variant detection
- **mothur**: Microbial ecology toolkit

## Changelog

### Version 0.1.0 (Current Development)

**Phase 1 - Core I/O (Complete)**
- ✅ FASTQ reader/writer with gzip support
- ✅ FASTA reader/writer with reference grouping
- ✅ BAM reader/writer with cluster tagging
- ✅ Format detection and FOFN support
- ✅ 21 unit tests

**Phase 2 - Alignment (Complete)**
- ✅ K-mer indexing with 2-bit encoding
- ✅ Read placement with confidence scoring
- ✅ Minimap2-style alignment with minimizers
- ✅ Edit distance calculations
- ✅ Platform-specific presets
- ✅ 30 additional unit tests

**Phase 3 - Clustering (In Progress)**
- Graph-based clustering
- K-means clustering
- De novo mode

**Phase 4-12 - Upcoming**
- Consensus generation
- Variant calling
- Quality metrics
- Chimera detection
- CLI integration
- Performance optimization

See `DEVELOPMENT_CHECKLIST.md` for detailed progress.
