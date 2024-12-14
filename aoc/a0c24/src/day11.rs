#![allow(dead_code)]

use std::collections::HashMap;

fn is_digit_length_even_iterative(mut num: usize, even_cache: &mut HashMap<usize, bool>) -> bool {
    if even_cache.contains_key(&num) {
        return *even_cache.get(&num).expect("day 11");
    }
    if num == 0 {
        return false;
    }

    let mut count = 0;
    let orig_num = num;
    while num > 0 {
        count += 1;
        num /= 10;
    }

    let res = count % 2 == 0;
    even_cache.insert(orig_num, res);
    res
}

fn split_usize_even_math(
    num: usize,
    hash: &mut HashMap<usize, (usize, usize)>,
) -> Option<(usize, usize)> {
    if hash.contains_key(&num) {
        return hash.get(&num).copied();
    }
    if num == 0 {
        return None;
    }

    let mut count = 0;
    let mut temp = num;
    while temp > 0 {
        count += 1;
        temp /= 10;
    }

    if count % 2 != 0 {
        return None;
    }

    let divisor = 10usize.pow((count / 2) as u32);

    let first_half = num / divisor;
    let second_half = num % divisor;
    hash.insert(num, (first_half, second_half));
    Some((first_half, second_half))
}

fn apply_iterative(stones: Vec<usize>, max_depth: usize) -> usize {
    let mut current = HashMap::new();
    for &stone in &stones {
        *current.entry(stone).or_insert(0) += 1;
    }

    let mut split_cache = HashMap::new();
    let mut even_cache = HashMap::new();

    for _ in 1..=max_depth {
        let mut next = HashMap::new();

        for (&num, &count) in &current {
            if is_digit_length_even_iterative(num, &mut even_cache) {
                if let Some((l, r)) = split_usize_even_math(num, &mut split_cache) {
                    *next.entry(l).or_insert(0) += count;
                    *next.entry(r).or_insert(0) += count;
                }
            } else {
                let r = if num == 0 { 1 } else { num * 2024 };
                *next.entry(r).or_insert(0) += count;
            }
        }

        current = next;
    }

    current.values().sum()
}

#[cfg(test)]
mod tests {
    use crate::day11::*;

    #[test]
    fn test_ex1() {
        let stones = [125, 17];
        let stones = stones.iter().map(|i| *i as usize).collect();
        let stones = apply_iterative(stones, 25);
        println!("Res: {stones}");
        assert_eq!(stones, 55312);
    }

    #[test]
    fn test_p1() {
        let stones = [28, 4, 3179, 96938, 0, 6617406, 490, 816207];
        let stones = stones.iter().map(|i| *i as usize).collect();
        let stones = apply_iterative(stones, 75);
        println!("Res: {stones}");
        assert_eq!(stones, 225253278506288);
    }
}
