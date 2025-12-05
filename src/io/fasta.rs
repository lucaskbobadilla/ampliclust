/// FASTA reader for reference sequences
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

/// Reference sequence with optional grouping information
#[derive(Debug, Clone)]
pub struct Reference {
    /// Sequence name (before '|' separator)
    pub name: String,
    
    /// Optional group name (after '|' separator)
    pub group: Option<String>,
    
    /// DNA sequence
    pub sequence: Vec<u8>,
    
    /// Sequence length (cached)
    pub length: usize,
}

impl Reference {
    /// Create a new reference sequence
    pub fn new(name: String, group: Option<String>, sequence: Vec<u8>) -> Self {
        let length = sequence.len();
        Self {
            name,
            group,
            sequence,
            length,
        }
    }
    
    /// Get the full identifier (name|group or just name)
    pub fn full_id(&self) -> String {
        match &self.group {
            Some(group) => format!("{}|{}", self.name, group),
            None => self.name.clone(),
        }
    }
    
    /// Get the group name, defaulting to the sequence name if no group specified
    pub fn group_name(&self) -> String {
        self.group.as_ref().unwrap_or(&self.name).clone()
    }
}

/// Read reference sequences from a FASTA file
pub fn read_references<P: AsRef<Path>>(path: P) -> Result<Vec<Reference>> {
    let path = path.as_ref();
    let file = File::open(path)
        .with_context(|| format!("Failed to open FASTA file: {:?}", path))?;
    
    let reader = BufReader::new(file);
    let mut references = Vec::new();
    let mut current_id: Option<String> = None;
    let mut current_seq = Vec::new();
    
    for (line_num, line_result) in reader.lines().enumerate() {
        let line = line_result
            .with_context(|| format!("Failed to read line {} from {:?}", line_num + 1, path))?;
        
        let line = line.trim();
        
        if line.is_empty() {
            continue; // Skip empty lines
        }
        
        if line.starts_with('>') {
            // Save previous sequence if exists
            if let Some(id) = current_id.take() {
                if current_seq.is_empty() {
                    anyhow::bail!("Empty sequence for record '{}' in {:?}", id, path);
                }
                
                let reference = parse_reference(id, current_seq)?;
                references.push(reference);
                current_seq = Vec::new();
            }
            
            // Start new sequence
            current_id = Some(line[1..].trim().to_string());
        } else if line.starts_with(';') {
            // Comment line, skip
            continue;
        } else {
            // Sequence line
            if current_id.is_none() {
                anyhow::bail!(
                    "Sequence data before header at line {} in {:?}",
                    line_num + 1,
                    path
                );
            }
            
            // Validate and add sequence data
            for &base in line.as_bytes() {
                match base {
                    b'A' | b'C' | b'G' | b'T' | b'N' |
                    b'a' | b'c' | b'g' | b't' | b'n' |
                    b'R' | b'Y' | b'S' | b'W' | b'K' | b'M' |
                    b'r' | b'y' | b's' | b'w' | b'k' | b'm' => {
                        current_seq.push(base.to_ascii_uppercase());
                    }
                    b' ' | b'\t' => continue, // Skip whitespace
                    _ => {
                        anyhow::bail!(
                            "Invalid character '{}' in sequence at line {} of {:?}",
                            base as char,
                            line_num + 1,
                            path
                        );
                    }
                }
            }
        }
    }
    
    // Don't forget the last sequence
    if let Some(id) = current_id {
        if current_seq.is_empty() {
            anyhow::bail!("Empty sequence for record '{}' in {:?}", id, path);
        }
        let reference = parse_reference(id, current_seq)?;
        references.push(reference);
    }
    
    if references.is_empty() {
        anyhow::bail!("No sequences found in FASTA file: {:?}", path);
    }
    
    Ok(references)
}

/// Parse a reference from ID and sequence, handling name|group format
fn parse_reference(id: String, sequence: Vec<u8>) -> Result<Reference> {
    let (name, group) = if id.contains('|') {
        let parts: Vec<&str> = id.splitn(2, '|').collect();
        (parts[0].to_string(), Some(parts[1].to_string()))
    } else {
        (id, None)
    };
    
    Ok(Reference::new(name, group, sequence))
}

