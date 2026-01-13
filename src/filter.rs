//! Core bloom filter implementation.

use std::hash::Hash;

use crate::accuracy::AccuracyTracker;
use crate::bit_array::BitArray;
use crate::hash::HashStrategy;
use crate::params::BloomParameters;

/// A precision bloom filter optimized for accuracy.
///
/// This bloom filter uses standard Kirsch-Mitzenmacher double hashing with two independent
/// hash functions (ahash and seahash) for excellent hash distribution and minimal false positive rates.
#[derive(Debug, Clone)]
pub struct PrecisionBloom {
    /// Bit array storing the filter state
    bits: BitArray,
    /// Hash strategy for generating indices
    hash_strategy: HashStrategy,
    /// Parameters of this filter
    params: BloomParameters,
    /// Accuracy tracking
    tracker: AccuracyTracker,
}

impl PrecisionBloom {
    /// Create a new bloom filter with specified parameters.
    ///
    /// # Arguments
    /// * `params` - The bloom filter parameters
    pub fn new(params: BloomParameters) -> Self {
        params.validate().expect("Invalid parameters");

        let bits = BitArray::new(params.num_bits);
        let hash_strategy = HashStrategy::new(params.num_hashes, params.num_bits);
        let tracker = AccuracyTracker::new(params);

        Self {
            bits,
            hash_strategy,
            params,
            tracker,
        }
    }

    /// Create a new bloom filter for a given number of items and false positive rate.
    ///
    /// This is the recommended constructor for most use cases.
    ///
    /// # Arguments
    /// * `expected_items` - Number of items expected to be inserted
    /// * `false_positive_rate` - Desired false positive rate (between 0 and 1)
    ///
    /// # Example
    /// ```
    /// use bloom_filter_rs::PrecisionBloom;
    ///
    /// // Create a filter for 10,000 items with 1% false positive rate
    /// let filter = PrecisionBloom::with_capacity(10_000, 0.01);
    /// ```
    pub fn with_capacity(expected_items: usize, false_positive_rate: f64) -> Self {
        let params = BloomParameters::from_item_count(expected_items, false_positive_rate);
        Self::new(params)
    }

    /// Insert an item into the bloom filter.
    ///
    /// # Arguments
    /// * `item` - The item to insert
    ///
    /// # Returns
    /// Returns `true` if the item was definitely not in the filter before,
    /// `false` if it might have been (could be a false positive).
    ///
    /// # Example
    /// ```
    /// use bloom_filter_rs::PrecisionBloom;
    ///
    /// let mut filter = PrecisionBloom::with_capacity(100, 0.01);
    /// filter.insert(&"hello");
    /// filter.insert(&42);
    /// ```
    pub fn insert<T: Hash>(&mut self, item: &T) -> bool {
        self.tracker.record_insert();

        let indices = self.hash_strategy.hash_indices(item);
        let mut was_absent = false;

        for &index in &indices {
            if !self.bits.get(index) {
                was_absent = true;
                self.bits.set(index);
            }
        }

        was_absent
    }

    /// Check if an item might be in the bloom filter.
    ///
    /// # Arguments
    /// * `item` - The item to check
    ///
    /// # Returns
    /// * `true` - Item might be in the set (or false positive)
    /// * `false` - Item is definitely not in the set
    ///
    /// # Example
    /// ```
    /// use bloom_filter_rs::PrecisionBloom;
    ///
    /// let mut filter = PrecisionBloom::with_capacity(100, 0.01);
    /// filter.insert(&"hello");
    ///
    /// assert!(filter.contains(&"hello"));  // Definitely inserted
    /// assert!(!filter.contains(&"world")); // Never inserted
    /// ```
    pub fn contains<T: Hash>(&self, item: &T) -> bool {
        let indices = self.hash_strategy.hash_indices(item);

        // Item is present only if ALL hash positions are set
        indices.iter().all(|&index| self.bits.get(index))
    }

    /// Check if an item might be in the bloom filter (alias for contains).
    ///
    /// This method is provided for clarity in some contexts.
    #[inline]
    pub fn may_contain<T: Hash>(&self, item: &T) -> bool {
        self.contains(item)
    }

    /// Clear all items from the filter.
    ///
    /// Resets the filter to its initial empty state.
    pub fn clear(&mut self) {
        self.bits.clear();
        self.tracker.reset();
    }

    /// Get the number of items inserted into the filter.
    ///
    /// Note: This is tracked by the filter, not guaranteed to be exact
    /// if the same item is inserted multiple times.
    pub fn len(&self) -> usize {
        self.tracker.items_inserted()
    }

    /// Check if the filter is empty (no items inserted).
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get the capacity (expected number of items) of the filter.
    pub fn capacity(&self) -> usize {
        self.params.expected_items
    }

    /// Get the number of bits in the filter.
    pub fn num_bits(&self) -> usize {
        self.params.num_bits
    }

    /// Get the number of hash functions used.
    pub fn num_hashes(&self) -> usize {
        self.params.num_hashes
    }

    /// Get the theoretical false positive rate.
    pub fn false_positive_rate(&self) -> f64 {
        self.tracker.theoretical_fpr()
    }

    /// Get the actual false positive rate based on current fill level.
    pub fn actual_false_positive_rate(&self) -> f64 {
        self.tracker.actual_fpr()
    }

    /// Get the saturation level of the filter (proportion of bits set).
    ///
    /// Returns a value between 0.0 (empty) and 1.0 (completely full).
    pub fn saturation(&self) -> f64 {
        self.bits.saturation()
    }

