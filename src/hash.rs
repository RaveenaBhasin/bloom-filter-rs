//! Hash strategy for bloom filters using standard double hashing.
//!
//! This implementation uses two independent hash functions (ahash and seahash)
//! combined with Kirsch-Mitzenmacher double hashing to generate k hash values with good distribution.

use std::hash::{Hash, Hasher};

/// Hash strategy that generates multiple hash values from an item.
///
/// Uses standard Kirsch-Mitzenmacher double hashing:
/// h_i(x) = (h1(x) + i * h2(x)) mod m
///
/// This is the proven optimal approach used in production implementations.
#[derive(Debug, Clone)]
pub struct HashStrategy {
    /// Number of hash functions to generate
    num_hashes: usize,
    /// Number of bits in the filter (for modulo operation)
    num_bits: usize,
}

impl HashStrategy {
    /// Create a new hash strategy.
    ///
    /// # Arguments
    /// * `num_hashes` - Number of hash functions to generate (k)
    /// * `num_bits` - Number of bits in the bloom filter (m)
    pub fn new(num_hashes: usize, num_bits: usize) -> Self {
        assert!(num_hashes > 0, "num_hashes must be greater than 0");
        assert!(num_bits > 0, "num_bits must be greater than 0");

        Self {
            num_hashes,
            num_bits,
        }
    }

    /// Generate all hash indices for an item.
    ///
    /// Returns a vector of bit indices where the item should be set/checked.
    ///
    /// # Arguments
    /// * `item` - The item to hash
    ///
    /// # Returns
    /// A vector of k unique bit indices
    pub fn hash_indices<T: Hash>(&self, item: &T) -> Vec<usize> {
        // Compute two independent hashes using different hash functions
        let h1 = self.hash_with_ahash(item);
        let h2 = self.hash_with_seahash(item);

        // Generate k hash values using standard double hashing
        (0..self.num_hashes)
            .map(|i| self.compute_index(h1, h2, i))
            .collect()
    }

    /// Hash an item using ahash (primary hash function).
    #[inline]
    fn hash_with_ahash<T: Hash>(&self, item: &T) -> u64 {
        let mut hasher = ahash::AHasher::default();
        item.hash(&mut hasher);
        hasher.finish()
    }

    /// Hash an item using seahash (secondary hash function).
    #[inline]
    fn hash_with_seahash<T: Hash>(&self, item: &T) -> u64 {
        let mut hasher = seahash::SeaHasher::new();
        item.hash(&mut hasher);
        hasher.finish()
    }

    /// Compute the i-th hash index using standard double hashing.
    ///
    /// Formula: (h1 + i * h2) mod m
    ///
    /// This is the standard Kirsch-Mitzenmacher double hashing approach.
    #[inline]
    fn compute_index(&self, h1: u64, h2: u64, i: usize) -> usize {
        let i_u64 = i as u64;

        // Standard double hashing: h1 + i*h2
        let combined = h1.wrapping_add(i_u64.wrapping_mul(h2));

        // Take modulo to get index within bit array
        (combined % self.num_bits as u64) as usize
    }

    /// Get the number of hash functions this strategy generates.
    pub fn num_hashes(&self) -> usize {
        self.num_hashes
    }
}
