use clap::{Parser, Subcommand, ValueEnum};
use anyhow::Result;
use log::{info, error};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ampliclust")]
#[command(author, version, about = "Universal amplicon clustering for PacBio, ONT, and Illumina", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Number of threads (0 = auto-detect)
    #[arg(short = 'j', long, default_value = "0", global = true)]
    threads: usize,

    /// Log level
    #[arg(long, default_value = "info", global = true)]
    log_level: String,

    /// Log to file instead of stderr
    #[arg(long, global = true)]
    log_file: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run clustering analysis
    Cluster {
        /// Guide/reference sequences (FASTA format)
        #[arg(short = 'g', long)]
        guide: Option<PathBuf>,

        /// Input reads (FASTQ, FASTQ.GZ, or BAM)
        #[arg(short = 'i', long)]
        input: PathBuf,

        /// Output prefix for results
        #[arg(short = 'o', long)]
        output_prefix: String,

        /// Sequencing platform
        #[arg(short = 'p', long, default_value = "auto")]
        platform: PlatformArg,

        /// Clustering mode
        #[arg(short = 'm', long, default_value = "reference-guided")]
        mode: ClusteringMode,

        /// Input is BAM format
        #[arg(long)]
        from_bam: bool,

        // Quality filtering options
        /// Minimum read quality (Phred scale)
        #[arg(long, default_value = "10")]
        min_read_quality: f64,

        /// Maximum amplicon size (bp)
        #[arg(long, default_value = "15000")]
        max_amplicon_size: usize,

        // Alignment options
        /// K-mer size for read placement
        #[arg(long, default_value = "15")]
        kmer_size: usize,

        /// Maximum reads per guide/locus
        #[arg(long, default_value = "1000")]
        max_reads_per_guide: usize,

        /// Maximum alignments per read
        #[arg(long, default_value = "100")]
        max_alignments_per_read: usize,

        // Clustering options
        /// Number of clustering iterations
        #[arg(long, default_value = "10")]
        iterations: usize,

        /// Random seed for reproducibility
        #[arg(long, default_value = "42")]
        seed: u64,

        // Consensus options
        /// Consensus algorithm
        #[arg(long, default_value = "spoa")]
        consensus_algorithm: ConsensusAlgorithm,

        /// Maximum reads for consensus
        #[arg(long, default_value = "100")]
        max_consensus_reads: usize,

        // Filtering options
        /// Minimum cluster frequency
        #[arg(long, default_value = "0.05")]
        min_cluster_frequency: f64,

        /// Minimum reads per cluster
        #[arg(long, default_value = "5")]
        min_cluster_reads: usize,

        /// Maximum UCHIME chimera score
        #[arg(long, default_value = "1.0")]
        max_chimera_score: f64,

        /// Skip chimera detection
        #[arg(long)]
        skip_chimera: bool,

        /// Output BAM with cluster tags
        #[arg(long)]
        output_bam: bool,
    },

    /// Add cluster tags to existing BAM file
    BamPaint {
        /// Read info file from clustering
        #[arg(short = 'r', long)]
        read_info: PathBuf,

        /// Input BAM file
        #[arg(short = 'i', long)]
        input_bam: PathBuf,

        /// Output BAM file
        #[arg(short = 'o', long)]
        output_bam: PathBuf,
    },

    /// Generate statistics from clustering results
    Stats {
        /// Cluster FASTA file
        #[arg(short = 'c', long)]
        clusters: PathBuf,

        /// Read info file
        #[arg(short = 'r', long)]
        read_info: PathBuf,

        /// Output statistics file
        #[arg(short = 'o', long)]
        output: PathBuf,

        /// Output format
        #[arg(short = 'f', long, default_value = "json")]
        format: OutputFormat,
    },
}

#[derive(Debug, Clone, ValueEnum)]
enum PlatformArg {
    Auto,
    Pacbio,
    Ont,
    Illumina,
}

#[derive(Debug, Clone, ValueEnum)]
enum ClusteringMode {
    ReferenceGuided,
    Denovo,
    Hybrid,
}

#[derive(Debug, Clone, ValueEnum)]
enum ConsensusAlgorithm {
    Spoa,
    Poa,
    Simple,
}

#[derive(Debug, Clone, ValueEnum)]
enum OutputFormat {
    Json,
    Tsv,
    Html,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Setup logging
    setup_logging(&cli.log_level, cli.log_file.as_ref())?;

    // Set thread count
    let threads = if cli.threads == 0 {
        num_cpus::get()
    } else {
        cli.threads
    };
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build_global()?;

    info!("AmpliClust v{}", env!("CARGO_PKG_VERSION"));
    info!("Using {} threads", threads);