    /// Check if the filter has exceeded its expected capacity.
    pub fn is_overfilled(&self) -> bool {
        self.tracker.is_overfilled()
    }

    /// Get a status summary of the filter.
    pub fn status(&self) -> String {
        self.tracker.status_summary()
    }

    /// Get the parameters of this filter.
    pub fn parameters(&self) -> &BloomParameters {
        &self.params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_filter() {
        let filter = PrecisionBloom::with_capacity(1000, 0.01);
        assert_eq!(filter.len(), 0);
        assert!(filter.is_empty());
        assert_eq!(filter.capacity(), 1000);
        assert!(filter.num_bits() > 9000 && filter.num_bits() < 10000);
        assert!(filter.num_hashes() >= 6 && filter.num_hashes() <= 8);
    }

    #[test]
    fn test_insert_and_contains() {
        let mut filter = PrecisionBloom::with_capacity(100, 0.01);

        // Insert some items
        filter.insert(&"hello");
        filter.insert(&"world");
        filter.insert(&42);

        // Check they are present
        assert!(filter.contains(&"hello"));
        assert!(filter.contains(&"world"));
        assert!(filter.contains(&42));

        // Check other items are not present
        assert!(!filter.contains(&"foo"));
        assert!(!filter.contains(&"bar"));
        assert!(!filter.contains(&99));
    }

    #[test]
    fn test_insert_returns_correct_value() {
        let mut filter = PrecisionBloom::with_capacity(100, 0.01);

        // First insert should return true (was absent)
        assert!(filter.insert(&"hello"));

        // Second insert might return false (might already be set)
        // This depends on hash collisions, so we just check it doesn't panic
        let _ = filter.insert(&"hello");
    }

    #[test]
    fn test_clear() {
        let mut filter = PrecisionBloom::with_capacity(100, 0.01);

        filter.insert(&"hello");
        filter.insert(&"world");
        assert!(filter.contains(&"hello"));
        assert!(!filter.is_empty());

        filter.clear();
        assert!(!filter.contains(&"hello"));
        assert!(!filter.contains(&"world"));
        assert!(filter.is_empty());
        assert_eq!(filter.len(), 0);
    }

    #[test]
    fn test_len() {
        let mut filter = PrecisionBloom::with_capacity(100, 0.01);

        assert_eq!(filter.len(), 0);

        filter.insert(&1);
        assert_eq!(filter.len(), 1);

        filter.insert(&2);
        assert_eq!(filter.len(), 2);

        filter.insert(&3);
        assert_eq!(filter.len(), 3);
    }

    #[test]
    fn test_saturation_increases() {
        let mut filter = PrecisionBloom::with_capacity(100, 0.01);

        let initial_saturation = filter.saturation();
        assert_eq!(initial_saturation, 0.0);

        // Insert items and check saturation increases
        for i in 0..50 {
            filter.insert(&i);
        }

        let mid_saturation = filter.saturation();
        assert!(mid_saturation > 0.0);

        for i in 50..100 {
            filter.insert(&i);
        }

        let final_saturation = filter.saturation();
        assert!(final_saturation > mid_saturation);
    }

    #[test]
    fn test_no_false_negatives() {
        let mut filter = PrecisionBloom::with_capacity(1000, 0.01);

        let items: Vec<i32> = (0..1000).collect();

        // Insert all items
        for &item in &items {
            filter.insert(&item);
        }

        // Check all items are present (no false negatives)
        for &item in &items {
            assert!(
                filter.contains(&item),
                "False negative detected for item {}",
                item
            );
        }
    }

    #[test]
    fn test_false_positive_rate() {
        let mut filter = PrecisionBloom::with_capacity(1000, 0.01);

        // Insert 1000 items
        for i in 0..1000 {
            filter.insert(&i);
        }

        // Test 10000 items that were NOT inserted
        let mut false_positives = 0;
        let test_count = 10000;

        for i in 1000..(1000 + test_count) {
            if filter.contains(&i) {
                false_positives += 1;
            }
        }

        let actual_fpr = false_positives as f64 / test_count as f64;

        // Actual FPR should be close to target (within 3x for statistical variation)
        assert!(
            actual_fpr < 0.03,
            "False positive rate too high: {:.4}",
            actual_fpr
        );
    }

    #[test]
    fn test_is_overfilled() {
        let mut filter = PrecisionBloom::with_capacity(10, 0.01);

        assert!(!filter.is_overfilled());

        for i in 0..10 {
            filter.insert(&i);
        }
        assert!(!filter.is_overfilled());

        filter.insert(&11);
        assert!(filter.is_overfilled());
    }

    #[test]
    fn test_different_types() {
        let mut filter = PrecisionBloom::with_capacity(100, 0.01);

        // Test with different types
        filter.insert(&"string");
        filter.insert(&42);
        filter.insert(&(1, 2, 3));
        filter.insert(&vec![1, 2, 3]);

        assert!(filter.contains(&"string"));
        assert!(filter.contains(&42));
        assert!(filter.contains(&(1, 2, 3)));
        assert!(filter.contains(&vec![1, 2, 3]));
    }

    #[test]
    fn test_status_summary() {
        let mut filter = PrecisionBloom::with_capacity(100, 0.01);

        for i in 0..50 {
            filter.insert(&i);
        }

        let status = filter.status();
        assert!(status.contains("50/100"));
    }
}
