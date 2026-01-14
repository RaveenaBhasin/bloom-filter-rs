//! Correctness tests for the bloom filter implementation.
//!
//! These tests verify the fundamental correctness guarantees of the bloom filter.

use bloom_filter_rs::PrecisionBloom;

/// Test basic insert and contains operations
#[test]
fn test_basic_operations() {
    let mut filter = PrecisionBloom::with_capacity(100, 0.01);

    // Initially empty
    assert!(filter.is_empty());
    assert_eq!(filter.len(), 0);

    // Insert an item
    filter.insert(&"hello");
    assert!(!filter.is_empty());
    assert_eq!(filter.len(), 1);
    assert!(filter.contains(&"hello"));

    // Insert more items
    filter.insert(&"world");
    filter.insert(&42);
    assert_eq!(filter.len(), 3);
    assert!(filter.contains(&"world"));
    assert!(filter.contains(&42));
}

/// Test that inserted items are always found (no false negatives)
#[test]
fn test_no_false_negatives_comprehensive() {
    let mut filter = PrecisionBloom::with_capacity(5_000, 0.01);

    // Insert a large number of diverse items
    for i in 0..5_000 {
        filter.insert(&i);
    }

    // Verify every single item is found
    for i in 0..5_000 {
        assert!(
            filter.contains(&i),
            "False negative detected for item {}",
            i
        );
    }
}

/// Test clear operation completely resets the filter
#[test]
fn test_clear_comprehensive() {
    let mut filter = PrecisionBloom::with_capacity(1_000, 0.01);

    // Insert many items
    for i in 0..1_000 {
        filter.insert(&i);
    }

    // Verify items are present
    assert!(filter.contains(&500));
    assert_eq!(filter.len(), 1_000);

    // Clear
    filter.clear();

    // Verify filter is completely reset
    assert!(filter.is_empty());
    assert_eq!(filter.len(), 0);
    assert_eq!(filter.saturation(), 0.0);

    // Previously inserted items should no longer be found
    // (or only found with very low probability)
    let mut found_after_clear = 0;
    for i in 0..1_000 {
        if filter.contains(&i) {
            found_after_clear += 1;
        }
    }

    assert_eq!(
        found_after_clear, 0,
        "Found {} items after clear, expected 0",
        found_after_clear
    );
}

/// Test that duplicate insertions work correctly
#[test]
fn test_duplicate_insertions() {
    let mut filter = PrecisionBloom::with_capacity(100, 0.01);

    // Insert same item multiple times
    filter.insert(&"duplicate");
    filter.insert(&"duplicate");
    filter.insert(&"duplicate");

    // Length tracks insertions (even duplicates)
    assert_eq!(filter.len(), 3);

    // Item should still be found
    assert!(filter.contains(&"duplicate"));
}

/// Test with various data types
#[test]
fn test_various_types() {
    let mut filter = PrecisionBloom::with_capacity(100, 0.01);

    // Integers
    filter.insert(&42);
    filter.insert(&-17);
    assert!(filter.contains(&42));
    assert!(filter.contains(&-17));

    // Strings
    filter.insert(&"hello");
    filter.insert(&String::from("world"));
    assert!(filter.contains(&"hello"));
    assert!(filter.contains(&String::from("world")));

    // Tuples
    filter.insert(&(1, 2));
    filter.insert(&(3, 4, 5));
    assert!(filter.contains(&(1, 2)));
    assert!(filter.contains(&(3, 4, 5)));

    // Vectors
    filter.insert(&vec![1, 2, 3]);
    assert!(filter.contains(&vec![1, 2, 3]));

    // Floats (be careful with floating point comparisons)
    let float_val: f64 = 3.14;
    filter.insert(&float_val.to_bits());
    assert!(filter.contains(&float_val.to_bits()));
}

/// Test edge case: small filter
#[test]
fn test_small_filter() {
    // Use slightly larger capacity to avoid high collision probability
    let mut filter = PrecisionBloom::with_capacity(10, 0.01);

    filter.insert(&"only");
    assert!(filter.contains(&"only"));

    // With a small filter, we can't guarantee no false positives
    // Just verify the inserted item is always found
}

/// Test edge case: very large capacity
#[test]
fn test_large_capacity() {
    let filter = PrecisionBloom::with_capacity(1_000_000, 0.001);

    assert_eq!(filter.capacity(), 1_000_000);
    assert!(filter.num_bits() > 10_000_000); // Should allocate enough bits
    assert!(filter.num_hashes() >= 10); // Should use enough hash functions
}

/// Test that different items (usually) have different hashes
#[test]
fn test_different_items_likely_different() {
    let mut filter = PrecisionBloom::with_capacity(10, 0.01);

    // Insert one item
    filter.insert(&"item1");

    // Different item should (very likely) not be found
    assert!(!filter.contains(&"item2"));
    assert!(!filter.contains(&"item3"));
}

