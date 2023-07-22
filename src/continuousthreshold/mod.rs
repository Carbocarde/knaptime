//! # Continuous Category Knapsack
//! A variant of the knapsack problem.
//!
//! Normally you need to have fixed precision in order to calculate the optimal solution. Floating point numbers have too much precision.
//! You might be able to get away with using dpwalkback, but not when there's a lot of items.
//!
//! An alternative approach is to use probability to find a solution that has some % chance of fitting/being the optimal solution.
//! There's a percentage chance that it'll be the optimal solution/undershoot/overshoot, but the distribution should be centered around the optimal solution.
//!
//! TODO: Do the actual math/experimentally verify this
//!
//! Given a list of items  with associated weights and values.
//! Maximize the value while ensuring the weight is minimal (weight must be greater than or equal to a given threshold).