    match cli.command {
        Commands::Cluster {
            guide,
            input,
            output_prefix,
            platform,
            mode,
            from_bam,
            min_read_quality,
            max_amplicon_size,
            kmer_size,
            max_reads_per_guide,
            max_alignments_per_read,
            iterations,
            seed,
            consensus_algorithm,
            max_consensus_reads,
            min_cluster_frequency,
            min_cluster_reads,
            max_chimera_score,
            skip_chimera,
            output_bam,
        } => {
            info!("Running clustering analysis...");
            
            // Create configuration
            let config = ampliclust::Config {
                guide,
                input,
                output_prefix,
                platform: convert_platform(platform),
                mode: convert_mode(mode),
                from_bam,
                min_read_quality,
                max_amplicon_size,
                kmer_size,
                max_reads_per_guide,
                max_alignments_per_read,
                iterations,
                seed,
                consensus_algorithm: convert_consensus(consensus_algorithm),
                max_consensus_reads,
                min_cluster_frequency,
                min_cluster_reads,
                max_chimera_score,
                skip_chimera,
                output_bam,
            };

            // Run clustering pipeline
            run_clustering(config)?;
        }
        
        Commands::BamPaint {
            read_info,
            input_bam,
            output_bam,
        } => {
            info!("Adding cluster tags to BAM...");
            run_bam_paint(read_info, input_bam, output_bam)?;
        }
        
        Commands::Stats {
            clusters,
            read_info,
            output,
            format,
        } => {
            info!("Generating statistics...");
            run_stats(clusters, read_info, output, format)?;
        }
    }

    info!("Analysis complete!");
    Ok(())
}

fn setup_logging(level: &str, log_file: Option<&PathBuf>) -> Result<()> {
    use env_logger::Builder;
    use std::io::Write;
    
    let mut builder = Builder::new();
    builder.filter_level(level.parse()?);
    
    if let Some(path) = log_file {
        let file = std::fs::File::create(path)?;
        builder.target(env_logger::Target::Pipe(Box::new(file)));
    }
    
    builder.format(|buf, record| {
        writeln!(
            buf,
            "[{} {} {}] {}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            record.level(),
            record.target(),
            record.args()
        )
    });
    
    builder.try_init()?;
    Ok(())
}

fn convert_platform(platform: PlatformArg) -> ampliclust::Platform {
    match platform {
        PlatformArg::Auto => ampliclust::Platform::Unknown,
        PlatformArg::Pacbio => ampliclust::Platform::PacBio,
        PlatformArg::Ont => ampliclust::Platform::ONT,
        PlatformArg::Illumina => ampliclust::Platform::Illumina,
    }
}

fn convert_mode(mode: ClusteringMode) -> String {
    match mode {
        ClusteringMode::ReferenceGuided => "reference-guided".to_string(),
        ClusteringMode::Denovo => "denovo".to_string(),
        ClusteringMode::Hybrid => "hybrid".to_string(),
    }
}

fn convert_consensus(algo: ConsensusAlgorithm) -> String {
    match algo {
        ConsensusAlgorithm::Spoa => "spoa".to_string(),
        ConsensusAlgorithm::Poa => "poa".to_string(),
        ConsensusAlgorithm::Simple => "simple".to_string(),
    }
}

