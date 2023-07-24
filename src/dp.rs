//! DP Knapsack
use crate::types::Item;
use std::cmp::max;

pub fn knapsack(capacity: u16, items: &[Item<u16, u64>]) -> Result<u64, String> {
  let capacity = capacity as usize;

  let mut dp = vec![0; capacity + 1];

  // Sort items by weight
  let mut items = items.to_vec();
  items.sort_by(|a, b| a.weight.cmp(&b.weight));

  // Throw error if smallest weight is 0
  if let Some(item) = items.first() {
    if item.weight == 0 {
      return Err("Item weight cannot be 0".to_string());
    }
  }

  for i in 0..=capacity {
    for item in &items {
      if item.weight as usize > i {
        // Skip to next capacity/set of items once the remaining items won't fit.
        break;
      } else {
        // This items fits
        match item.value.checked_add(dp[i - item.weight as usize]) {
          Some(use_item_value) => {
            // Max of (Don't use item) (Use item)
            dp[i] = max(dp[i], use_item_value);
          }
          None => {
            return Err("Value overflow".to_string());
          }
        };
      }
    }
  }

  Ok(dp[capacity])
}
