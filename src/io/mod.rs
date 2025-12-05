/// I/O module for reading and writing sequence files
pub mod fastq;
pub mod bam;
pub mod fasta;
pub mod formats;

// Re-export commonly used items
pub use fastq::{FastqReader, FastqWriter};
pub use fasta::{Reference, read_references, group_references, FastaWriter};
pub use bam::{read_from_bam, paint_bam, BamWriter, generate_cluster_colors};
pub use formats::{FileFormat, detect_format, read_fofn, is_fofn, validate_input_file};
