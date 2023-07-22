//! Types and casting utilities
use arbitrary::Arbitrary;

/// Used for fuzzing
#[derive(Clone, Debug, Arbitrary)]
pub struct KnapsackData<T, U, V> {
  pub capacity: T,
  pub items: Vec<Item<U, V>>,
}

/// Used for fuzzing
#[derive(Clone, Debug, Arbitrary)]
pub struct ContinuousKnapsackData<T, U, V, W> {
  pub capacity: T,
  pub items: Vec<Item<U, V>>,
  pub precision: W,
}

/// Used for fuzzing
#[derive(Clone, Debug, Arbitrary)]
pub struct CategoryKnapsackData<T, U, V> {
  pub capacity: T,
  pub items: Vec<Vec<Item<U, V>>>,
}

#[derive(Clone, Debug, Arbitrary)]
pub struct Item<T, V> {
  pub weight: T,
  pub value: V,
}

impl From<Item<u8, u64>> for Item<u16, u64> {
  fn from(item: Item<u8, u64>) -> Item<u16, u64> {
    Item {
      weight: item.weight as u16,
      value: item.value,
    }
  }
}

impl From<Item<u8, u64>> for Item<i16, u64> {
  fn from(item: Item<u8, u64>) -> Item<i16, u64> {
    Item {
      weight: item.weight as i16,
      value: item.value,
    }
  }
}

pub fn cast_vec_items<T, U, V, W>(vec: Vec<Item<T, U>>) -> Vec<Item<V, W>>
where
  Item<V, W>: From<Item<T, U>>,
{
  let len = vec.len();
  let mut vec_items = Vec::<Item<V, W>>::with_capacity(len);
  for item in vec {
    vec_items.push(item.into());
  }
  vec_items
}
