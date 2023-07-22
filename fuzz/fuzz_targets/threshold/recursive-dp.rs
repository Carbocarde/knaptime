#![no_main]

use knaptime::threshold::dp::knapsack as dp_knapsack;
use knaptime::threshold::recursive::knapsack as recursive_knapsack;
use knaptime::types::KnapsackData;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: KnapsackData<u8, u8, u64>| {
  let recursive_value = recursive_knapsack(data.capacity, &data.items);
  let dp_value = dp_knapsack(data.capacity, &data.items);
  // Overflows will happen in one but not the other, ignore this difference.
  match recursive_value {
    Ok(recursive_value) => match dp_value {
      Ok(dp_value) => {
        assert_eq!(recursive_value, dp_value);
      }
      Err(dp_error) => {
        // Dp shouldn't error unless recursive also fails
        assert_eq!(dp_error, "Value overflow".to_string());
      }
    },
    Err(recursive_err) => match dp_value {
      Ok(_) => {
        // Validate recursive_err is an overflow
        assert_eq!(recursive_err, "Value overflow".to_string());
      }
      Err(dp_err) => {
        assert_eq!(recursive_err, dp_err);
      }
    },
  }
});
