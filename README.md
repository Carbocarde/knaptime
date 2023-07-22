 # Knaptime
This crate aims to solve variants of the knapsack problem.

It was constructed due to the lack of easily-accessible implementations of knapsack variant algorithms online.
Language bindings are planned to allow these functions to be used in your language of choice.

# Documentation
View all the variants using `cargo doc --open`

# Testing
Ideally we would have a comprehensive test suite, but this is just a side project for me ;).

Currently I just implement the same algorithm twice (using a recursive/naive approach and optimized DP approach). Fuzzing these two approaches tends to reveal most implementation issues.

If you have ideas for how to reliably test rust code in a way that isn't a headache, I'd welcome any proof of concept PRs :)

# Contributing
PRs & bugfixes are welcome!
