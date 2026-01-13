//! Bit array implementation for storing bloom filter state.
//!
//! This module provides a simple, correct bit array implementation
//! using a vector of 64-bit integers.

/// A bit array for storing bloom filter state.
///
/// Internally uses a `Vec<u64>` where each u64 stores 64 bits.
/// Bits are indexed from 0 to (capacity - 1).
#[derive(Debug, Clone, PartialEq)]
pub struct BitArray {
    /// Internal storage of bits, each u64 holds 64 bits
    words: Vec<u64>,
    /// Total number of bits this array can hold
    capacity: usize,
}

impl BitArray {
    /// Create a new bit array with the specified capacity in bits.
    ///
    /// All bits are initialized to 0 (unset).
    ///
    /// # Arguments
    /// * `capacity` - Number of bits the array should hold
    ///
    /// # Panics
    /// Panics if capacity is 0
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must be greater than 0");

        // Calculate how many u64 words we need
        // We need to round up: (capacity + 63) / 64
        let num_words = (capacity + 63) / 64;

        Self {
            words: vec![0u64; num_words],
            capacity,
        }
    }

    /// Get the capacity of the bit array (total number of bits).
    #[inline]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Set the bit at the given index to 1.
    ///
    /// # Arguments
    /// * `index` - The bit index to set (0-indexed)
    #[inline]
    pub fn set(&mut self, index: usize) {
        assert!(index < self.capacity, "index out of bounds");

        // Determine which word and which bit within that word
        let word_index = index / 64;
        let bit_index = index % 64;

        // Set the bit using bitwise OR
        self.words[word_index] |= 1u64 << bit_index;
    }

    /// Get the value of the bit at the given index.
    ///
    /// Returns `true` if the bit is set (1), `false` if unset (0).
    #[inline]
    pub fn get(&self, index: usize) -> bool {
        assert!(index < self.capacity, "index out of bounds");

        // Determine which word and which bit within that word
        let word_index = index / 64;
        let bit_index = index % 64;

        // Check if the bit is set using bitwise AND
        (self.words[word_index] & (1u64 << bit_index)) != 0
    }

    /// Clear all bits in the array (set to 0).
    pub fn clear(&mut self) {
        self.words.fill(0);
    }

    /// Count the number of set bits (1s) in the array.
    pub fn count_ones(&self) -> usize {
        self.words.iter().map(|word| word.count_ones() as usize).sum()
    }

    /// Returns saturation ratio between 0.0 (empty) and 1.0 (completely full).
    pub fn saturation(&self) -> f64 {
        self.count_ones() as f64 / self.capacity as f64
    }

    /// Get a reference to the internal word array.
    ///
    /// This can be useful for serialization or inspection.
    pub fn as_words(&self) -> &[u64] {
        &self.words
    }

    /// Create a BitArray from a vector of words and capacity.
    pub fn from_words(words: Vec<u64>, capacity: usize) -> Self {
        let required_words = (capacity + 63) / 64;
        assert!(
            words.len() >= required_words,
            "words vector too small for capacity"
        );

        Self { words, capacity }
    }
}
