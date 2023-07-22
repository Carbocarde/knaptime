#![no_main]

use knaptime::dp::knapsack as dp_knapsack;
use knaptime::dpwalkback::knapsack as dpwalkback_knapsack;
use knaptime::types::KnapsackData;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: KnapsackData<u16, u16, u64>| {
  let dp_value = dp_knapsack(data.capacity, &data.items);
  let dpwalkback_value = dpwalkback_knapsack(data.capacity, &data.items);
  assert_eq!(dp_value, dpwalkback_value);
});
