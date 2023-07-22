use crate::types::{cast_vec_items, Item};
use std::collections::HashMap;

pub fn knapsack(
  capacity: u8,
  items: &[Item<u8, u64>],
  // Return value, weight
) -> Result<(u64, usize), String> {
  // Perform pre-run checks
  for item in items {
    if item.weight == 0 {
      return Err("Item weight cannot be 0".to_string());
    }
  }

  // Sort items by weight
  let mut items = cast_vec_items::<u8, u64, i16, u64>(items.to_vec());
  items.sort_by(|a, b| a.weight.cmp(&b.weight));

  let mut memoization = HashMap::new();

  recursive_knapsack_threshold_helper(capacity.into(), &items, &mut memoization)
}

fn recursive_knapsack_threshold_helper(
  capacity: i16,
  items: &[Item<i16, u64>],
  memoization: &mut HashMap<i16, (u64, usize)>,
  // Return value, weight
) -> Result<(u64, usize), String> {
  if capacity <= 0 || items.is_empty() {
    return Ok((0, 0));
  }

  let cached = memoization.get(&capacity);
  if let Some(item) = cached {
    return Ok(*item);
  }

  // Now, only call recursive knapsack on the items with min_threshold.
  let mut current_best = (0, usize::MAX);
  for item in items {
    let res = recursive_knapsack_threshold_helper(capacity - item.weight, items, memoization);
    let res = res?;
    let value = match res.0.checked_add(item.value) {
      Some(value) => value,
      None => return Err("Value overflow".to_string()),
    };
    let weight = match res.1.checked_add(item.weight.try_into().unwrap()) {
      Some(weight) => weight,
      None => return Err("Value overflow".to_string()),
    };
    let res = (value, weight);

    #[allow(clippy::if_same_then_else)]
    if res.1 < current_best.1 {
      // Always prefer less weight
      current_best = res;
    } else if res.0 > current_best.0 && res.1 == current_best.1 {
      // Prefer higher value on tie
      current_best = res;
    }
  }

  memoization.insert(capacity, current_best);
  Ok(current_best)
}
