use crate::types::Item;
use std::cmp::max;

pub fn knapsack(capacity: u8, items: &[Item<u8, u64>]) -> Result<u64, String> {
  if items.is_empty() {
    return Ok(0);
  }

  // Sort items by weight
  let mut items = items.to_vec();
  items.sort_by(|a, b| a.weight.cmp(&b.weight));

  // Throw error if smallest weight is 0
  if let Some(item) = items.first() {
    if item.weight == 0 {
      return Err("Item weight cannot be 0".to_string());
    }
  }

  let mut dp = vec![vec![0; capacity as usize + 1]; items.len()];
  let capacity = capacity as usize;

  for i in 0..items.len() {
    for j in 0..capacity + 1 {
      if j >= items[i].weight as usize {
        // Item fits
        if i == 0 {
          // No prior item
          if j >= items[i].weight as usize {
            dp[i][j] = items[i].value; // Use item
          }
        } else {
          dp[i][j] = max(
            match dp[i - 1][j - items[i].weight as usize].checked_add(items[i].value) {
              Some(value) => value,
              None => return Err("Overflow".to_string()),
            }, // Use item
            dp[i - 1][j], // Don't use item
          )
        }
      } else {
        // Item doesn't fit, just copy weight from above
        let item_index = max(i, 1);
        dp[i][j] = dp[item_index - 1][j];
      }
    }
  }

  Ok(dp[items.len() - 1][capacity])
}
