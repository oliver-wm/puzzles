#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type Rules = HashMap<usize, Vec<usize>>;

#[derive(Debug)]
struct Input {
    rules: Rules,
    inputs: Vec<Vec<usize>>,
}

fn read_input(file_path: &str) -> Result<Input, io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut rules = HashMap::new();
    let mut inputs = Vec::new();

    let mut first_input = true;
    for line_result in reader.lines() {
        let line = line_result?;
        if line.is_empty() {
            first_input = false;
            continue;
        }
        if first_input {
            let chars: Vec<usize> = line
                .trim()
                .split('|')
                .map(|i| i.parse::<usize>().expect("valid"))
                .collect();
            assert!(chars.len() == 2);
            rules
                .entry(chars[0])
                .and_modify(|e: &mut Vec<usize>| e.push(chars[1]))
                .or_insert(vec![chars[1]]);
        } else {
            let input: Vec<usize> = line
                .trim()
                .split(',')
                .map(|i| i.parse::<usize>().expect("valid input"))
                .collect();
            inputs.push(input);
        }
    }

    Ok(Input { rules, inputs })
}

fn middle_page(ordering: &Vec<usize>) -> usize {
    let middle = ordering.len().div_euclid(2);
    ordering[middle]
}

fn print_queue(input: Input) -> usize {
    input
        .inputs
        .iter()
        .filter(|ordering| correct_ordering(ordering, &input.rules))
        .map(middle_page)
        .sum()
}

fn correct_ordering(ordering: &Vec<usize>, rules: &Rules) -> bool {
    let mut seen: HashSet<usize> = HashSet::new();

    for page in ordering.iter() {
        if let Some(page_order) = rules.get(page) {
            if page_order.iter().any(|ord| seen.contains(ord)) {
                return false;
            }
        }
        seen.insert(*page);
    }

    true
}

#[cfg(test)]
mod tests {

    use std::vec;

    use super::*;

    #[test]
    fn test_example() {
        let out = read_input("inputs/input_example_d5.txt").expect("failed to read input");

        println!("Out: {out:?}");

        let res = print_queue(out);
        assert_eq!(res, 143);
    }

    #[test]
    fn test_middle_page() {
        assert_eq!(middle_page(&vec![75, 47, 61, 53, 29]), 61);
    }

    #[test]
    fn test_all_true_print_queue() {
        let out = Input {
            rules: HashMap::new(),
            inputs: vec![
                vec![75, 47, 61, 53, 29],
                vec![97, 61, 53, 29, 13],
                vec![75, 29, 13],
            ],
        };

        let res = print_queue(out);
        assert!(res == 143);
    }
    #[test]
    fn test_correct_ordering() {
        let mut rules = HashMap::new();
        rules.insert(97, vec![75]);
        assert_eq!(correct_ordering(&vec![75, 97, 47, 61, 53], &rules), false);
    }

    #[test]
    fn test_example_p1() {
        let out = read_input("inputs/input_d5.txt").expect("failed to read input");

        let res = print_queue(out);
        println!("Res: {res}");
        // assert_eq!(res, 143);
    }

    #[test]
    fn test_p2() {}
}
