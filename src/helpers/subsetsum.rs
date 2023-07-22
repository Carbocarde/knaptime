use std::collections::HashSet;

/// Naive implementation.
///
/// Used as a source of truth to fuzz the other functions.
pub fn subset_sum_reversed(target: u16, nums: &Vec<u16>) -> Vec<bool> {
  let target = target as usize;

  // Search backwards, marking cells that need to be computed
  let mut dp = vec![false; target + 1];
  dp[0] = true;

  // Compute backwards array
  for num in nums {
    let num = *num as usize;
    for i in 0..target + 1 {
      if i >= num {
        dp[i] = dp[i] || dp[i - num];
      }
    }
  }

  dp.reverse();

  dp
}

/// Iterate backwards so we don't need to reverse the array at the end.
/// Used to fuzz the backward_subset_sum_indices function.
pub fn backward_subset_sum(target: u16, nums: &Vec<u16>) -> Vec<bool> {
  let target = target as usize;

  // Initialize the dp array
  let mut dp = vec![false; target + 1];
  dp[target] = true;

  // Iterate over the numbers in nums
  for num in nums {
    let num = *num as usize;

    for i in 0..=target {
      // Iterate over the dp array, from the end to the beginning
      let index = target - i;
      if i >= num {
        dp[index] = dp[index] || dp[index + num];
      }
    }
  }

  dp
}

/// Use a set of indices instead to avoid allocating a large array.
pub fn backward_subset_sum_indices(target: u16, nums: &Vec<u16>) -> HashSet<usize> {
  let target = target as usize;

  let mut dp = HashSet::new();
  dp.insert(target);

  // Iterate over the numbers in nums
  for num in nums {
    // Iterate over the dp array, from the end to the beginning
    for i in (0..=target).rev() {
      let num = *num as usize;

      if dp.contains(&(i + num)) {
        dp.insert(i);
      }
    }
  }

  dp
}
