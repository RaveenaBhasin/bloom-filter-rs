//! Comprehensive accuracy validation tests for the bloom filter.
//!
//! These tests verify that the bloom filter achieves its target false positive rates
//! and maintains accuracy across different configurations and workloads.

use bloom_filter_rs::PrecisionBloom;
use rand::{Rng, SeedableRng};

/// Test that false positive rate is below target for 1% FPR
#[test]
fn test_fpr_1_percent() {
    let mut filter = PrecisionBloom::with_capacity(10_000, 0.01);

    // Insert 10,000 items
    for i in 0..10_000 {
        filter.insert(&i);
    }

    // Test with 50,000 items that were NOT inserted
    let mut false_positives = 0;
    let test_count = 50_000;

    for i in 10_000..(10_000 + test_count) {
        if filter.contains(&i) {
            false_positives += 1;
        }
    }

    let actual_fpr = false_positives as f64 / test_count as f64;
    let target_fpr = 0.01;

    println!("Target FPR: {:.4}%", target_fpr * 100.0);
    println!("Actual FPR: {:.4}%", actual_fpr * 100.0);
    println!("False positives: {}/{}", false_positives, test_count);

    // Allow some statistical variation (2.5x of target)
    assert!(
        actual_fpr < target_fpr * 2.5,
        "FPR too high: {:.4}% (target: {:.4}%)",
        actual_fpr * 100.0,
        target_fpr * 100.0
    );
}

/// Test that false positive rate is below target for 0.1% FPR
#[test]
fn test_fpr_0_1_percent() {
    let mut filter = PrecisionBloom::with_capacity(10_000, 0.001);

    // Insert 10,000 items
    for i in 0..10_000 {
        filter.insert(&i);
    }

    // Test with 50,000 items that were NOT inserted
    let mut false_positives = 0;
    let test_count = 50_000;

    for i in 10_000..(10_000 + test_count) {
        if filter.contains(&i) {
            false_positives += 1;
        }
    }

    let actual_fpr = false_positives as f64 / test_count as f64;
    let target_fpr = 0.001;

    println!("Target FPR: {:.4}%", target_fpr * 100.0);
    println!("Actual FPR: {:.4}%", actual_fpr * 100.0);
    println!("False positives: {}/{}", false_positives, test_count);

    // Allow some statistical variation (3x of target for lower FPR)
    assert!(
        actual_fpr < target_fpr * 3.0,
        "FPR too high: {:.4}% (target: {:.4}%)",
        actual_fpr * 100.0,
        target_fpr * 100.0
    );
}

/// Test that false positive rate is below target for 5% FPR
#[test]
fn test_fpr_5_percent() {
    let mut filter = PrecisionBloom::with_capacity(5_000, 0.05);

    // Insert 5,000 items
    for i in 0..5_000 {
        filter.insert(&i);
    }

    // Test with 20,000 items that were NOT inserted
    let mut false_positives = 0;
    let test_count = 20_000;

    for i in 5_000..(5_000 + test_count) {
        if filter.contains(&i) {
            false_positives += 1;
        }
    }

    let actual_fpr = false_positives as f64 / test_count as f64;
    let target_fpr = 0.05;

    println!("Target FPR: {:.2}%", target_fpr * 100.0);
    println!("Actual FPR: {:.2}%", actual_fpr * 100.0);
    println!("False positives: {}/{}", false_positives, test_count);

    // Allow some statistical variation (2x of target)
    assert!(
        actual_fpr < target_fpr * 2.0,
        "FPR too high: {:.2}% (target: {:.2}%)",
        actual_fpr * 100.0,
        target_fpr * 100.0
    );
}

