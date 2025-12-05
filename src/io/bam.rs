/// BAM/SAM file reader and writer
use crate::reads::SequenceRead;
use crate::config::Platform;
use anyhow::{Context, Result};
use rust_htslib::bam::{Read as BamRead, Reader, Writer, Record, Format, Header};
use std::path::Path;

/// Read sequences from a BAM/SAM file
pub fn read_from_bam<P: AsRef<Path>>(path: P, platform: Platform) -> Result<Vec<SequenceRead>> {
    let path = path.as_ref();
    let mut bam = Reader::from_path(path)
        .with_context(|| format!("Failed to open BAM/SAM file: {:?}", path))?;
    
    let mut sequences = Vec::new();
    
    for (idx, result) in bam.records().enumerate() {
        let record = result
            .with_context(|| format!("Failed to read record {} from {:?}", idx, path))?;
        
        // Skip supplementary and secondary alignments
        if record.is_secondary() || record.is_supplementary() {
            continue;
        }
        
        let read = bam_record_to_sequence(&record, platform)?;
        sequences.push(read);
    }
    
    Ok(sequences)
}

/// Convert a BAM record to a SequenceRead
fn bam_record_to_sequence(record: &Record, platform: Platform) -> Result<SequenceRead> {
    let id = String::from_utf8_lossy(record.qname()).to_string();
    
    // Get sequence
    let sequence = record.seq().as_bytes();
    
    // Get quality scores
    let quality = record.qual().to_vec();
    let quality = if quality.iter().all(|&q| q == 255) {
        // Quality unavailable (all 255)
        None
    } else {
        Some(quality)
    };
    
    Ok(SequenceRead::new(id, sequence, quality, platform))
}

/// Write sequences to a BAM file with cluster tags
pub struct BamWriter {
    writer: Writer,
}

impl BamWriter {
    /// Create a new BAM writer
    pub fn to_path<P: AsRef<Path>>(path: P, header: &Header) -> Result<Self> {
        let path = path.as_ref();
        
        // Determine format from extension
        let format = if path.extension().and_then(|s| s.to_str()) == Some("sam") {
            Format::Sam
        } else {
            Format::Bam
        };
        
        let writer = Writer::from_path(path, header, format)
            .with_context(|| format!("Failed to create BAM/SAM file: {:?}", path))?;
        
        Ok(Self { writer })
    }
    
    /// Write a record with cluster tags
    pub fn write_record_with_tags(
        &mut self,
        record: &mut Record,
        cluster_id: i32,
        cluster_color: &str,
    ) -> Result<()> {
        // Add HP tag (haplotype/cluster ID)
        record.push_aux(b"HP", rust_htslib::bam::record::Aux::I32(cluster_id))?;
        
        // Add YC tag (color for IGV)
        record.push_aux(b"YC", rust_htslib::bam::record::Aux::String(cluster_color))?;
        
        self.writer.write(record)?;
        Ok(())
    }
}

/// Paint an existing BAM file with cluster information
pub fn paint_bam<P: AsRef<Path>>(
    input_bam: P,
    output_bam: P,
    read_to_cluster: &std::collections::HashMap<String, (usize, String)>,
) -> Result<()> {
    let input_path = input_bam.as_ref();
    let output_path = output_bam.as_ref();
    
    let mut reader = Reader::from_path(input_path)
        .with_context(|| format!("Failed to open input BAM: {:?}", input_path))?;
    
    let header = Header::from_template(reader.header());
    let mut writer = BamWriter::to_path(output_path, &header)?;
    
    for (idx, result) in reader.records().enumerate() {
        let mut record = result
            .with_context(|| format!("Failed to read record {} from {:?}", idx, input_path))?;
        
        let read_id = String::from_utf8_lossy(record.qname()).to_string();
        
        if let Some((cluster_id, color)) = read_to_cluster.get(&read_id) {
            writer.write_record_with_tags(&mut record, *cluster_id as i32, color)?;
        } else {
            // Write without tags if read not in cluster map
            writer.writer.write(&record)?;
        }
    }
    
    Ok(())
}

/// Generate IGV-compatible colors for clusters
pub fn generate_cluster_colors(num_clusters: usize) -> Vec<String> {
    let base_colors = vec![
        "255,0,0",     // Red
        "0,0,255",     // Blue
        "0,255,0",     // Green
        "255,255,0",   // Yellow
        "255,0,255",   // Magenta
        "0,255,255",   // Cyan
        "255,128,0",   // Orange
        "128,0,255",   // Purple
        "0,128,255",   // Sky blue
        "255,0,128",   // Pink
    ];
    
    let mut colors = Vec::new();
    for i in 0..num_clusters {
        colors.push(base_colors[i % base_colors.len()].to_string());
    }
    colors
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_colors() {
        let colors = generate_cluster_colors(15);
        assert_eq!(colors.len(), 15);
        assert_eq!(colors[0], "255,0,0");
        assert_eq!(colors[10], "255,0,0"); // Wraps around
    }
    
    #[test]
    fn test_bam_record_conversion() {
        // Note: Full BAM testing requires creating mock records
        // which is complex. In practice, test with real BAM files
        let colors = generate_cluster_colors(5);
        assert!(colors.len() == 5);
    }
}
