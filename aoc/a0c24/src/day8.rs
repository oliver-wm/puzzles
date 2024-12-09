#![allow(dead_code)]
#![allow(unused)]
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

use crate::utils::Point;

fn read_input_to_grid(file_path: &str) -> Vec<Vec<char>> {
    let f = File::open(file_path).expect("failed to open file");
    let reader = BufReader::new(f);
    let mut grid = vec![];

    for line in reader.lines() {
        let line = line.expect("line?");
        grid.push(line.chars().collect());
    }
    grid
}

fn get_char_from_pos_grid(x: usize, y: usize, grid: &Vec<Vec<char>>) -> Option<char> {
    if let Some(row) = grid.get(x) {
        if let Some(char) = row.get(y) {
            return Some(*char);
        }
    }
    None
}

fn collect_antennas(grid: &Vec<Vec<char>>) -> HashMap<char, Vec<(usize, usize)>> {
    let mut all_antennas = HashMap::new();
    for row in grid.iter() {
        for val in row.iter() {
            if *val != '.' && !all_antennas.contains_key(val) {
                all_antennas.insert(*val, find_antennas(grid, *val));
            }
        }
    }
    println!("ALL: {all_antennas:?}");
    all_antennas
}

fn find_antennas(grid: &Vec<Vec<char>>, antenna: char) -> Vec<(usize, usize)> {
    let mut antennas = Vec::new();
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if Some(antenna) == get_char_from_pos_grid(x, y, grid) {
                antennas.push((x, y));
            }
        }
    }
    antennas
}

fn diffs(p1: (usize, usize), p2: (usize, usize)) -> Vec<(isize, isize)> {
    let x = p1.0 as isize - p2.0 as isize;
    let y = p1.1 as isize - p2.1 as isize;
    vec![(x, y), (-x, y), (-x, -y), (x, -y)]
}

fn get_coords(p: &(usize, usize), dx: isize, dy: isize) -> Option<(usize, usize)> {
    let nx = p.0 as isize + dx;
    let ny = p.1 as isize + dy;
    if nx >= 0 && ny >= 0 {
        Some((nx as usize, ny as usize))
    } else {
        None
    }
}

fn pairs<I: IntoIterator>(x: I) -> impl Iterator<Item = (I::Item, I::Item)>
where
    I::Item: Clone,
    I: Copy,
{
    x.into_iter()
        .enumerate()
        .flat_map(move |t| std::iter::repeat(t.1).zip(x.into_iter().skip(t.0 + 1)))
}

fn find_antinodes(
    grid: &Vec<Vec<char>>,
    antennas: &Vec<(usize, usize)>,
    process_all_pts: bool,
) -> HashSet<(usize, usize)> {
    let mut antinode_positions = HashSet::new();

    for (antenna1, antenna2) in pairs(antennas) {
        let p1 = Point::from(antenna1);
        let p2 = Point::from(antenna2);

        for (dx, dy) in diffs(*antenna1, *antenna2) {
            process_antinode(
                grid,
                antenna1,
                p1,
                p2,
                dx,
                dy,
                &mut antinode_positions,
                process_all_pts,
            );
            process_antinode(
                grid,
                antenna2,
                p1,
                p2,
                dx,
                dy,
                &mut antinode_positions,
                process_all_pts,
            );
        }
    }

    for antenna in antennas {
        antinode_positions.remove(antenna);
    }

    antinode_positions
}

fn process_antinode(
    grid: &Vec<Vec<char>>,
    antenna: &(usize, usize),
    p1: Point,
    p2: Point,
    dx: isize,
    dy: isize,
    antinode_positions: &mut HashSet<(usize, usize)>,
    all_pts: bool,
) {
    if let Some((px, py)) = get_coords(antenna, dx, dy) {
        if let Some(_) = get_char_from_pos_grid(px, py, grid) {
            let p3 = Point::from(&(px, py));
            if Point::collinear(p1, p2, p3) {
                antinode_positions.insert((px, py));
                if all_pts {
                    find_collinear_points(grid, p1, p2, (px, py), dx, dy, antinode_positions);
                }
            }
        }
    }
}

