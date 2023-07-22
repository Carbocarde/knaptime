#![no_main]

use knaptime::category::dp::knapsack;
use knaptime::types::CategoryKnapsackData;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: CategoryKnapsackData<u8, u8, u64>| {
  let _ = knapsack(data.capacity, data.items);
});
