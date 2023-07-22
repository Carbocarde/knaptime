//! # Threshold Knapsack
//! A variant of the knapsack problem.
//!
//! Given a list of items  with associated weights and values.
//! Maximize the value while ensuring the weight is minimal (weight must be greater than or equal to a given threshold).
//!
//! Possible optimizations:
//! - Sliding (similar to [dpsliding](crate::dpsliding))

pub mod dp;
pub mod recursive;
