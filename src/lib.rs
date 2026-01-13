//! # Precision Bloom Filter
//!
//! A high-accuracy bloom filter implementation focused on correctness and minimal false positive rates.
//!
//! ## Features
//!
//! - **Standard Double Hashing**: Uses two independent hash functions (ahash and seahash) with
//!   Kirsch-Mitzenmacher double hashing for excellent hash distribution
//! - **Optimal Parameters**: Automatically calculates optimal bit count and hash functions
//!   based on desired false positive rate
//! - **Accuracy Tracking**: Built-in monitoring of actual vs theoretical false positive rates
//! - **Simple API**: Clean, intuitive interface with comprehensive documentation
//! - **No Unsafe Code**: Pure safe Rust implementation
//!
//! ## Quick Start
//!
//! ```
//! use bloom_filter_rs::PrecisionBloom;
//!
//! // Create a filter for 10,000 items with 1% false positive rate
//! let mut filter = PrecisionBloom::with_capacity(10_000, 0.01);
//!
//! // Insert some items
//! filter.insert(&"hello");
//! filter.insert(&"world");
//! filter.insert(&42);
//!
//! // Check for membership
//! assert!(filter.contains(&"hello"));  // true - definitely inserted
//! assert!(!filter.contains(&"foo"));   // false - never inserted
//! ```
//!
//! ## How It Works
//!
//! A bloom filter is a space-efficient probabilistic data structure that can test whether
//! an element is a member of a set. It guarantees:
//!
//! - **No false negatives**: If an item was inserted, `contains()` will return `true`
//! - **Possible false positives**: `contains()` might return `true` for items never inserted
//!
//! The false positive rate can be controlled by adjusting the size of the filter and the
//! number of hash functions used.
//!
//! ## Accuracy
//!
//! This implementation prioritizes accuracy through:
//!
//! 1. **True Independent Hashing**: Uses two completely different hash algorithms (ahash and seahash)
//!    rather than deriving multiple hashes from a single algorithm
//! 2. **Standard Double Hashing**: Uses proven Kirsch-Mitzenmacher double hashing (h1 + i*h2)
//!    for optimal distribution as used in production implementations
//! 3. **Optimal Parameters**: Mathematically calculates the best configuration for your requirements
//! 4. **Runtime Monitoring**: Tracks actual performance to help detect capacity issues
//!
//! ## Mathematical Background
//!
//! Given:
//! - `n` = number of items to insert
//! - `p` = desired false positive rate
//!
//! The optimal parameters are:
//! - Number of bits: `m = -n * ln(p) / (ln(2)²)`
//! - Number of hashes: `k = (m/n) * ln(2)`
//!
//! The actual false positive rate after inserting `n` items:
//! - `p = (1 - e^(-kn/m))^k`
//!
//! ## Examples
//!
//! ### Basic Usage
//!
//! ```
//! use bloom_filter_rs::PrecisionBloom;
//!
//! let mut filter = PrecisionBloom::with_capacity(1000, 0.01);
//!
//! // Insert items
//! for i in 0..1000 {
//!     filter.insert(&i);
//! }
//!
//! // Check membership
//! assert!(filter.contains(&42));
//! assert!(!filter.contains(&9999));
//!
//! // Check status
//! println!("{}", filter.status());
//! ```
//!
//! ### Monitoring Accuracy
//!
//! ```
//! use bloom_filter_rs::PrecisionBloom;
//!
//! let mut filter = PrecisionBloom::with_capacity(1000, 0.01);
//!
//! // Insert items
//! for i in 0..500 {
//!     filter.insert(&i);
//! }
//!
//! println!("Theoretical FPR: {:.4}%", filter.false_positive_rate() * 100.0);
//! println!("Actual FPR: {:.4}%", filter.actual_false_positive_rate() * 100.0);
//! println!("Saturation: {:.2}%", filter.saturation() * 100.0);
//! ```
//!
//! ### Custom Parameters
//!
//! ```
//! use bloom_filter_rs::{PrecisionBloom, BloomParameters};
//!
//! // Create filter with explicit bit count
//! let params = BloomParameters::from_bit_count(10000, 1000);
//! let filter = PrecisionBloom::new(params);
//!
//! println!("Using {} hash functions", filter.num_hashes());
//! ```

mod accuracy;
mod bit_array;
mod filter;
mod hash;
mod params;

pub use accuracy::AccuracyTracker;
pub use bit_array::BitArray;
pub use filter::PrecisionBloom;
pub use hash::HashStrategy;
pub use params::BloomParameters;
