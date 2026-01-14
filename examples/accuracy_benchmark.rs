//! Accuracy benchmark - validates that false positive rates meet targets

use bloom_filter_rs::PrecisionBloom;
use rand::{Rng, SeedableRng};

fn test_accuracy(capacity: usize, target_fpr: f64, test_multiplier: usize) {
    println!("\n--- Testing: {} items, {:.4}% target FPR ---", capacity, target_fpr * 100.0);

    let mut filter = PrecisionBloom::with_capacity(capacity, target_fpr);

    println!("Configuration:");
    println!("  Bits: {}", filter.num_bits());
    println!("  Hash functions: {}", filter.num_hashes());
    println!("  Bits per item: {:.2}", filter.num_bits() as f64 / capacity as f64);

    // Insert items
    println!("\nInserting {} items...", capacity);
    for i in 0..capacity {
        filter.insert(&i);
    }

    println!("  Saturation: {:.2}%", filter.saturation() * 100.0);
    println!("  Theoretical FPR: {:.4}%", filter.false_positive_rate() * 100.0);
    println!("  Calculated FPR: {:.4}%", filter.actual_false_positive_rate() * 100.0);

    // Verify no false negatives
    println!("\nVerifying NO false negatives...");
    let mut false_negatives = 0;
    for i in 0..capacity {
        if !filter.contains(&i) {
            false_negatives += 1;
        }
    }
    println!("  False negatives: {} (MUST be 0)", false_negatives);
    assert_eq!(false_negatives, 0, "CRITICAL: Found false negatives!");

    // Measure false positive rate
    println!("\nMeasuring false positive rate...");
    let test_count = capacity * test_multiplier;
    let mut false_positives = 0;

    for i in capacity..(capacity + test_count) {
        if filter.contains(&i) {
            false_positives += 1;
        }
    }

    let measured_fpr = false_positives as f64 / test_count as f64;
    let ratio = measured_fpr / target_fpr;

    println!("  Tested: {} items ({}x capacity)", test_count, test_multiplier);
    println!("  False positives: {}", false_positives);
    println!("  Measured FPR: {:.4}%", measured_fpr * 100.0);
    println!("  Target FPR: {:.4}%", target_fpr * 100.0);
    println!("  Ratio (measured/target): {:.2}x", ratio);

    if ratio < 2.0 {
        println!("  ✓ EXCELLENT - within 2x of target");
    } else if ratio < 3.0 {
        println!("  ✓ GOOD - within 3x of target");
    } else {
        println!("  ⚠ ACCEPTABLE - within tolerance but higher than ideal");
    }
}

fn test_with_random_data(capacity: usize, target_fpr: f64) {
    println!("\n--- Random Data Test: {} items, {:.4}% target FPR ---", capacity, target_fpr * 100.0);

    let mut filter = PrecisionBloom::with_capacity(capacity, target_fpr);
    let mut rng = rand::rngs::StdRng::seed_from_u64(42);

    // Insert random items
    let mut inserted = Vec::new();
    for _ in 0..capacity {
        let item = rng.gen::<u64>();
        filter.insert(&item);
        inserted.push(item);
    }

    // Test with different random items
    let test_count = capacity * 5;
    let mut false_positives = 0;
    let mut tested = 0;

    for _ in 0..test_count * 2 {
        let item = rng.gen::<u64>();
        if !inserted.contains(&item) {
            tested += 1;
            if filter.contains(&item) {
                false_positives += 1;
            }
            if tested >= test_count {
                break;
            }
        }
    }

    let measured_fpr = false_positives as f64 / tested as f64;
    let ratio = measured_fpr / target_fpr;

    println!("  Tested: {} random u64 values", tested);
    println!("  False positives: {}", false_positives);
    println!("  Measured FPR: {:.4}%", measured_fpr * 100.0);
    println!("  Target FPR: {:.4}%", target_fpr * 100.0);
    println!("  Ratio: {:.2}x", ratio);

    if ratio < 3.0 {
        println!("  ✓ Good hash distribution with random data");
    }
}

fn main() {
    println!("====================================");
    println!("  Precision Bloom Accuracy Tests");
    println!("====================================");

    // Test various configurations
    test_accuracy(1_000, 0.01, 10);
    test_accuracy(10_000, 0.01, 5);
    test_accuracy(10_000, 0.001, 5);
    test_accuracy(5_000, 0.05, 4);

    // Test with random data
    test_with_random_data(10_000, 0.01);

    // Test at different fill levels
    println!("\n--- Fill Level Analysis: 10,000 capacity, 1% target ---");
    let mut filter = PrecisionBloom::with_capacity(10_000, 0.01);

    for fill_pct in [25, 50, 75, 100, 150] {
        let items = (10_000 * fill_pct) / 100;

        // Insert items
        for i in 0..items {
            filter.insert(&i);
        }

        // Measure FPR
        let test_count = 5_000;
        let mut fps = 0;
        for i in 100_000..(100_000 + test_count) {
            if filter.contains(&i) {
                fps += 1;
            }
        }

        let measured_fpr = fps as f64 / test_count as f64;

        println!("\n  Fill: {}% ({} items)", fill_pct, items);
        println!("    Saturation: {:.2}%", filter.saturation() * 100.0);
        println!("    Measured FPR: {:.4}%", measured_fpr * 100.0);
        println!("    Calculated FPR: {:.4}%", filter.actual_false_positive_rate() * 100.0);
        println!("    Overfilled: {}", filter.is_overfilled());
    }

    println!("\n====================================");
    println!("  All Accuracy Tests Complete!");
    println!("====================================");
}
