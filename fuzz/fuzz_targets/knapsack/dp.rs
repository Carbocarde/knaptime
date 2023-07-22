#![no_main]

use knaptime::dp::knapsack;
use knaptime::types::KnapsackData;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: KnapsackData<u16, u16, u64>| {
  let _ = knapsack(data.capacity, &data.items);
});
