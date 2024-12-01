use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

pub fn input_to_vec() -> Result<(Vec<usize>, Vec<usize>), io::Error> {
    let file_path = "inputs/input_d1.txt";

    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut column1: Vec<usize> = Vec::new();
    let mut column2: Vec<usize> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<&str> = line.split_whitespace().collect();
        if numbers.len() == 2 {
            column1.push(numbers[0].parse::<usize>().unwrap());
            column2.push(numbers[1].parse::<usize>().unwrap());
        }
    }

    // println!("Column 1: {:?}", column1);
    // println!("Column 2: {:?}", column2);
    Ok((column1, column2))
}

pub fn find_distances(mut l1: Vec<usize>, mut l2: Vec<usize>) -> usize {
    l1.sort();
    l2.sort();

    let count = l1
        .iter()
        .zip(l2.iter())
        .map(|zipped| (zipped.0).abs_diff(*zipped.1))
        .sum();
    count
}

fn similarity_score(l1: Vec<usize>, l2: Vec<usize>) -> usize {
    let frequencies = l2.iter().copied().fold(HashMap::new(), |mut map, val| {
        map.entry(val).and_modify(|frq| *frq += 1).or_insert(1);
        map
    });

    let similarity = l1
        .iter()
        .map(|num| num * frequencies.get(num).unwrap_or(&0))
        .sum();

    similarity
}

#[cfg(test)]
mod tests {
    use crate::day1::{input_to_vec, similarity_score};

    use super::find_distances;

    #[test]
    fn test_p1_answer() {
        let (v1, v2) = input_to_vec().expect("parse error");
        println!("Sol is {}", find_distances(v1, v2));
    }

    #[test]
    fn test_find_distances_example() {
        let v1 = vec![3, 4, 2, 1, 3, 3];
        let v2 = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(find_distances(v1, v2), 11);
    }

    #[test]
    fn test_p2_answer() {
        let (v1, v2) = input_to_vec().expect("parse error");
        println!("Sol is {}", similarity_score(v1, v2));
    }

    #[test]
    fn test_similarity_example() {
        let v1 = vec![3, 4, 2, 1, 3, 3];
        let v2 = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(similarity_score(v1, v2), 31);
    }
}
