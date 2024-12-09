#![allow(dead_code)]
#![allow(unused)]
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    vec,
};

#[derive(Debug)]
pub struct Equation {
    result: usize,
    nums: Vec<usize>,
}

pub fn read_input_to_eqns(file_path: &str) -> Result<Vec<Equation>, io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut eqns = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        if let Some((result_part, nums_part)) = line.split_once(':') {
            let result: usize = result_part.trim().parse().expect("Invalid result value");
            let nums: Vec<usize> = nums_part
                .split_whitespace()
                .map(|n| n.parse::<usize>().expect("Invalid number in nums"))
                .collect();
            eqns.push(Equation { result, nums });
        } else {
            panic!("Invalid input format");
        }
    }

    Ok(eqns)
}

// :/
fn concat(left: usize, right: usize) -> usize {
    let left_str = left.to_string();
    let right_str = right.to_string();

    let concatenated = format!("{}{}", left_str, right_str);

    concatenated
        .parse::<usize>()
        .expect("Failed to parse concatenated number")
}

type CENode = Option<ENode>;

#[derive(Debug)]
struct ENode {
    val: usize,
    next_add: Box<CENode>,
    next_mul: Box<CENode>,
}

impl ENode {
    fn new(val: usize) -> Self {
        Self {
            val,
            next_add: Box::new(None),
            next_mul: Box::new(None),
        }
    }

    fn add(&mut self, val: usize) {
        if self.next_add.is_none() && self.next_mul.is_none() {
            self.next_add = Box::new(Some(ENode::new(val)));
            self.next_mul = Box::new(Some(ENode::new(val)));
        } else {
            if let Some(add_next) = self.next_add.as_mut() {
                add_next.add(val);
            }

            if let Some(mul_next) = self.next_mul.as_mut() {
                mul_next.add(val);
            }
        }
    }

    fn eval(&self, result: &mut usize, eqn_val: usize) {
        if let Some(add_next) = self.next_add.as_ref() {
            add_next.eval_inner(result, self.val, eqn_val);
        }

        if let Some(mul_next) = self.next_mul.as_ref() {
            mul_next.eval_inner(result, self.val, eqn_val);
        }
    }

    fn eval_concat(&self, result: &mut usize, eqn_val: usize) {
        if let Some(add_next) = self.next_add.as_ref() {
            add_next.eval_inner_concat(result, self.val, eqn_val);
        }

        if let Some(mul_next) = self.next_mul.as_ref() {
            mul_next.eval_inner_concat(result, self.val, eqn_val);
        }
    }

    fn eval_inner(&self, result: &mut usize, acc: usize, eqn_val: usize) {
        let mul_val = acc * self.val;
        let add_val = acc + self.val;
        if self.next_add.is_none() && self.next_mul.is_none() {
            if add_val == eqn_val || mul_val == eqn_val {
                *result += 1;
            }
        }

        if let Some(add_next) = self.next_add.as_ref() {
            add_next.eval_inner(result, add_val, eqn_val);
        }

        if let Some(mul_next) = self.next_mul.as_ref() {
            mul_next.eval_inner(result, mul_val, eqn_val);
        }
    }

    fn eval_inner_concat(&self, result: &mut usize, acc: usize, eqn_val: usize) {
        if *result >= 1 {
            return;
        }
        let mul_val = acc * self.val;
        let add_val = acc + self.val;
        let concat_val = concat(acc, self.val);
        if self.next_add.is_none() && self.next_mul.is_none() {
            if add_val == eqn_val || mul_val == eqn_val || concat_val == eqn_val {
                *result += 1;
            }
        }
        if let Some(add_next) = self.next_add.as_ref() {
            add_next.eval_inner_concat(result, add_val, eqn_val);
            add_next.eval_inner_concat(result, concat_val, eqn_val);
        }

        if let Some(mul_next) = self.next_mul.as_ref() {
            mul_next.eval_inner_concat(result, mul_val, eqn_val);
            mul_next.eval_inner_concat(result, concat_val, eqn_val);
        }
    }
}

fn p1(equations: Vec<Equation>) -> usize {
    equations.iter().filter_map(valid).sum()
}

// could remove the Equation -> ENode and just go straight to ENode
fn valid(eqn: &Equation) -> Option<usize> {
    let mut root = ENode::new(*eqn.nums.first().expect("first"));

    for num in &eqn.nums[1..eqn.nums.len()] {
        root.add(*num);
    }
    let mut res: usize = 0;
    root.eval(&mut res, eqn.result);
    if res > 1 {
        return Some(eqn.result);
    }

    None
}

pub fn p2(equations: Vec<Equation>) -> usize {
    equations.iter().filter_map(valid_concat).sum()
}

fn valid_concat(eqn: &Equation) -> Option<usize> {
    let mut root = ENode::new(*eqn.nums.first().expect("first"));

    for num in &eqn.nums[1..eqn.nums.len()] {
        root.add(*num);
    }
    println!("{root:?}");

    let mut res: usize = 0;
    root.eval_concat(&mut res, eqn.result);
    println!("!!!!{res}");
    if res >= 1 {
        return Some(eqn.result);
    }

    None
}

#[cfg(test)]
mod tests {

    use crate::day7::*;

    #[test]
    fn test_concat() {
        let result = concat(12, 345);
        assert_eq!(12345, result);

        let result = concat(1, 0);
        assert_eq!(10, result); // should this be 1?

        let result = concat(0, 0);
        assert_eq!(0, result);

        let result = concat(123, 456789);
        assert_eq!(123456789, result);
    }

    #[test]
    fn ex1() {
        let eqn = read_input_to_eqns("inputs/d7e.txt").expect("input error");
        println!("Lines: {eqn:?}");
        let res = p1(eqn);
        println!("Res: {res:?}");
        assert_eq!(res, 3749);
    }

    #[test]
    fn ex2() {
        let eqn = read_input_to_eqns("inputs/d7e.txt").expect("input error");
        println!("Lines: {eqn:?}");
        let res = p2(eqn);
        println!("Res: {res:?}");
        assert_eq!(res, 11387);
    }

    #[test]
    fn test_p1() {
        let eqn = read_input_to_eqns("inputs/input_d7.txt").expect("input error");
        println!("Lines: {eqn:?}");
        let res = p1(eqn);
        println!("Res: {res:?}");
    }

    #[test]
    fn test_p2() {
        let eqn = read_input_to_eqns("inputs/input_d7.txt").expect("input error");
        println!("Lines: {eqn:?}");
        let res = p2(eqn);
        println!("Res: {res:?}");
        assert_eq!(res, 328790210468594);
    }
}