/// Group references by their group name
pub fn group_references(references: &[Reference]) -> HashMap<String, Vec<usize>> {
    let mut groups: HashMap<String, Vec<usize>> = HashMap::new();
    
    for (idx, reference) in references.iter().enumerate() {
        let group_name = reference.group_name();
        groups.entry(group_name).or_insert_with(Vec::new).push(idx);
    }
    
    groups
}

/// Write references to a FASTA file
pub struct FastaWriter {
    writer: Box<dyn Write>,
    line_width: usize,
}

impl FastaWriter {
    /// Create a new FASTA writer
    pub fn to_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::create(path.as_ref())
            .with_context(|| format!("Failed to create FASTA file: {:?}", path.as_ref()))?;
        
        Ok(Self {
            writer: Box::new(file),
            line_width: 80,
        })
    }
    
    /// Set the line width for sequence wrapping (default: 80)
    pub fn with_line_width(mut self, width: usize) -> Self {
        self.line_width = width;
        self
    }
    
    /// Write a reference sequence
    pub fn write_reference(&mut self, reference: &Reference) -> Result<()> {
        writeln!(self.writer, ">{}", reference.full_id())?;
        
        // Write sequence with line wrapping
        for chunk in reference.sequence.chunks(self.line_width) {
            writeln!(self.writer, "{}", String::from_utf8_lossy(chunk))?;
        }
        
        Ok(())
    }
    
    /// Write a sequence with custom header
    pub fn write_sequence(&mut self, header: &str, sequence: &[u8]) -> Result<()> {
        writeln!(self.writer, ">{}", header)?;
        
        for chunk in sequence.chunks(self.line_width) {
            writeln!(self.writer, "{}", String::from_utf8_lossy(chunk))?;
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;
    
    #[test]
    fn test_read_simple_fasta() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, ">seq1").unwrap();
        writeln!(temp_file, "ACGT").unwrap();
        writeln!(temp_file, ">seq2").unwrap();
        writeln!(temp_file, "TGCA").unwrap();
        
        let refs = read_references(temp_file.path()).unwrap();
        
        assert_eq!(refs.len(), 2);
        assert_eq!(refs[0].name, "seq1");
        assert_eq!(refs[0].sequence, b"ACGT");
        assert_eq!(refs[1].name, "seq2");
        assert_eq!(refs[1].sequence, b"TGCA");
    }
    
    #[test]
    fn test_read_grouped_fasta() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, ">Allele1|HLA-A").unwrap();
        writeln!(temp_file, "ACGT").unwrap();
        writeln!(temp_file, ">Allele2|HLA-A").unwrap();
        writeln!(temp_file, "ACGG").unwrap();
        
        let refs = read_references(temp_file.path()).unwrap();
        
        assert_eq!(refs.len(), 2);
        assert_eq!(refs[0].name, "Allele1");
        assert_eq!(refs[0].group, Some("HLA-A".to_string()));
        assert_eq!(refs[1].name, "Allele2");
        assert_eq!(refs[1].group, Some("HLA-A".to_string()));
    }
    
    #[test]
    fn test_group_references() {
        let refs = vec![
            Reference::new("A1".to_string(), Some("HLA-A".to_string()), b"ACGT".to_vec()),
            Reference::new("A2".to_string(), Some("HLA-A".to_string()), b"ACGG".to_vec()),
            Reference::new("B1".to_string(), Some("HLA-B".to_string()), b"TGCA".to_vec()),
        ];
        
        let groups = group_references(&refs);
        
        assert_eq!(groups.len(), 2);
        assert_eq!(groups.get("HLA-A").unwrap().len(), 2);
        assert_eq!(groups.get("HLA-B").unwrap().len(), 1);
    }
    
    #[test]
    fn test_multiline_sequence() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, ">seq1").unwrap();
        writeln!(temp_file, "ACGT").unwrap();
        writeln!(temp_file, "TGCA").unwrap();
        
        let refs = read_references(temp_file.path()).unwrap();
        
        assert_eq!(refs[0].sequence, b"ACGTTGCA");
    }
    
    #[test]
    fn test_lowercase_conversion() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, ">seq1").unwrap();
        writeln!(temp_file, "acgt").unwrap();
        
        let refs = read_references(temp_file.path()).unwrap();
        
        assert_eq!(refs[0].sequence, b"ACGT");
    }
}
