#![allow(dead_code)]

use std::{
    collections::{BTreeSet, HashSet, VecDeque},
    fs::File,
    io::{self, BufRead, BufReader},
};

fn read_input_to_grid(file_path: &str) -> Result<Vec<Vec<char>>, io::Error> {
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

const fn moves_map(ch: char) -> (isize, isize) {
    match ch {
        '^' => (-1, 0),
        'v' => (1, 0),
        '<' => (0, -1),
        '>' => (0, 1),
        _ => unreachable!(),
    }
}

const fn turn_moves(ch: char) -> (isize, isize, char) {
    match ch {
        '^' => (0, 1, '>'),
        'v' => (0, -1, '<'),
        '<' => (-1, 0, '^'),
        '>' => (1, 0, 'v'),
        _ => unreachable!(),
    }
}

fn find_start(grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if "v><^".contains(grid[x][y]) {
                return Some((x, y));
            }
        }
    }
    None
}

fn validate_move(
    nx: isize,
    x: usize,
    ny: isize,
    y: usize,
    grid: &Vec<Vec<char>>,
) -> Option<(usize, usize)> {
    let test_x = x as isize + nx;
    let test_y = y as isize + ny;

    if test_x < 0 || test_x >= grid.len() as isize || test_y < 0 || test_y >= grid[0].len() as isize
    {
        return None;
    }

    Some((test_x as usize, test_y as usize))
}

fn guard_path(grid: &Vec<Vec<char>>) -> BTreeSet<(usize, usize)> {
    let start: (usize, usize) = find_start(grid).expect("start");
    let mut guard_pos = grid[start.0][start.1];

    let mut guard_path = BTreeSet::new();
    let mut q = VecDeque::new();

    q.push_front(start);
    while !q.is_empty() {
        let curr = q.pop_front().expect("no value");
        guard_path.insert(curr);
        let next = moves_map(guard_pos);

        let (next_x, next_y) = match validate_move(next.0, curr.0, next.1, curr.1, grid) {
            Some(value) => value,
            None => break,
        };

        if grid[next_x][next_y] == '#' {
            let (turn_x, turn_y, new_pos) = turn_moves(guard_pos);
            guard_pos = new_pos;

            let (next_x, next_y) = match validate_move(turn_x, curr.0, turn_y, curr.1, grid) {
                Some(value) => value,
                None => continue,
            };
            q.push_back((next_x, next_y));
        } else {
            q.push_back((next_x, next_y));
        }
    }
    guard_path
}

fn next(x: usize, y: usize, guard: char, grid: &Vec<Vec<char>>) -> Option<(usize, usize, char)> {
    let next = moves_map(guard);
    let mut new_guard = guard;
    let mut dx: isize = next.0;
    let mut dy: isize = next.1;
    for _ in 0..4 {
        if let Some(valid) = validate_move(dx, x, dy, y, grid) {
            if grid[valid.0][valid.1] != '#' {
                return Some((valid.0, valid.1, new_guard));
            }
        }
        let next = turn_moves(new_guard);
        dx = next.0;
        dy = next.1;
        new_guard = next.2;
    }
    None
}

fn guard_path_loops(grid: &Vec<Vec<char>>, (start_x, start_y): (usize, usize)) -> bool {
    let guard_start = grid[start_x][start_y];

    let mut q = VecDeque::new();
    q.push_back((start_x, start_y, guard_start));
    let mut turns = HashSet::with_capacity(500);
    let mut seen = HashSet::with_capacity(500);

    while let Some((x, y, guard)) = q.pop_front() {
        seen.insert((x, y, guard));
        let (nx, ny, nd) = match next(x, y, guard, grid) {
            Some(n) => n,
            None => break,
        };

        let next = (nx, ny, nd);
        let prev = turn_moves(nd);
        if seen.contains(&(next.0, next.1, prev.2)) {
            return true;
        }
        turns.insert(next);
        q.push_back(next);
    }

    false
}

// fix this
fn test_obstructions(grid: &mut Vec<Vec<char>>) -> usize {
    let visited_positions = guard_path(grid);
    let (start_x, start_y) = find_start(grid).expect("should start");

    let mut obstruction_points = HashSet::with_capacity(2000);

    for &(x, y) in &visited_positions {
        let curr_pos = grid[x][y];

        if curr_pos == '.' {
            let orig = grid[x][y];
            grid[x][y] = '#';

            if guard_path_loops(grid, (start_x, start_y)) {
                obstruction_points.insert((x, y));
            }

            grid[x][y] = orig;
        }
    }

    obstruction_points.len()
}

#[cfg(test)]
mod tests {
    use crate::day6::*;

    #[test]
    fn test_ex1() {
        let grid = read_input_to_grid("inputs/input_example_d6.txt").expect("parse error");
        let positions = guard_path(&grid);
        assert_eq!(positions.len(), 41);
    }

    #[test]
    fn test_p1() {
        let grid = read_input_to_grid("inputs/input_d6.txt").expect("parse error");
        let positions = guard_path(&grid);
        println!("Result: {}", positions.len());
    }

    #[test]
    fn test_ex3() {
        let mut grid = read_input_to_grid("inputs/d6_ex.txt").expect("parse error");
        let positions = test_obstructions(&mut grid);

        println!("Positions {:?}", positions);
        assert_eq!(positions, 6)
    }

    #[test]
    fn test_ex2() {
        let mut grid = read_input_to_grid("inputs/input_example_d6.txt").expect("parse error");
        let positions = test_obstructions(&mut grid);

        println!("Positions {:?}", positions);
        assert_eq!(positions, 1)
    }

    #[test]
    fn test_p2() {
        let mut grid = read_input_to_grid("inputs/input_d6.txt").expect("parse error");
        let positions = test_obstructions(&mut grid);

        println!("Positions {:?}", positions);
        println!("Positions {:?}", positions / 2);
    }
}
