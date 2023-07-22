//! # Knaptime Crate
//! This crate aims to solve variants of the knapsack problem.
//!
//! It was constructed due to the lack of easily-accessible implementations of knapsack variant algorithms online.
//! Language bindings are planned to allow these functions to be used in your language of choice.
//!
//! # Variants
//! Variants         | Regular Knapsack | 01          | Threshold           | Category             | Continuous
//! -----------------|------------------|-------------|---------------------|----------------------|---------------------
//! Regular Knapsack | [dp] [recursive] | Invalid[^1] | Invalid[^1]         | [categoryrepeat]!    | [continuous]
//! 01               | x                | [zeroone]   | [zeroonethreshold]! | [category]           | [continuouszeroone]!
//! Threshold        | x                | x           | [threshold]         | [categorythreshold]! | [continuousthreshold]!
//! Category         | x                | x           | x                   | [category]           | [continuouscategory]!
//! Continuous       | x                | x           | x                   | x                    | [continuous]
//!
//! Items with a ! are not implemented (yet!). Feel free to open a PR or request help via GitHub issues.
//!
//! [^1]: This isn't compatible with regular knapsack since it's a direct variant. Feel free to open a GitHub issue if you have an idea of how to implement an invalid variant.
//!
//! # Knapsack Optimizations
//! - [dpsliding]
//! - [dpwalkback]
//!
//! # Continuous value knapsack
//! Normally you need to have fixed precision in order to calculate the optimal solution. Floating point numbers have too much precision.
//! You might be able to get away with using dpwalkback, but not when there's a lot of items.
//!
//! An alternative approach is to use probability to find a solution that has some % chance of fitting/being the optimal solution.
//!
//! - [continuous]
//!
//! # Testing
//! Ideally we would have a comprehensive test suite, but this is just a side project for me ;).
//!
//! Currently I just implement the same algorithm twice (using a recursive/naive approach and optimized DP approach). Fuzzing these two approaches tends to reveal most implementation issues.
//!
//! If you have ideas for how to reliably test rust code in a way that isn't a headache, I'd welcome any proof of concept PRs :)
//!
//! # Contributing
//! PRs & bugfixes are welcome!

pub mod category;
pub mod categoryrepeat;
pub mod categorythreshold;
pub mod continuous;
pub mod continuouscategory;
pub mod continuousthreshold;
pub mod continuouszeroone;
pub mod dp;
pub mod dpsliding;
pub mod dpwalkback;
pub mod helpers;
pub mod recursive;
pub mod threshold;
pub mod types;
pub mod zeroone;
pub mod zeroonethreshold;
