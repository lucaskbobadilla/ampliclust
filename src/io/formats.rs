/// File format detection and utilities
use anyhow::{Context, Result};
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Detected file format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileFormat {
    Fastq,
    FastqGz,
    Fasta,
    FastaGz,
    Bam,
    Sam,
    Unknown,
}

/// Detect file format from extension and magic bytes
pub fn detect_format<P: AsRef<Path>>(path: P) -> Result<FileFormat> {
    let path = path.as_ref();
    
    // First try extension
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        let format = match ext.to_lowercase().as_str() {
            "fq" | "fastq" => FileFormat::Fastq,
            "gz" => {
                // Check if it's fastq.gz or fasta.gz
                if let Some(stem) = path.file_stem()
                    .and_then(|s| s.to_str())
                    .and_then(|s| Path::new(s).extension())
                    .and_then(|e| e.to_str())
                {
                    match stem.to_lowercase().as_str() {
                        "fq" | "fastq" => return Ok(FileFormat::FastqGz),
                        "fa" | "fasta" | "fna" => return Ok(FileFormat::FastaGz),
                        _ => {}
                    }
                }
                FileFormat::Unknown
            }
            "fa" | "fasta" | "fna" => FileFormat::Fasta,
            "bam" => FileFormat::Bam,
            "sam" => FileFormat::Sam,
            _ => FileFormat::Unknown,
        };
        
        if format != FileFormat::Unknown {
            return Ok(format);
        }
    }
    
    // Fall back to magic bytes
    detect_format_from_magic(path)
}

/// Detect format from magic bytes (file signature)
fn detect_format_from_magic<P: AsRef<Path>>(path: P) -> Result<FileFormat> {
    let mut file = File::open(path.as_ref())
        .with_context(|| format!("Failed to open file: {:?}", path.as_ref()))?;
    
    let mut magic = [0u8; 4];
    let bytes_read = file.read(&mut magic)?;
    
    if bytes_read < 4 {
        return Ok(FileFormat::Unknown);
    }
    
    // Check magic bytes
    let format = if magic[0..2] == [0x1f, 0x8b] {
        // Gzipped file - could be .fastq.gz or .fasta.gz
        // We can't determine without decompressing, so default to FastqGz
        FileFormat::FastqGz
    } else if magic == [b'B', b'A', b'M', 0x01] {
        FileFormat::Bam
    } else if magic[0] == b'@' {
        FileFormat::Fastq
    } else if magic[0] == b'>' {
        FileFormat::Fasta
    } else {
        FileFormat::Unknown
    };
    
    Ok(format)
}

/// Read a file-of-filenames (FOFN)
pub fn read_fofn<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
    let content = std::fs::read_to_string(path.as_ref())
        .with_context(|| format!("Failed to read FOFN: {:?}", path.as_ref()))?;
    
    let paths: Vec<String> = content
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| line.to_string())
        .collect();
    
    if paths.is_empty() {
        anyhow::bail!("FOFN file is empty: {:?}", path.as_ref());
    }
    
    Ok(paths)
}

/// Check if a path is a FOFN (by extension or content)
pub fn is_fofn<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();
    
    // Check extension
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        if ext.to_lowercase() == "fofn" {
            return true;
        }
    }
    
    // Check if file contains paths (heuristic: ends with common extensions)
    if let Ok(content) = std::fs::read_to_string(path) {
        let lines: Vec<&str> = content.lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .take(10) // Check first 10 lines
            .collect();
        
        if lines.is_empty() {
            return false;
        }
        
        // If most lines look like file paths, it's probably a FOFN
        let path_like_count = lines.iter()
            .filter(|line| {
                Path::new(line).extension().is_some() &&
                line.contains(std::path::MAIN_SEPARATOR)
            })
            .count();
        
        return path_like_count >= lines.len() / 2;
    }
    
    false
}

/// Validate that a file exists and is readable
pub fn validate_input_file<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();
    
    if !path.exists() {
        anyhow::bail!("File does not exist: {:?}", path);
    }
    
    if !path.is_file() {
        anyhow::bail!("Path is not a file: {:?}", path);
    }
    
    // Try to open for reading
    File::open(path)
        .with_context(|| format!("Cannot read file: {:?}", path))?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;
    
    #[test]
    fn test_detect_format_by_extension() {
        assert_eq!(detect_format("test.fastq").unwrap(), FileFormat::Fastq);
        assert_eq!(detect_format("test.fq").unwrap(), FileFormat::Fastq);
        assert_eq!(detect_format("test.fasta").unwrap(), FileFormat::Fasta);
        assert_eq!(detect_format("test.fa").unwrap(), FileFormat::Fasta);
        assert_eq!(detect_format("test.bam").unwrap(), FileFormat::Bam);
        assert_eq!(detect_format("test.sam").unwrap(), FileFormat::Sam);
        assert_eq!(detect_format("test.fastq.gz").unwrap(), FileFormat::FastqGz);
        assert_eq!(detect_format("test.fasta.gz").unwrap(), FileFormat::FastaGz);
    }
    
    #[test]
    fn test_detect_fasta_magic() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, ">seq1").unwrap();
        writeln!(temp_file, "ACGT").unwrap();
        
        let format = detect_format(temp_file.path()).unwrap();
        assert_eq!(format, FileFormat::Fasta);
    }
    
    #[test]
    fn test_detect_fastq_magic() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "@read1").unwrap();
        writeln!(temp_file, "ACGT").unwrap();
        
        let format = detect_format(temp_file.path()).unwrap();
        assert_eq!(format, FileFormat::Fastq);
    }
    
    #[test]
    fn test_read_fofn() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "/path/to/file1.fastq").unwrap();
        writeln!(temp_file, "# This is a comment").unwrap();
        writeln!(temp_file, "/path/to/file2.fastq").unwrap();
        writeln!(temp_file, "").unwrap();
        writeln!(temp_file, "/path/to/file3.fastq").unwrap();
        
        let paths = read_fofn(temp_file.path()).unwrap();
        assert_eq!(paths.len(), 3);
        assert_eq!(paths[0], "/path/to/file1.fastq");
        assert_eq!(paths[1], "/path/to/file2.fastq");
        assert_eq!(paths[2], "/path/to/file3.fastq");
    }
    
    #[test]
    fn test_validate_input_file() {
        let temp_file = NamedTempFile::new().unwrap();
        assert!(validate_input_file(temp_file.path()).is_ok());
        
        assert!(validate_input_file("/nonexistent/file.txt").is_err());
    }
}
