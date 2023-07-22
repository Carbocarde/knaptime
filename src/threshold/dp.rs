use crate::types::Item;
use std::cmp::max;

pub fn knapsack(capacity: u8, items: &[Item<u8, u64>]) -> Result<(u64, usize), String> {
  // Sort items by weight
  let mut items = items.to_vec();
  items.sort_by(|a, b| a.weight.cmp(&b.weight));

  let max_overflow = items
    .first()
    .unwrap_or(&Item {
      weight: 0,
      value: 0,
    })
    .weight as usize;

  let capacity = capacity as usize;
  let mut dp = vec![0; capacity + max_overflow + 1];
  let mut weight = vec![0; capacity + max_overflow + 1];

  // Throw error if smallest weight is 0
  if let Some(item) = items.first() {
    if item.weight == 0 {
      return Err("Item weight cannot be 0".to_string());
    }
  }

  for i in 0..=(capacity + max_overflow) {
    for item in &items {
      if item.weight as usize > i {
        // Skip to next capacity/set of items once the remaining items won't fit.
        break;
      } else {
        // This items fits
        // Only consider this if it's fully utilizing the knapsack
        if i - item.weight as usize == 0 || weight[i - item.weight as usize] != 0 {
          match item.value.checked_add(dp[i - item.weight as usize]) {
            Some(use_item_value) => {
              // Max of (Don't use item) (Use item)
              dp[i] = max(dp[i], use_item_value);
              weight[i] = max(
                weight[i],
                weight[i - item.weight as usize] + item.weight as usize,
              )
            }
            None => {
              return Err("Value overflow".to_string());
            }
          };
        }
      }
    }
    if i >= capacity && weight[i] >= capacity {
      return Ok((dp[i], i));
    }
  }

  if items.is_empty() {
    return Ok((0, 0));
  }

  Err("Unable to reach threshold".to_string())
}
