//! Mathematical parameter calculations for optimal bloom filter configuration.
//!
//! This module implements the core mathematical formulas for determining
//! optimal bloom filter parameters to achieve desired false positive rates.

use std::f64;

/// Parameters for configuring a bloom filter.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BloomParameters {
    /// Number of bits in the filter (m)
    pub num_bits: usize,
    /// Number of hash functions (k)
    pub num_hashes: usize,
    /// Expected number of items (n)
    pub expected_items: usize,
    /// Target false positive rate
    pub false_positive_rate: f64,
}

impl BloomParameters {
    /// Calculate optimal bloom filter parameters given expected items and desired false positive rate.
    ///
    /// # Arguments
    /// * `expected_items` - Number of items expected to be inserted (n)
    /// * `false_positive_rate` - Desired false positive rate (must be between 0 and 1)
    ///
    /// # Formula
    /// The optimal number of bits: m = -n * ln(p) / (ln(2)^2)
    /// The optimal number of hashes: k = (m/n) * ln(2)
    ///
    /// Where:
    /// - n = expected_items
    /// - p = false_positive_rate
    /// - m = num_bits
    /// - k = num_hashes
    ///
    pub fn from_item_count(expected_items: usize, false_positive_rate: f64) -> Self {
        assert!(expected_items > 0, "expected_items must be greater than 0");
        assert!(
            false_positive_rate > 0.0 && false_positive_rate < 1.0,
            "false_positive_rate must be between 0 and 1"
        );

        let n = expected_items as f64;
        let p = false_positive_rate;

        // Calculate optimal number of bits: m = -n * ln(p) / (ln(2)^2)
        let ln_2 = f64::ln(2.0);
        let num_bits = (-n * f64::ln(p) / (ln_2 * ln_2)).ceil() as usize;

        // Calculate optimal number of hashes: k = (m/n) * ln(2)
        let num_hashes = ((num_bits as f64 / n) * ln_2).ceil() as usize;

        // Ensure at least 1 hash function
        let num_hashes = num_hashes.max(1);

        Self {
            num_bits,
            num_hashes,
            expected_items,
            false_positive_rate,
        }
    }

    /// Create parameters with explicit bit count and item count, calculating optimal hash count.
    ///
    /// # Arguments
    /// * `num_bits` - Number of bits to allocate (m)
    /// * `expected_items` - Number of items expected to be inserted (n)
    ///
    /// # Panics
    /// Panics if num_bits or expected_items is 0
    pub fn from_bit_count(num_bits: usize, expected_items: usize) -> Self {
        assert!(num_bits > 0, "num_bits must be greater than 0");
        assert!(expected_items > 0, "expected_items must be greater than 0");

        let m = num_bits as f64;
        let n = expected_items as f64;

        // Calculate optimal number of hashes: k = (m/n) * ln(2)
        let num_hashes = ((m / n) * f64::ln(2.0)).ceil() as usize;
        let num_hashes = num_hashes.max(1);

        // Calculate actual false positive rate for these parameters
        // Formula: p = (1 - e^(-kn/m))^k
        let false_positive_rate = Self::calculate_fpr(num_bits, num_hashes, expected_items);

        Self {
            num_bits,
            num_hashes,
            expected_items,
            false_positive_rate,
        }
    }

    /// Calculate the theoretical false positive rate for given parameters.
    ///
    /// Formula: p = (1 - e^(-kn/m))^k
    ///
    /// Where:
    /// - k = num_hashes
    /// - n = expected_items
    /// - m = num_bits
    pub fn calculate_fpr(num_bits: usize, num_hashes: usize, expected_items: usize) -> f64 {
        let m = num_bits as f64;
        let k = num_hashes as f64;
        let n = expected_items as f64;

        // p = (1 - e^(-kn/m))^k
        let exponent = -k * n / m;
        let base = 1.0 - f64::exp(exponent);
        f64::powf(base, k)
    }

    /// Get the actual false positive rate after inserting items.
    ///
    /// This recalculates the FPR based on the actual number of items inserted.
    pub fn actual_fpr(&self, actual_items: usize) -> f64 {
        Self::calculate_fpr(self.num_bits, self.num_hashes, actual_items)
    }

    /// Validate parameters for sanity.
    pub fn validate(&self) -> Result<(), String> {
        if self.num_bits == 0 {
            return Err("num_bits must be greater than 0".to_string());
        }
        if self.num_hashes == 0 {
            return Err("num_hashes must be greater than 0".to_string());
        }
        if self.expected_items == 0 {
            return Err("expected_items must be greater than 0".to_string());
        }
        if self.false_positive_rate <= 0.0 || self.false_positive_rate >= 1.0 {
            return Err("false_positive_rate must be between 0 and 1".to_string());
        }
        Ok(())
    }
}
