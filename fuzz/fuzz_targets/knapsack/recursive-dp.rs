#![no_main]

use knaptime::dp::knapsack as dp_knapsack;
use knaptime::recursive::knapsack as recursive_knapsack;
use knaptime::types::{cast_vec_items, KnapsackData};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: KnapsackData<u8, u8, u64>| {
  let recursive_value = recursive_knapsack(data.capacity, &data.items);
  let items = cast_vec_items::<u8, u64, u16, u64>(data.items);
  let dp_value = dp_knapsack(data.capacity as u16, &items);
  assert_eq!(recursive_value, dp_value);
});