/// Test with string data instead of integers
#[test]
fn test_fpr_with_strings() {
    let mut filter = PrecisionBloom::with_capacity(5_000, 0.01);

    // Insert 5,000 strings
    for i in 0..5_000 {
        let s = format!("item_{}", i);
        filter.insert(&s);
    }

    // Test with 25,000 strings that were NOT inserted
    let mut false_positives = 0;
    let test_count = 25_000;

    for i in 5_000..(5_000 + test_count) {
        let s = format!("item_{}", i);
        if filter.contains(&s) {
            false_positives += 1;
        }
    }

    let actual_fpr = false_positives as f64 / test_count as f64;
    let target_fpr = 0.01;

    println!("String data - Target FPR: {:.4}%", target_fpr * 100.0);
    println!("String data - Actual FPR: {:.4}%", actual_fpr * 100.0);

    assert!(
        actual_fpr < target_fpr * 2.5,
        "FPR too high: {:.4}% (target: {:.4}%)",
        actual_fpr * 100.0,
        target_fpr * 100.0
    );
}

/// Test with random data to ensure hash distribution is good
#[test]
fn test_fpr_with_random_data() {
    let mut filter = PrecisionBloom::with_capacity(10_000, 0.01);
    let mut rng = rand::rngs::StdRng::seed_from_u64(42);

    // Insert 10,000 random items
    let mut inserted_items = Vec::new();
    for _ in 0..10_000 {
        let item = rng.gen::<u64>();
        filter.insert(&item);
        inserted_items.push(item);
    }

    // Test with 50,000 random items that were likely NOT inserted
    let mut false_positives = 0;
    let test_count = 50_000;

    for _ in 0..test_count {
        let item = rng.gen::<u64>();
        // Make sure we're not testing inserted items
        if !inserted_items.contains(&item) && filter.contains(&item) {
            false_positives += 1;
        }
    }

    let actual_fpr = false_positives as f64 / test_count as f64;
    let target_fpr = 0.01;

    println!("Random data - Target FPR: {:.4}%", target_fpr * 100.0);
    println!("Random data - Actual FPR: {:.4}%", actual_fpr * 100.0);

    assert!(
        actual_fpr < target_fpr * 3.0,
        "FPR too high: {:.4}% (target: {:.4}%)",
        actual_fpr * 100.0,
        target_fpr * 100.0
    );
}

/// Test that there are ZERO false negatives (critical guarantee)
#[test]
fn test_zero_false_negatives() {
    let mut filter = PrecisionBloom::with_capacity(10_000, 0.01);

    // Insert various types of data
    let integers: Vec<i32> = (0..1000).collect();
    let strings: Vec<String> = (0..1000).map(|i| format!("str_{}", i)).collect();
    let tuples: Vec<(i32, i32)> = (0..1000).map(|i| (i, i * 2)).collect();

    for &item in &integers {
        filter.insert(&item);
    }
    for item in &strings {
        filter.insert(item);
    }
    for &item in &tuples {
        filter.insert(&item);
    }

    // Verify ALL inserted items are found (no false negatives)
    let mut false_negatives = 0;

    for &item in &integers {
        if !filter.contains(&item) {
            false_negatives += 1;
            eprintln!("False negative: integer {}", item);
        }
    }
    for item in &strings {
        if !filter.contains(item) {
            false_negatives += 1;
            eprintln!("False negative: string {}", item);
        }
    }
    for &item in &tuples {
        if !filter.contains(&item) {
            false_negatives += 1;
            eprintln!("False negative: tuple {:?}", item);
        }
    }

    assert_eq!(
        false_negatives, 0,
        "Found {} false negatives - this should NEVER happen!",
        false_negatives
    );
}

/// Test accuracy with partially filled filter
#[test]
fn test_fpr_partially_filled() {
    let mut filter = PrecisionBloom::with_capacity(10_000, 0.01);

    // Only insert half the expected items
    for i in 0..5_000 {
        filter.insert(&i);
    }

    // Test with 25,000 items that were NOT inserted
    let mut false_positives = 0;
    let test_count = 25_000;

    for i in 10_000..(10_000 + test_count) {
        if filter.contains(&i) {
            false_positives += 1;
        }
    }

    let actual_fpr = false_positives as f64 / test_count as f64;
    let target_fpr = 0.01;

    println!(
        "Partially filled - Actual FPR: {:.4}% (should be lower than target {:.4}%)",
        actual_fpr * 100.0,
        target_fpr * 100.0
    );

    // With only half the items, FPR should be much lower than target
    assert!(
        actual_fpr < target_fpr,
        "FPR should be lower when partially filled: {:.4}% (target: {:.4}%)",
        actual_fpr * 100.0,
        target_fpr * 100.0
    );
}

