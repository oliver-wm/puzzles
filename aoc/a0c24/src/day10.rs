#![allow(dead_code)]

use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{self, BufRead, BufReader},
};

const MOVES: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

macro_rules! unwrap_or_continue {
    ($opt: expr) => {
        match $opt {
            Some(v) => v,
            None => {
                continue;
            }
        }
    };
}

fn p1(top: Vec<Vec<usize>>) -> usize {
    let mut res = 0;
    for (x, row) in top.iter().enumerate() {
        for (y, pos) in row.iter().enumerate() {
            if *pos == 9 {
                let mut q: VecDeque<(usize, usize)> = VecDeque::new();
                let mut seen: HashSet<(usize, usize)> = HashSet::new();
                let mut curr_res: usize = 0;
                q.push_back((x, y));
                while !q.is_empty() {
                    let (cx, cy) = q.pop_front().expect("contains");
                    if seen.contains(&(cx, cy)) {
                        continue;
                    }
                    seen.insert((cx, cy));

                    if top[cx][cy] == 0 {
                        curr_res += 1;
                    }

                    for (nx, ny) in MOVES {
                        let next_x = unwrap_or_continue!(cx.checked_add_signed(nx));
                        let next_y = unwrap_or_continue!(cy.checked_add_signed(ny));

                        if next_x < top.len()
                            && next_y < top[0].len()
                            && top[next_x][next_y] as isize == top[cx][cy] as isize - 1
                        {
                            q.push_back((next_x, next_y));
                        }
                    }
                }
                res += curr_res;
            }
        }
    }
    res
}

fn p2(top: Vec<Vec<usize>>) -> usize {
    let mut res = 0;
    for (x, row) in top.iter().enumerate() {
        for (y, pos) in row.iter().enumerate() {
            if *pos == 9 {
                let mut q: VecDeque<(usize, usize)> = VecDeque::new();
                let mut curr_res: usize = 0;
                q.push_back((x, y));
                while !q.is_empty() {
                    let (cx, cy) = q.pop_front().expect("contains");

                    if top[cx][cy] == 0 {
                        curr_res += 1;
                    }

                    for (nx, ny) in MOVES {
                        let next_x = unwrap_or_continue!(cx.checked_add_signed(nx));
                        let next_y = unwrap_or_continue!(cy.checked_add_signed(ny));

                        if next_x < top.len()
                            && next_y < top[0].len()
                            && top[next_x][next_y] as isize == top[cx][cy] as isize - 1
                        {
                            q.push_back((next_x, next_y));
                        }
                    }
                }
                res += curr_res;
            }
        }
    }
    res
}

fn read_in<P: AsRef<std::path::Path>>(file_path: P) -> io::Result<Vec<Vec<usize>>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut top = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        let heights: Vec<usize> = line
            .trim()
            .chars()
            .map(|c| c.to_digit(10).expect("number") as usize)
            .collect();
        top.push(heights);
    }

    Ok(top)
}

#[cfg(test)]
mod tests {
    use crate::day10::*;

    #[test]
    fn test_p1() {
        let paths = p1(read_in("inputs/input_d10.txt").expect("valid"));
        println!("Res: {paths:?}");
        assert_eq!(paths, 717);
    }

    #[test]
    fn test_p2() {
        let paths = p2(read_in("inputs/input_d10.txt").expect("valid"));
        println!("Res: {paths:?}");
        assert_eq!(paths, 1686);
    }
}