fn run_clustering(config: ampliclust::Config) -> Result<()> {
    use ampliclust::io::formats::{detect_format, FileFormat};
    use ampliclust::io::fastq::FastqReader;
    use ampliclust::io::fasta::{read_references, group_references};
    use ampliclust::io::bam::read_from_bam;
    use ampliclust::alignment::kmer::KmerIndex;
    use ampliclust::alignment::placement::{place_reads, PlacementConfig, calculate_placement_stats};
    use std::collections::HashMap;
    
    info!("=== Phase 1: Loading Input Data ===");
    
    // Step 1: Load reads
    info!("Loading reads from: {:?}", config.input);
    let format = detect_format(&config.input)?;
    info!("Detected format: {:?}", format);
    
    let reads = match format {
        FileFormat::Fastq | FileFormat::FastqGz => {
            let mut reader = FastqReader::from_path(&config.input)?;
            let r = reader.read_all(config.platform)?;
            info!("Loaded {} reads from FASTQ", r.len());
            r
        }
        FileFormat::Bam => {
            let r = read_from_bam(&config.input, config.platform)?;
            info!("Loaded {} reads from BAM", r.len());
            r
        }
        _ => {
            anyhow::bail!("Unsupported input format: {:?}", format);
        }
    };
    
    let initial_count = reads.len();
    
    // Filter reads by quality and length
    info!("Filtering reads (min_quality={}, max_length={})", 
          config.min_read_quality, config.max_amplicon_size);
    
    let filtered_reads: Vec<_> = reads.into_iter()
        .filter(|read| {
            let quality_ok = read.quality.as_ref()
                .map(|q| {
                    let avg_qual = q.iter().map(|&b| b as f64).sum::<f64>() / q.len() as f64;
                    avg_qual >= config.min_read_quality
                })
                .unwrap_or(true);
            
            let length_ok = read.sequence.len() <= config.max_amplicon_size;
            
            quality_ok && length_ok
        })
        .collect();
    
    info!("After filtering: {} reads retained ({:.1}%)", 
          filtered_reads.len(),
          100.0 * filtered_reads.len() as f64 / initial_count.max(1) as f64);
    
    if filtered_reads.is_empty() {
        anyhow::bail!("No reads passed quality filters!");
    }
    
    // Step 2: Load references (if reference-guided mode)
    let reference_mode = config.guide.is_some();
    
    if reference_mode {
        info!("=== Phase 2: Reference-Guided Alignment ===");
        
        let guide_path = config.guide.as_ref().unwrap();
        info!("Loading references from: {:?}", guide_path);
        
        let references = read_references(guide_path)?;
        info!("Loaded {} reference sequences", references.len());
        
        // Group references by locus
        let grouped = group_references(&references);
        info!("Grouped into {} loci", grouped.len());
        for (group, refs) in &grouped {
            info!("  {}: {} sequences", group, refs.len());
        }
        
        // Build k-mer index
        info!("Building k-mer index (k={})", config.kmer_size);
        let mut index = KmerIndex::new(config.kmer_size);
        
        for (ref_id, reference) in references.iter().enumerate() {
            index.index_reference(ref_id, &reference.sequence);
        }
        
        let stats = index.stats();
        info!("K-mer index stats:");
        info!("  Unique k-mers: {}", stats.unique_kmers);
        info!("  Total k-mers: {}", stats.total_kmers);
        info!("  Avg occurrences: {:.2}", stats.avg_occurrences);
        
        // Place reads against references
        info!("Placing reads against references...");
        
        let placement_config = PlacementConfig {
            k: config.kmer_size,
            min_hits: 5,
            min_confidence: 0.5,
            report_secondary: false,
            max_secondary: 0,
            threads: rayon::current_num_threads(),
        };
        
        let placements = place_reads(&filtered_reads, &index, &placement_config)?;
        
        // Calculate placement statistics
        let placement_stats = calculate_placement_stats(&placements);
        info!("Placement results:");
        info!("  Total reads: {}", placement_stats.total_reads);
        info!("  Placed reads: {}", placement_stats.placed_reads);
        info!("  Unplaced reads: {}", placement_stats.unplaced_reads);
        info!("  Placement rate: {:.1}%", placement_stats.placement_rate * 100.0);
        info!("  Avg confidence: {:.3}", placement_stats.avg_confidence);
        info!("  Avg hits: {:.1}", placement_stats.avg_hits);
        
        // Count reads per reference
        let mut ref_counts: HashMap<usize, usize> = HashMap::new();
        for placement in placements.iter().flatten() {
            *ref_counts.entry(placement.ref_id).or_insert(0) += 1;
        }
        
        info!("Reads per reference:");
        for (ref_id, count) in ref_counts.iter() {
            let ref_name = &references[*ref_id].name;
            info!("  {}: {} reads", ref_name, count);
        }
        
        // Write placement output
        let placement_file = format!("{}_placements.txt", config.output_prefix);
        info!("Writing placement results to: {}", placement_file);
        
        use std::io::Write;
        let mut out = std::fs::File::create(&placement_file)?;
        writeln!(out, "read_id\tref_id\tref_name\tconfidence\thits\tread_length")?;
        
        for (read, placement) in filtered_reads.iter().zip(placements.iter()) {
            if let Some(p) = placement {
                let ref_name = &references[p.ref_id].name;
                writeln!(
                    out,
                    "{}\t{}\t{}\t{:.4}\t{}\t{}",
                    read.id,
                    p.ref_id,
                    ref_name,
                    p.confidence,
                    p.hits,
                    read.sequence.len()
                )?;
            }
        }
        
        info!("Phase 1 & 2 complete! Next: implement clustering (Phase 3)");
        
    } else {
        info!("=== De novo mode ===");
        info!("De novo clustering not yet implemented");
        info!("Phase 1 complete! Reads loaded and filtered.");
    }
    
    Ok(())
}

fn run_bam_paint(
    read_info: PathBuf,
    input_bam: PathBuf,
    output_bam: PathBuf,
) -> Result<()> {
    // TODO: Implement BAM painting
    info!("Read info: {:?}", read_info);
    info!("Input BAM: {:?}", input_bam);
    info!("Output BAM: {:?}", output_bam);
    
    error!("BAM painting not yet implemented");
    Ok(())
}

fn run_stats(
    clusters: PathBuf,
    read_info: PathBuf,
    output: PathBuf,
    format: OutputFormat,
) -> Result<()> {
    // TODO: Implement statistics generation
    info!("Clusters: {:?}", clusters);
    info!("Read info: {:?}", read_info);
    info!("Output: {:?}", output);
    info!("Format: {:?}", format);
    
    error!("Statistics generation not yet implemented");
    Ok(())
}