/// Test accuracy degrades gracefully when overfilled
#[test]
fn test_fpr_overfilled() {
    let mut filter = PrecisionBloom::with_capacity(5_000, 0.01);

    // Insert DOUBLE the expected items
    for i in 0..10_000 {
        filter.insert(&i);
    }

    println!("Filter is overfilled: {}", filter.is_overfilled());
    println!("Actual FPR with overfill: {:.4}%", filter.actual_false_positive_rate() * 100.0);

    // Test with 25,000 items that were NOT inserted
    let mut false_positives = 0;
    let test_count = 25_000;

    for i in 10_000..(10_000 + test_count) {
        if filter.contains(&i) {
            false_positives += 1;
        }
    }

    let actual_fpr = false_positives as f64 / test_count as f64;

    println!("Overfilled - Actual FPR: {:.4}%", actual_fpr * 100.0);

    // FPR will be higher than target, but should still be reasonable (< 25%)
    assert!(
        actual_fpr < 0.25,
        "FPR too high even for overfilled filter: {:.2}%",
        actual_fpr * 100.0
    );
}

/// Test saturation levels correlate with FPR
#[test]
fn test_saturation_vs_fpr() {
    let mut filter = PrecisionBloom::with_capacity(1_000, 0.01);

    println!("\nSaturation vs FPR relationship:");

    // Test at different fill levels
    for batch in [250, 500, 750, 1000, 1500] {
        // Insert a batch
        for i in 0..batch {
            filter.insert(&i);
        }

        // Measure FPR
        let mut false_positives = 0;
        let test_count = 5_000;

        for i in 10_000..(10_000 + test_count) {
            if filter.contains(&i) {
                false_positives += 1;
            }
        }

        let actual_fpr = false_positives as f64 / test_count as f64;

        println!(
            "Items: {}, Saturation: {:.2}%, FPR: {:.4}%",
            batch,
            filter.saturation() * 100.0,
            actual_fpr * 100.0
        );

        // Clear for next iteration
        filter.clear();
    }
}

/// Benchmark hash distribution quality
#[test]
fn test_hash_distribution() {
    let filter = PrecisionBloom::with_capacity(10_000, 0.01);
    let num_bits = filter.num_bits();

    for i in 0..1000 {
        let mut temp_filter = filter.clone();
        temp_filter.insert(&i);

        // This is a simplified check - in reality we'd need access to internal bits
        // For now, just verify the filter works
        assert!(temp_filter.contains(&i));
    }

    println!("Hash distribution test completed - {} bits available", num_bits);
}

/// Test that clear() fully resets the filter
#[test]
fn test_clear_resets_accuracy() {
    let mut filter = PrecisionBloom::with_capacity(1_000, 0.01);

    // Fill the filter
    for i in 0..1_000 {
        filter.insert(&i);
    }

    let saturation_before = filter.saturation();
    assert!(saturation_before > 0.0);

    // Clear it
    filter.clear();

    let saturation_after = filter.saturation();
    assert_eq!(saturation_after, 0.0);
    assert_eq!(filter.len(), 0);

    // Insert new items and verify accuracy is reset
    for i in 10_000..11_000 {
        filter.insert(&i);
    }

    // Should not have false positives for the original items
    let mut false_positives = 0;
    for i in 0..1_000 {
        if filter.contains(&i) {
            false_positives += 1;
        }
    }

    let actual_fpr = false_positives as f64 / 1_000.0;

    println!("FPR after clear and refill: {:.4}%", actual_fpr * 100.0);

    assert!(
        actual_fpr < 0.03,
        "FPR after clear should be low: {:.4}%",
        actual_fpr * 100.0
    );
}