/// Test boundary values
#[test]
fn test_boundary_values() {
    let mut filter = PrecisionBloom::with_capacity(100, 0.01);

    // Test with extreme values
    filter.insert(&i64::MIN);
    filter.insert(&i64::MAX);
    filter.insert(&0i64);

    assert!(filter.contains(&i64::MIN));
    assert!(filter.contains(&i64::MAX));
    assert!(filter.contains(&0i64));

    // Test with unsigned extremes
    filter.insert(&u64::MIN);
    filter.insert(&u64::MAX);

    assert!(filter.contains(&u64::MIN));
    assert!(filter.contains(&u64::MAX));
}

/// Test empty string and special characters
#[test]
fn test_special_strings() {
    let mut filter = PrecisionBloom::with_capacity(100, 0.01);

    filter.insert(&"");
    filter.insert(&" ");
    filter.insert(&"  ");
    filter.insert(&"\n");
    filter.insert(&"\t");
    filter.insert(&"🦀");
    filter.insert(&"Hello\nWorld");

    assert!(filter.contains(&""));
    assert!(filter.contains(&" "));
    assert!(filter.contains(&"  "));
    assert!(filter.contains(&"\n"));
    assert!(filter.contains(&"\t"));
    assert!(filter.contains(&"🦀"));
    assert!(filter.contains(&"Hello\nWorld"));

    // Different whitespace should not match
    assert!(!filter.contains(&"   ")); // 3 spaces vs 2
}

/// Test that parameters are correctly reported
#[test]
fn test_parameter_reporting() {
    let filter = PrecisionBloom::with_capacity(1000, 0.01);

    assert_eq!(filter.capacity(), 1000);
    assert!(filter.false_positive_rate() <= 0.01);
    assert!(filter.num_bits() > 0);
    assert!(filter.num_hashes() > 0);

    // For 1000 items and 1% FPR, should use approximately:
    // m ≈ 9585 bits, k ≈ 7 hashes
    assert!(filter.num_bits() > 9000 && filter.num_bits() < 11000);
    assert!(filter.num_hashes() >= 6 && filter.num_hashes() <= 8);
}

/// Test saturation increases as items are added
#[test]
fn test_saturation_monotonic() {
    let mut filter = PrecisionBloom::with_capacity(100, 0.01);

    let mut previous_saturation = 0.0;

    for i in 0..50 {
        filter.insert(&i);
        let current_saturation = filter.saturation();

        // Saturation should increase (or stay same if bits already set)
        assert!(
            current_saturation >= previous_saturation,
            "Saturation decreased: {} -> {}",
            previous_saturation,
            current_saturation
        );

        previous_saturation = current_saturation;
    }

    // After 50 items, saturation should be > 0
    assert!(previous_saturation > 0.0);
}

/// Test clone creates independent copy
#[test]
fn test_clone_independence() {
    let mut filter1 = PrecisionBloom::with_capacity(100, 0.01);

    filter1.insert(&"original");

    let mut filter2 = filter1.clone();

    // Both should contain the original item
    assert!(filter1.contains(&"original"));
    assert!(filter2.contains(&"original"));

    // Insert into filter2 only
    filter2.insert(&"clone_only");

    // filter2 should have both items
    assert!(filter2.contains(&"original"));
    assert!(filter2.contains(&"clone_only"));

    // filter1 should NOT have the new item
    assert!(filter1.contains(&"original"));
    assert!(!filter1.contains(&"clone_only"));
}

/// Test may_contain alias
#[test]
fn test_may_contain_alias() {
    let mut filter = PrecisionBloom::with_capacity(100, 0.01);

    filter.insert(&"test");

    // Both methods should return same result
    assert_eq!(filter.contains(&"test"), filter.may_contain(&"test"));
    assert_eq!(filter.contains(&"other"), filter.may_contain(&"other"));
}

/// Test status reporting
#[test]
fn test_status() {
    let mut filter = PrecisionBloom::with_capacity(100, 0.01);

    for i in 0..50 {
        filter.insert(&i);
    }

    let status = filter.status();

    // Status should contain useful information
    assert!(status.contains("50"));
    assert!(status.contains("100"));
    assert!(status.contains("%"));
}

/// Test overfill detection
#[test]
fn test_overfill_detection() {
    let mut filter = PrecisionBloom::with_capacity(10, 0.01);

    // Fill to capacity
    for i in 0..10 {
        filter.insert(&i);
        if i < 10 {
            assert!(!filter.is_overfilled());
        }
    }

    // Should not be overfilled yet
    assert!(!filter.is_overfilled());

    // Add one more
    filter.insert(&100);

    // Now should be overfilled
    assert!(filter.is_overfilled());
}
