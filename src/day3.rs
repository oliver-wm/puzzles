#![allow(unused_imports)]
#![allow(dead_code)]

use std::{fs, io};

// To make recursive descent add Number here and replace rhs, lhs with Expression in Mul
#[derive(Debug)]
enum Expression {
    Op(Mul),
}

#[derive(Debug)]
struct Mul {
    lhs: i64,
    rhs: i64,
}

fn starts_with(chars: &[char], pos: usize, prefix: &str) -> bool {
    if pos + prefix.len() < chars.len() {
        chars[pos..].iter().collect::<String>().starts_with(prefix)
    } else {
        false
    }
}

fn tokens(st: String) -> Vec<Expression> {
    let mut i = 0;
    let mut digits: Vec<String> = Vec::new();
    let mut tokens: Vec<Expression> = Vec::new();
    let st: Vec<char> = st.chars().collect();
    let st = st.as_slice();
    let mut toggle = true;
    while i < st.len() {
        if starts_with(st, i, "don't()") {
            toggle = false;
            i += 7;
            continue;
        }
        if starts_with(st, i, "do()") {
            toggle = true;
            i += 4;
            continue;
        }
        if toggle && starts_with(st, i, "mul(") {
            i += 4;
            let mut start = i;
            while st[start].is_digit(10) {
                start += 1;
            }
            digits.push(st[i..start].iter().collect());
            i = start;

            if st[i] != ',' {
                digits.pop();
            } else {
                i += 1;
                let mut start = i;
                while st[start].is_digit(10) {
                    start += 1;
                }
                digits.push(st[i..start].iter().collect());
                i = start;
                if st[i] != ')' {
                    digits.pop();
                    digits.pop();
                } else {
                    let rhs = digits
                        .pop()
                        .expect("rhs")
                        .parse::<i64>()
                        .expect("parse rhs");
                    let lhs = digits
                        .pop()
                        .expect("lhs")
                        .parse::<i64>()
                        .expect("parse rhs");
                    tokens.push(Expression::Op(Mul { lhs, rhs }));
                }
            }
        }

        i += 1;
    }

    tokens
}

pub fn input_to_string() -> Result<String, io::Error> {
    let file_path = "inputs/input_d3.txt";
    fs::read_to_string(file_path)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let tokens = tokens(input.into());
        println!("Tokens {:?}", tokens);

        let mut result: i64 = 0;
        for expr in tokens {
            result += match expr {
                Expression::Op(Mul { lhs, rhs }) => lhs * rhs,
            }
        }
        assert_eq!(161, result);
    }

    #[test]
    fn test_example2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let tokens = tokens(input.into());
        println!("Tokens {:?}", tokens);

        let mut result: i64 = 0;
        for expr in tokens {
            result += match expr {
                Expression::Op(Mul { lhs, rhs }) => lhs * rhs,
            }
        }
        assert_eq!(48, result);
    }

    #[test]
    fn test_p1() {
        let input = input_to_string().expect("bad input");

        let tokens = tokens(input);
        println!("Tokens {:?}", tokens);

        let mut result: i64 = 0;
        for expr in tokens {
            result += match expr {
                Expression::Op(Mul { lhs, rhs }) => lhs * rhs,
            }
        }

        print!("Result {}", result);
    }
}
