//! Accuracy tracking for monitoring bloom filter performance.
//!
//! This module provides tools to track and monitor the actual false positive
//! rate of a bloom filter compared to its theoretical rate.

use crate::params::BloomParameters;

/// Tracks the accuracy and performance of a bloom filter.
#[derive(Debug, Clone)]
pub struct AccuracyTracker {
    /// Parameters of the bloom filter
    params: BloomParameters,
    /// Number of items actually inserted
    items_inserted: usize,
    /// Number of queries performed
    queries_performed: usize,
}

impl AccuracyTracker {
    /// Create a new accuracy tracker.
    pub fn new(params: BloomParameters) -> Self {
        Self {
            params,
            items_inserted: 0,
            queries_performed: 0,
        }
    }

    /// Record an insertion.
    pub fn record_insert(&mut self) {
        self.items_inserted += 1;
    }

    /// Record a query operation.
    pub fn record_query(&mut self) {
        self.queries_performed += 1;
    }

    /// Get the number of items inserted.
    pub fn items_inserted(&self) -> usize {
        self.items_inserted
    }

    /// Get the number of queries performed.
    pub fn queries_performed(&self) -> usize {
        self.queries_performed
    }

    /// Get the theoretical false positive rate based on parameters.
    pub fn theoretical_fpr(&self) -> f64 {
        self.params.false_positive_rate
    }

    /// Get the actual false positive rate based on items inserted.
    ///
    /// This recalculates the FPR using the actual number of items
    /// inserted, which may differ from expected_items.
    pub fn actual_fpr(&self) -> f64 {
        if self.items_inserted == 0 {
            return 0.0;
        }
        self.params.actual_fpr(self.items_inserted)
    }

    /// Check if the filter is overfilled.
    ///
    /// Returns true if more items have been inserted than expected.
    pub fn is_overfilled(&self) -> bool {
        self.items_inserted > self.params.expected_items
    }

    /// Get the fill ratio (actual items / expected items).
    pub fn fill_ratio(&self) -> f64 {
        if self.params.expected_items == 0 {
            return 0.0;
        }
        self.items_inserted as f64 / self.params.expected_items as f64
    }

    /// Get the overfill amount (how many items over capacity).
    ///
    /// Returns 0 if not overfilled.
    pub fn overfill_amount(&self) -> usize {
        if self.items_inserted > self.params.expected_items {
            self.items_inserted - self.params.expected_items
        } else {
            0
        }
    }

    /// Get a status summary as a string.
    pub fn status_summary(&self) -> String {
        format!(
            "Inserted: {}/{} items ({:.1}% full), Queries: {}, Theoretical FPR: {:.4}%, Actual FPR: {:.4}%",
            self.items_inserted,
            self.params.expected_items,
            self.fill_ratio() * 100.0,
            self.queries_performed,
            self.theoretical_fpr() * 100.0,
            self.actual_fpr() * 100.0
        )
    }

    /// Reset the tracker (useful for reusing a filter).
    pub fn reset(&mut self) {
        self.items_inserted = 0;
        self.queries_performed = 0;
    }
}
