#![no_main]

use knaptime::dp::knapsack as dp_knapsack;
use knaptime::dpsliding::knapsack as dpsliding_knapsack;
use knaptime::types::KnapsackData;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: KnapsackData<u16, u16, u64>| {
  let dp_value = dp_knapsack(data.capacity, &data.items);
  let dpsliding_value = dpsliding_knapsack(data.capacity, &data.items);
  assert_eq!(dp_value, dpsliding_value);
});
