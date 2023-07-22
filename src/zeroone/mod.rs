//! # 0-1 Knapsack
//! A variant of the knapsack problem.
//!
//! Given a list of items with associated weights and values, choose at most one of each given item.
//! Maximize the value while ensuring the weight less than or equal to a given capacity.
//!
//! Possible optimizations:
//! - Sliding (similar to [dpsliding](crate::dpsliding))
//! - Walkback
//!     - This is a bit more nuanced than sliding.
//!     - Sort the items from large to small to minimize the number of marked squares.
//!     - Use the existing markings and combine them with each item in the table to get the row markings.

pub mod dp;
pub mod recursive;
