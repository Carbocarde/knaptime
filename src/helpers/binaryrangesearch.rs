use std::cmp::Ordering;

/// Returns the index of the closest number that is strictly less than or equal to the given number.
pub fn range_search_binary(ranges: &[usize], number: usize) -> Option<usize> {
  let mut low = 0;
  let mut high = ranges.len() - 1;

  while low <= high {
    let mid = (low + high) / 2;

    match ranges[mid].cmp(&number) {
      Ordering::Equal => return Some(mid),
      Ordering::Less => low = mid + 1,
      Ordering::Greater => high = mid - 1,
    }
  }

  if low == 0 {
    return None;
  }

  Some(low - 1)
}
