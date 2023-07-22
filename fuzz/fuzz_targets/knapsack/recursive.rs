#![no_main]

use knaptime::recursive::knapsack;
use knaptime::types::KnapsackData;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: KnapsackData<u8, u8, u64>| {
  let _ = knapsack(data.capacity, &data.items);
});
