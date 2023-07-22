use crate::types::Item;
use std::collections::HashMap;

pub fn knapsack(capacity: u8, items: &[Item<u8, u64>]) -> Result<u64, String> {
  // Perform pre-run checks
  for item in items {
    if item.weight == 0 {
      return Err("Item weight cannot be 0".to_string());
    }
  }

  // Memoization table
  let mut memoization = HashMap::new();

  // Run
  _recursive_knapsack_helper(capacity, items, &mut memoization)
}

fn _recursive_knapsack_helper(
  capacity: u8,
  items: &[Item<u8, u64>],
  memoization: &mut HashMap<u8, u64>,
) -> Result<u64, String> {
  if capacity == 0 {
    return Ok(0);
  } else if let Some(value) = memoization.get(&capacity) {
    return Ok(*value);
  }

  let capacity = capacity as usize;

  let mut best_value = 0;
  for item in items {
    if item.weight as usize <= capacity {
      match item.value.checked_add(_recursive_knapsack_helper(
        capacity as u8 - item.weight,
        items,
        memoization,
      )?) {
        Some(value) => {
          if value > best_value {
            best_value = value;
          }
        }
        None => {
          return Err("Value overflow".to_string());
        }
      };
    }
  }

  memoization.insert(capacity as u8, best_value);
  Ok(best_value)
}
