use crate::types::Item;
use std::cmp::max;

use rand::{rngs::ThreadRng, Rng};

fn calculate_steps(
  capacity: f64,
  precision: f64,
  rng: ThreadRng,
  // Steps
) -> Result<usize, String> {
  if capacity <= 0.0 {
    return Err("Capacity cannot be <= 0".to_string());
  }

  if precision <= 0.0 {
    return Err("Precision cannot be <= 0".to_string());
  }

  let steps = capacity / precision;

  if steps.floor() > u16::MAX.into() {
    return Err("Precision too low.".to_string());
  }

  Ok(calculate_steps_with_probability(capacity / precision, rng)?.1)
}

fn sort_items(items: &[Item<f64, i64>]) -> Result<Vec<&Item<f64, i64>>, String> {
  // Ensure items are sortable
  for item in items {
    if !item.weight.is_finite() {
      return Err("Item weight is not finite".to_string());
    }
  }

  // Sort items by weight
  let mut items: Vec<&Item<f64, i64>> = items.iter().collect();
  items.sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap());

  // Throw error if any weight is 0
  for item in &items {
    if item.weight <= 0.0 || !item.weight.is_finite() {
      return Err("Item weight cannot be <= 0".to_string());
    }
  }

  Ok(items)
}

fn calculate_steps_with_probability(
  exact_steps: f64,
  mut rng: ThreadRng,
  // Returns (Confirmed steps, Actual steps, Probability of additional step)
) -> Result<(usize, usize, f64), String> {
  let confirmed_steps = exact_steps.floor() as usize;
  let probability_of_additional_step = exact_steps - confirmed_steps as f64;

  // Add an additional step (with the given probability)
  let additional_step: usize = if rng.gen_range(0.0..1.0) > probability_of_additional_step {
    1
  } else {
    0
  };

  confirmed_steps
    .checked_add(additional_step)
    .map(|s| (confirmed_steps, s, probability_of_additional_step))
    .ok_or("Value overflow".to_string())
}

pub fn knapsack(capacity: f64, items: &[Item<f64, i64>], precision: f64) -> Result<i64, String> {
  // Preprocessing
  let steps = calculate_steps(capacity, precision, rand::thread_rng())?;
  let items = sort_items(items)?;

  let mut dp = vec![0; steps + 1];

  for i in 0..=steps {
    for item in &items {
      let (confirmed_steps, item_steps, probability) =
        calculate_steps_with_probability(item.weight / precision, rand::thread_rng())?;

      let expected_instances = if item_steps == 0 {
        // Consider adding multiple instances when item_steps == 0
        // Rather than repeatedly simulate when item_steps == 0, use the expected # of flips
        // Expect # is 1/probability
        let (_confirmed_flips, expected_flips, _probability_of_additional_flip) =
          calculate_steps_with_probability(1.0 / probability, rand::thread_rng())?;
        expected_flips as i64
      } else {
        // Since item_steps != 0, we know we're reaching backwards and can only add one item before the capacity is affected.
        1
      };

      if confirmed_steps > i {
        // Skip to next capacity/set of items once it's 100% certain the remaining items won't fit.
        break;
      } else if item_steps > i {
        // This particular item does not fit.
      } else {
        // This items fits
        match item.value.checked_add(dp[i - item_steps]) {
          Some(use_item_value) => {
            // Max of (Don't use item) (Use item)
            match use_item_value.checked_mul(expected_instances) {
              Some(use_item_sum) => {
                dp[i] = max(dp[i], use_item_sum);
              }
              None => {
                return Err("Value overflow. Likely caused by item that is extremely small when divided by precision.".to_string());
              }
            }
          }
          None => {
            return Err("Value overflow".to_string());
          }
        };
      }
    }
  }

  Ok(dp[steps])
}
