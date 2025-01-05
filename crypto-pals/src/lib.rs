#![allow(dead_code)]

use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

fn frequency_count<T>(items: &[T]) -> HashMap<&T, usize>
where
    T: Eq + Hash,
{
    let mut freq = HashMap::new();
    for item in items {
        *freq.entry(item).or_insert(0) += 1;
    }
    freq
}

fn most_frequent_item<T>(freq: &HashMap<T, usize>) -> Option<(&T, usize)>
where
    T: Eq + Hash,
{
    freq.iter()
        .max_by_key(|&(_, &count)| count)
        .map(|(item, &count)| (item, count))
}
