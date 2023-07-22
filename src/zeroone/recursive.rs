use crate::types::Item;

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

  if capacity == 0 {
    return Ok(0);
  }

  let mut best_result = 0;
  for (i, item) in items.iter().enumerate() {
    if item.weight > capacity {
      continue;
    }

    let value = knapsack(capacity - item.weight, &items[i + 1..])?;
    match item.value.checked_add(value) {
      Some(value) => {
        if value > best_result {
          best_result = value;
        }
      }
      None => return Err("Overflow".to_string()),
    }
  }

  Ok(best_result)
}
