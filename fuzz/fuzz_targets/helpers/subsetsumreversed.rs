#![no_main]

use arbitrary::Arbitrary;
use knaptime::helpers::subsetsum::{
  backward_subset_sum, backward_subset_sum_indices, subset_sum_reversed,
};
use libfuzzer_sys::fuzz_target;
use std::collections::HashSet;

#[derive(Clone, Debug, Arbitrary)]
pub struct SubsetSumData {
  pub target: u16,
  pub nums: Vec<u16>,
}

fuzz_target!(|data: SubsetSumData| {
  let naive = subset_sum_reversed(data.target, &data.nums);
  let backward = backward_subset_sum(data.target, &data.nums);
  assert_eq!(naive, backward);

  let indices = backward_subset_sum_indices(data.target, &data.nums);

  let mut manually_computed_indices = HashSet::new();
  for (i, value) in backward.iter().enumerate() {
    if *value {
      manually_computed_indices.insert(i);
    }
  }

  assert_eq!(manually_computed_indices, indices);
});
