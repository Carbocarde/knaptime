use crate::types::Item;
use std::cmp::max;

pub fn knapsack(capacity: u8, item_categories: Vec<Vec<Item<u8, u64>>>) -> Result<u64, String> {
  if item_categories.is_empty() {
    return Ok(0);
  }

  // Throw error if any weight is 0
  for itemcat in &item_categories {
    if let Some(weight) = itemcat.iter().map(|item| item.weight).min() {
      if weight == 0 {
        return Err("Item weight cannot be 0".to_string());
      }
    }
  }

  let mut dp = vec![vec![0; capacity as usize + 1]; item_categories.len()];
  let capacity = capacity as usize;

  for i in 0..item_categories.len() {
    for j in 0..capacity + 1 {
      for item in &item_categories[i] {
        if j >= item.weight as usize {
          // Item fits
          if i == 0 {
            // No prior item category
            if j >= item.weight as usize {
              dp[i][j] = max(
                item.value, // Use item
                dp[i][j],   // Keep existing answer/Don't use item
              )
            }
          } else {
            dp[i][j] = max(
              match dp[i - 1][j - item.weight as usize].checked_add(item.value) {
                Some(value) => value,
                None => return Err("Overflow".to_string()),
              }, // Use item
              max(
                dp[i - 1][j], // Refer to previous category, not using item
                dp[i][j],     // Keep existing answer/Don't use item
              ),
            )
          }
        } else {
          // Item doesn't fit, just copy weight from category above
          let item_index = max(i, 1);
          dp[i][j] = dp[item_index - 1][j];
        }
      }
    }
  }

  Ok(dp[item_categories.len() - 1][capacity])
}
