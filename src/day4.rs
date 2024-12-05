#![allow(unused_imports)]
#![allow(dead_code)]
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use std::{
    any,
    collections::{HashSet, VecDeque},
};

/// up, down, left, right, horizonals
const MOVES: [(i32, i32); 8] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
];

fn word_search(words: Vec<Vec<char>>) -> u64 {
    let rows = words.len();
    let cols = words[0].len();
    let mut count = 0;

    for x in 0..rows {
        for y in 0..cols {
            if words[x][y] == 'X' {
                for &(dx, dy) in &MOVES {
                    count += search_in_direction(&words, x as i32, y as i32, dx, dy);
                }
            }
        }
    }

    count
}

fn search_in_direction(words: &Vec<Vec<char>>, x: i32, y: i32, dx: i32, dy: i32) -> u64 {
    let target = ['X', 'M', 'A', 'S'];
    let mut nx = x;
    let mut ny = y;

    for &ch in &target {
        if nx < 0 || ny < 0 || nx >= words.len() as i32 || ny >= words[0].len() as i32 {
            return 0;
        }

        if words[nx as usize][ny as usize] != ch {
            return 0;
        }

        nx += dx;
        ny += dy;
    }

    1
}

pub fn read_input_to_grid() -> Result<Vec<Vec<char>>, io::Error> {
    let file_path = "inputs/input_d4.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut grid = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        let chars: Vec<char> = line.trim().chars().collect();
        grid.push(chars);
    }

    Ok(grid)
}

#[cfg(test)]
mod tests {
    use crate::day4::{read_input_to_grid, word_search};

    #[test]
    fn test_example() {
        let words = vec![vec!['X', 'M', 'A', 'S']];

        assert_eq!(word_search(words), 1);
    }

    #[test]
    fn test_example2() {
        let words = vec![
            vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
            vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
            vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
            vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
            vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
            vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
            vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
            vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
            vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
            vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
        ];
        assert_eq!(word_search(words), 18);
    }

    #[test]
    fn test_p1() {
        let words = read_input_to_grid().expect("parse error");
        println!("Words: {:?}", words);
        println!("Result: {}", word_search(words));
    }
}