fn find_collinear_points(
    grid: &Vec<Vec<char>>,
    p1: Point,
    p2: Point,
    start: (usize, usize),
    dx: isize,
    dy: isize,
    antinode_positions: &mut HashSet<(usize, usize)>,
) {
    let mut current = start;

    loop {
        if let Some((nx, ny)) = get_coords(&current, dx, dy) {
            if let Some(_) = get_char_from_pos_grid(nx, ny, grid) {
                let p3 = Point::from(&(nx, ny));
                if Point::collinear(p1, p2, p3) {
                    antinode_positions.insert((nx, ny));
                    current = (nx, ny);
                } else {
                    break;
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }
}

fn p1(grid: &Vec<Vec<char>>) -> usize {
    let all = collect_antennas(grid);

    let antennas: HashSet<_> = all.values().flat_map(|v| v.iter().cloned()).collect();

    let mut antinodes: HashSet<_> = all
        .iter()
        .flat_map(|(_, v)| find_antinodes(grid, v, false))
        .collect();

    antinodes.len()
}

fn p2(grid: &Vec<Vec<char>>) -> usize {
    let all = collect_antennas(grid);

    let antennas: HashSet<_> = all.values().flat_map(|v| v.iter().cloned()).collect();

    let mut antinodes: HashSet<_> = all
        .iter()
        .flat_map(|(_, v)| find_antinodes(grid, v, true))
        .collect();

    antinodes.extend(antennas.iter());

    antinodes.len()
}
#[cfg(test)]
mod tests {

    use crate::day8::*;

    #[test]
    fn ex1() {
        let eqn = read_input_to_grid("inputs/d8_ex.txt");
        for g in eqn.iter() {
            println!("vec!{g:?},")
        }
        let res = p1(&eqn);
        println!("Res: {res:?}");
    }

    #[test]
    fn ex3() {
        let anitnodes = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '#', '.', '.', '.', '.', '.', '.'],
            vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', 'a', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', 'a', '.'],
            vec!['.', '.', '.', '.', '.', 'a', '.', '.', '.', '.'],
            vec!['.', '.', '#', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];

        let find_antennas = find_antennas(&anitnodes, 'a');
        let antennas: Vec<(usize, usize)> = find_antennas;
        println!("{antennas:?}");

        let res = find_antinodes(&anitnodes, &antennas, false);

        println!("{res:?}");
        for (x, y) in res.iter() {
            println!("{x} {y}");
            assert_eq!(get_char_from_pos_grid(*x, *y, &anitnodes), Some('#'));
        }
        assert_eq!(res.len(), 4);
    }
    #[test]
    fn ex2() {
        let anitnodes = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '#', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', 'a', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', 'a', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];
        let find_antennas = find_antennas(&anitnodes, 'a');
        let antennas: Vec<(usize, usize)> = find_antennas;

        let res = find_antinodes(&anitnodes, &antennas, false);

        println!("{res:?}");
        for (x, y) in res.iter() {
            println!("{x} {y}");
            assert_eq!(get_char_from_pos_grid(*x, *y, &anitnodes), Some('#'));
        }
        assert_eq!(res.len(), 2);
    }
    #[test]
    fn test_ex1_0s() {
        let grid = read_input_to_grid("inputs/d8_ex.txt");
        let antennas = find_antennas(&grid, 'A');
        println!("antennas: {antennas:?}");
        let res = find_antinodes(&grid, &antennas, false);

        println!("{res:?}");
        println!("{:?}", res.len());
        for (x, y) in res {
            println!("{x} {y}");
        }
    }
    #[test]
    fn test_ex1f() {
        let grid = read_input_to_grid("inputs/d8_ex.txt");
        let res = p1(&grid);
        assert_eq!(res, 14);
    }

    #[test]
    fn test_p1() {
        let grid = read_input_to_grid("inputs/input_d8.txt");
        let res = p1(&grid);
        assert_eq!(res, 278);
    }

    #[test]
    fn p2_ex1() {
        let anitnodes = vec![
            vec!['T', '.', '.', '.', '.', '#', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'T', '.', '.', '.', '.', '.', '.'],
            vec!['.', 'T', '.', '.', '.', '.', '#', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
            vec!['.', '.', '#', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '#', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];

        let find_antennas = find_antennas(&anitnodes, 'T');
        let antennas: Vec<(usize, usize)> = find_antennas;
        println!("{antennas:?}");

        let res = find_antinodes(&anitnodes, &antennas, true);

        println!("{res:?}");
        for (x, y) in res.iter() {
            println!("{x} {y}");
            let res = get_char_from_pos_grid(*x, *y, &anitnodes).expect("char");
            assert!(res == 'T' || res == '#');
        }
        assert_eq!(res.len() + 3, 9);
    }
    #[test]
    fn test_ex2f_manual() {
        let mut grid = read_input_to_grid("inputs/d8_ex.txt");
        let antennas = collect_antennas(&grid);
        println!("{antennas:?}");

        let mut c = 0;
        for (k, v) in antennas.iter() {
            let res = find_antinodes(&grid, &v, true);
            c += res.len();
            for (x, y) in res.iter() {
                if get_char_from_pos_grid(*x, *y, &grid) == Some('.') {
                    grid[*x][*y] = '#';
                }
            }
        }

        for g in grid.iter() {
            for c in g.iter() {
                print!("{c}");
            }
            println!("");
        }
        let antennas: HashSet<_> = antennas.values().flat_map(|v| v.iter().cloned()).collect();

        assert_eq!(c, 30);
    }

    #[test]
    fn test_p2ex() {
        let grid = read_input_to_grid("inputs/d8_ex.txt");
        let res = p2(&grid);
        println!("Res: {res:?}");
        assert_eq!(res, 34);
    }

    #[test]
    fn test_p2() {
        let grid = read_input_to_grid("inputs/input_d8.txt");
        let res = p2(&grid);
        // wrong 24066
        println!("Res: {res:?}");
        assert_eq!(res, 1067);
    }
}
