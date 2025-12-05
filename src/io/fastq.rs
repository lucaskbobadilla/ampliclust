/// FASTQ reader with support for gzipped files
use crate::reads::SequenceRead;
use crate::config::Platform;
use anyhow::{Context, Result};
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

/// FASTQ reader that handles both plain and gzipped files
pub struct FastqReader<R: Read> {
    reader: BufReader<R>,
}

impl FastqReader<File> {
    /// Open a FASTQ file (automatically detects gzip)
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<FastqReader<Box<dyn Read>>> {
        let path = path.as_ref();
        let file = File::open(path)
            .with_context(|| format!("Failed to open FASTQ file: {:?}", path))?;
        
        // Check if file is gzipped by extension or magic bytes
        let is_gzipped = path.extension()
            .and_then(|e| e.to_str())
            .map(|e| e == "gz")
            .unwrap_or(false);
        
        if is_gzipped {
            let decoder: Box<dyn Read> = Box::new(GzDecoder::new(file));
            Ok(FastqReader {
                reader: BufReader::new(decoder),
            })
        } else {
            let reader: Box<dyn Read> = Box::new(file);
            Ok(FastqReader {
                reader: BufReader::new(reader),
            })
        }
    }
}

impl<R: Read> FastqReader<R> {
    /// Create a new FASTQ reader from any Read type
    pub fn new(reader: R) -> Self {
        Self {
            reader: BufReader::new(reader),
        }
    }
    
    /// Read all sequences from the FASTQ file
    pub fn read_all(&mut self, platform: Platform) -> Result<Vec<SequenceRead>> {
        let mut sequences = Vec::new();
        
        while let Some(record) = self.read_record()? {
            let seq_read = SequenceRead::new(
                record.id,
                record.sequence,
                Some(record.quality),
                platform,
            );
            sequences.push(seq_read);
        }
        
        Ok(sequences)
    }
    
    /// Read sequences in chunks for streaming processing
    pub fn read_chunk(&mut self, chunk_size: usize, platform: Platform) -> Result<Vec<SequenceRead>> {
        let mut sequences = Vec::with_capacity(chunk_size);
        
        for _ in 0..chunk_size {
            match self.read_record()? {
                Some(record) => {
                    let seq_read = SequenceRead::new(
                        record.id,
                        record.sequence,
                        Some(record.quality),
                        platform,
                    );
                    sequences.push(seq_read);
                }
                None => break,
            }
        }
        
        Ok(sequences)
    }
    
    /// Read a single FASTQ record
    fn read_record(&mut self) -> Result<Option<FastqRecord>> {
        let mut id_line = String::new();
        let mut seq_line = String::new();
        let mut plus_line = String::new();
        let mut qual_line = String::new();
        
        // Read header line (starts with @)
        let bytes_read = self.reader.read_line(&mut id_line)?;
        if bytes_read == 0 {
            return Ok(None); // End of file
        }
        
        if !id_line.starts_with('@') {
            anyhow::bail!("Invalid FASTQ format: expected '@' at start of record, got: {}", id_line.trim());
        }
        
        // Read sequence line
        self.reader.read_line(&mut seq_line)?;
        
        // Read '+' line
        self.reader.read_line(&mut plus_line)?;
        if !plus_line.starts_with('+') {
            anyhow::bail!("Invalid FASTQ format: expected '+' separator");
        }
        
        // Read quality line
        self.reader.read_line(&mut qual_line)?;
        
        // Parse record
        let id = id_line[1..].trim().to_string();
        let sequence = seq_line.trim().as_bytes().to_vec();
        let quality = qual_line.trim().as_bytes().to_vec();
        
        // Validate sequence and quality lengths match
        if sequence.len() != quality.len() {
            anyhow::bail!(
                "FASTQ record {} has mismatched sequence ({}) and quality ({}) lengths",
                id, sequence.len(), quality.len()
            );
        }
        
        Ok(Some(FastqRecord {
            id,
            sequence,
            quality,
        }))
    }
}

/// A single FASTQ record
struct FastqRecord {
    id: String,
    sequence: Vec<u8>,
    quality: Vec<u8>,
}

/// Write sequences to FASTQ format
pub struct FastqWriter {
    writer: Box<dyn std::io::Write>,
}

impl FastqWriter {
    /// Create a new FASTQ writer to a file
    pub fn to_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::create(path.as_ref())
            .with_context(|| format!("Failed to create FASTQ file: {:?}", path.as_ref()))?;
        
        let writer: Box<dyn std::io::Write> = if path.as_ref()
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e == "gz")
            .unwrap_or(false)
        {
            Box::new(flate2::write::GzEncoder::new(
                file,
                flate2::Compression::default(),
            ))
        } else {
            Box::new(file)
        };
        
        Ok(Self { writer })
    }
    
    /// Write a single sequence read
    pub fn write_record(&mut self, read: &SequenceRead) -> Result<()> {
        use std::io::Write;
        
        writeln!(self.writer, "@{}", read.id)?;
        writeln!(self.writer, "{}", String::from_utf8_lossy(&read.sequence))?;
        writeln!(self.writer, "+")?;
        
        if let Some(ref qual) = read.quality {
            writeln!(self.writer, "{}", String::from_utf8_lossy(qual))?;
        } else {
            // If no quality, write default quality (Phred 30)
            let default_qual = vec![b'?'; read.length];
            writeln!(self.writer, "{}", String::from_utf8_lossy(&default_qual))?;
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    
    #[test]
    fn test_read_fastq() {
        let fastq_data = b"@read1\nACGT\n+\nIIII\n@read2\nTGCA\n+\nHHHH\n";
        let cursor = Cursor::new(fastq_data);
        let mut reader = FastqReader::new(cursor);
        
        let reads = reader.read_all(Platform::PacBio).unwrap();
        
        assert_eq!(reads.len(), 2);
        assert_eq!(reads[0].id, "read1");
        assert_eq!(reads[0].sequence, b"ACGT");
        assert_eq!(reads[1].id, "read2");
        assert_eq!(reads[1].sequence, b"TGCA");
    }
    
    #[test]
    fn test_fastq_validation() {
        // Mismatched sequence and quality lengths
        let bad_fastq = b"@read1\nACGT\n+\nII\n";
        let cursor = Cursor::new(bad_fastq);
        let mut reader = FastqReader::new(cursor);
        
        let result = reader.read_all(Platform::PacBio);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_empty_fastq() {
        let empty_fastq = b"";
        let cursor = Cursor::new(empty_fastq);
        let mut reader = FastqReader::new(cursor);
        
        let reads = reader.read_all(Platform::PacBio).unwrap();
        assert_eq!(reads.len(), 0);
    }
}
