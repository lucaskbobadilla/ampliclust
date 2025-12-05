/// Quality score handling and conversion

/// Convert Phred quality to error probability
pub fn phred_to_prob(phred: u8) -> f64 {
    10_f64.powf(-(phred as f64) / 10.0)
}

/// Convert error probability to Phred quality
pub fn prob_to_phred(prob: f64) -> u8 {
    let phred = -10.0 * prob.log10();
    phred.round().min(93.0).max(0.0) as u8
}

/// Quality score representation
#[derive(Debug, Clone, Copy)]
pub struct QualityScore(pub u8);

impl QualityScore {
    pub fn new(score: u8) -> Self {
        Self(score)
    }

    pub fn error_probability(&self) -> f64 {
        phred_to_prob(self.0)
    }

    pub fn from_ascii(ascii: u8) -> Self {
        Self(ascii.saturating_sub(33))
    }

    pub fn to_ascii(&self) -> u8 {
        self.0 + 33
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phred_conversion() {
        let prob = phred_to_prob(20);
        assert!((prob - 0.01).abs() < 0.001);
        
        let phred = prob_to_phred(0.01);
        assert_eq!(phred, 20);
    }
}
