use crate::helpers::binaryrangesearch::range_search_binary;
use crate::helpers::subsetsum::backward_subset_sum_indices;
use crate::types::Item;
use std::cmp::max;

/// Walkback first trims the search space by running subset sum.
/// Any indices that are not needed to calculate the end result are omitted from the knapsack calculation.
///
/// # Example
/// Capacity: 5                                  \
/// Items (weight): \[3, 4\]                     \
/// \[o, x, x, o, o, x\]                         \
/// Trim o's                                     \
/// \[1,2,5\]                                    \
/// Perform knapsack using _only_ these indices \
/// dp = \[0,0,0\]                               \
/// return dp\[-1\]
///
/// # Example
/// ```
/// use knapsack::types::Item;
/// let items = &[Item {weight: 3, value: 10}, Item {weight: 4, value: 5}];
/// let result = knapsack::dpwalkback::knapsack(5, items);
/// assert_eq!(result, Ok(10));
/// ```
pub fn knapsack(capacity: u16, items: &[Item<u16, u64>]) -> Result<u64, String> {
  // Compute the knapsack problem at the given indices
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

  // Calculate indices that _can_ contribute to the result
  let offsets = backward_subset_sum_indices(
    capacity as u16,
    &items.iter().map(|item| item.weight).collect(),
  );

  let mut offsets: Vec<usize> = offsets.into_iter().collect();
  offsets.sort();
  let offsets = offsets;

  // Compute knapsack at the given points
  let mut dp = vec![0; offsets.len()];

  for i in 0..=offsets.len() - 1 {
    for item in &items {
      if item.weight as usize > offsets[i] {
        // Skip to next capacity/set of items once the remaining items won't fit.
        break;
      } else {
        // This items fits
        // Binary search for correct offset range (<= actual index)
        let index = range_search_binary(&offsets, offsets[i] - item.weight as usize).unwrap();

        match item.value.checked_add(dp[index]) {
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

  Ok(dp[offsets.len() - 1])
}
