// Example demonstrating Phase 1 I/O functionality
use ampliclust::io::*;
use ampliclust::config::Platform;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    println!("=== AmpliClust Phase 1 I/O Demo ===\n");
    
    // Example 1: Read FASTA references
    println!("1. Reading FASTA references...");
    if Path::new("test_reference.fasta").exists() {
        let references = read_references("test_reference.fasta")?;
        println!("   Loaded {} reference sequences", references.len());
        for ref_seq in &references {
            println!("   - {} (group: {:?}, length: {})", 
                     ref_seq.name, ref_seq.group, ref_seq.length);
        }
        
        // Group references
        let groups = group_references(&references);
        println!("   Found {} unique groups", groups.len());
    } else {
        println!("   (test_reference.fasta not found, skipping)");
    }
    
    // Example 2: Read FASTQ
    println!("\n2. Reading FASTQ files...");
    if Path::new("test.fastq").exists() {
        let mut reader = FastqReader::from_path("test.fastq")?;
        let reads = reader.read_all(Platform::PacBio)?;
        println!("   Loaded {} reads", reads.len());
        if !reads.is_empty() {
            println!("   First read: {} (length: {}, quality: {:?})", 
                     reads[0].id, reads[0].length, reads[0].avg_quality);
        }
    } else {
        println!("   (test.fastq not found, skipping)");
    }
    
    // Example 3: Format detection
    println!("\n3. Format detection:");
    let test_files = vec![
        "test.fastq",
        "test.fastq.gz",
        "test.fasta",
        "test.bam",
    ];
    
    for file in test_files {
        let format = detect_format(file)?;
        println!("   {} -> {:?}", file, format);
    }
    
    // Example 4: Create test FASTQ
    println!("\n4. Writing test FASTQ...");
    let mut writer = FastqWriter::to_path("demo_output.fastq")?;
    let test_read = ampliclust::reads::SequenceRead::new(
        "demo_read_1".to_string(),
        b"ACGTACGTACGT".to_vec(),
        Some(vec![b'I'; 12]),
        Platform::PacBio,
    );
    writer.write_record(&test_read)?;
    println!("   Created demo_output.fastq");
    
    // Example 5: Create test FASTA
    println!("\n5. Writing test FASTA...");
    let mut fasta_writer = FastaWriter::to_path("demo_output.fasta")?;
    fasta_writer.write_sequence("test_sequence|test_group", b"ACGTACGTACGTACGT")?;
    println!("   Created demo_output.fasta");
    
    println!("\n=== Demo Complete ===");
    println!("All I/O operations working correctly! ✅");
    
    Ok(())
}
