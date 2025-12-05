/// Read filtering utilities

use super::SequenceRead;

pub struct ReadFilter {
    pub min_quality: f64,
    pub max_length: usize,
    pub min_length: usize,
}

impl ReadFilter {
    pub fn new(min_quality: f64, max_length: usize, min_length: usize) -> Self {
        Self {
            min_quality,
            max_length,
            min_length,
        }
    }

    pub fn passes(&self, read: &SequenceRead) -> bool {
        read.passes_quality(self.min_quality) 
            && read.length <= self.max_length
            && read.length >= self.min_length
    }
}
