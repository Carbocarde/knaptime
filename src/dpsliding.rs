use crate::types::Item;
use std::cmp::max;

/// A sliding window dp knapsack solution.
/// Useful when capacity is large and your max weight is small.
/// Instead of allocating a large array, just allocate an array of max weight.
/// Then look back and wrap around if needed.
///
/// # Example
/// Capacity: 50                                              \
/// Items (weight): \[3\]                                       \
/// Perform knapsack using a rolling array of max(weight + 1) \
/// dp = \[0,0,0,0\]                                            \
/// return dp\[capacity % max(weight)\]
///
/// # Example
/// ```
/// use knapsack::types::Item;
/// let items = &[Item {weight: 3, value: 10}];
/// let result = knapsack::dpsliding::knapsack(50, items);
/// assert_eq!(result, Ok(160));
/// ```
pub fn knapsack(capacity: u16, items: &[Item<u16, u64>]) -> Result<u64, String> {
  let capacity = capacity as usize;

  // Sort items by weight
  let mut items = items.to_vec();
  items.sort_by(|a, b| a.weight.cmp(&b.weight));

  // Throw error if smallest weight is 0
  if let Some(item) = items.first() {
    if item.weight == 0 {
      return Err("Item weight cannot be 0".to_string());
    }
  }

  // TODO: Get largest weight <= capacity
  let largest_weight = items
    .last()
    .unwrap_or(&Item {
      weight: 0,
      value: 0,
    })
    .weight as usize;

  let dp_capacity = largest_weight + 1;

  let mut dp = vec![0; dp_capacity];

  for i in 0..=capacity {
    for item in &items {
      if item.weight as usize > i {
        // Skip to next capacity/set of items once the remaining items won't fit.
        break;
      } else {
        // This items fits
        match item
          .value
          .checked_add(dp[((i - item.weight as usize) + dp_capacity) % dp_capacity])
        {
          Some(use_item_value) => {
            // Max of (Don't use item) (Use item)
            dp[i % dp_capacity] = max(dp[i % dp_capacity], use_item_value);
          }
          None => {
            return Err("Value overflow".to_string());
          }
        };
      }
    }
  }

  Ok(dp[capacity % dp_capacity])
}
