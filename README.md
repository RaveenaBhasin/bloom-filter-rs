# Bloom Filter Implementation

A simple bloom filter implementation in Rust that focuses on accuracy and correctness.

A bloom filter is a space-efficient data structure that can tell you if something is *probably* in a set, or *definitely not* in a set. It never gives false negatives (if you inserted it, it will say it's there), but it might give false positives (might say something is there when it isn't).

Useful when you need fast membership tests and can tolerate the occasional false positive.


## Quick Start

```rust
use bloom_filter_rs::PrecisionBloom;

// Create a filter for 10,000 items with 1% false positive rate
let mut filter = PrecisionBloom::with_capacity(10_000, 0.01);

// Insert items
filter.insert(&"hello");
filter.insert(&42);

// Check membership
if filter.contains(&"hello") {
    // Might be there (or false positive)
}

if !filter.contains(&"world") {
    // Definitely not there
}
```

## How It Works

The filter uses two hash functions (ahash and seahash) to generate multiple hash values using double hashing. When you insert an item, it sets k bits in a bit array. When you check if something exists, it checks if all k bits are set.

The filter automatically calculates the optimal number of bits and hash functions based on your desired false positive rate and expected number of items.