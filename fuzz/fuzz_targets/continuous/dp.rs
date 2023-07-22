#![no_main]

use knaptime::continuous::dp::knapsack;
use knaptime::types::ContinuousKnapsackData;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: ContinuousKnapsackData<f64, f64, i64, f64>| {
  let _ = knapsack(data.capacity, &data.items, data.precision);
});
