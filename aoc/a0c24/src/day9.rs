#![allow(dead_code)]

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn block_pattern(disk: String) -> Vec<String> {
    let mut pattern: Vec<String> = Vec::new();
    let mut count: usize = 0;
    for (i, char) in disk.chars().enumerate() {
        if let Some(n) = char.to_digit(10) {
            if i % 2 == 1 {
                for _ in 0..n {
                    pattern.push(".".to_string());
                }
            } else {
                for _ in 0..n {
                    pattern.push(count.to_string());
                }
                count += 1;
            }
        } else {
            println!("Encountered non-digit in loop");
            continue;
        }
    }
    pattern
}

fn free_space(block: Vec<String>) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    let chars = block.iter().filter(|c| **c != ".").count();
    let mut s = 0;
    let mut e = block.len() - 1;

    while s < block.len() {
        if res.len() == chars {
            break;
        }
        if block[s] != "." {
            res.push(block[s].clone());
        } else {
            while block[e] == "." {
                e -= 1;
            }
            res.push(block[e].clone());
            e -= 1;
        }
        s += 1;
    }
    res.extend(
        (0..(block.len() - res.len()))
            .map(|_| ".".to_string()),
    );
    res
}

struct Block {
    start: usize,
    size: usize,
    id: Option<usize>,
}

/// find first empty from left
fn defrag_helper(idx: usize, size: usize, blocks: &[Block]) -> Option<usize> {
    blocks
        .iter()
        .enumerate()
        .take(idx)
        .skip(1)
        .find(|(_, b)| b.id.is_none() && b.size >= size)
        .map(|(i, _)| i)
}

fn defrag(blocks: &mut [Block]) {
    for i in (0..blocks.len()).rev() {
        if blocks[i].id.is_some() {
            if let Some(idx) = defrag_helper(i, blocks[i].size, blocks) {
                let move_size = blocks[i].size;
                let new_start = blocks[idx].start;

                blocks[i].start = new_start;
                blocks[idx].start += move_size;
                blocks[idx].size -= move_size;
            }
        }
    }
}

fn checksum(block: Vec<String>) -> usize {
    block
        .iter()
        .enumerate()
        .map(|(i, c)| {
            if let Ok(val) = c.parse::<usize>() {
                return (i, val);
            }
            (i, 0)
        })
        .map(|(i, c)| i * c)
        .sum()
}

fn checksum_blocks(blocks: &[Block]) -> usize {
    blocks
        .iter()
        .filter_map(|block| block.id.map(|id| (block, id)))
        .map(|(block, id)| {
            (block.start..block.start + block.size)
                .map(|pos| pos * id)
                .sum::<usize>()
        })
        .sum()
}

fn read_blocks_string(input_path: &str) -> String {
    let file = File::open(input_path).expect("Should read");
    let mut reader = BufReader::new(file);
    let mut disk = String::new();
    reader.read_line(&mut disk).expect("read to string");
    disk
}

fn p1(fs: &str) -> usize {
    checksum(free_space(block_pattern(read_blocks_string(fs))))
}

fn read_blocks<P: AsRef<std::path::Path>>(input_path: P) -> io::Result<Vec<Block>> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let line = reader
        .lines()
        .map(|line| line.expect("could not read line"))
        .next()
        .expect("no lines in file");
    let nums: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
    let mut fs = Vec::with_capacity(nums.len());
    let mut start_idx = 0;
    for (i, &num) in nums.iter().enumerate() {
        fs.push(Block {
            start: start_idx,
            size: num as usize,
            id: if i % 2 == 0 { Some(i / 2) } else { None },
        });
        start_idx += num as usize;
    }
    Ok(fs)
}

fn p2(fs: &str) -> usize {
    let mut blocks = read_blocks(fs).expect("failed to collect blocks");
    defrag(&mut blocks);
    checksum_blocks(&mut blocks)
}

#[cfg(test)]
mod tests {

    use crate::day9::*;

    #[test]
    fn test_p1() {
        let block_pattern = p1("inputs/input_d9.txt");
        println!("Res: {block_pattern:?}");
        assert_eq!(block_pattern, 6398608069280);
    }

    #[test]
    fn test_p1ex() {
        let block_pattern = p1("inputs/input_d9ex.txt");
        println!("Res: {block_pattern:?}");
        assert_eq!(block_pattern, 1928);
    }

    #[test]
    fn test_p2() {
        let block_pattern = p2("inputs/input_d9.txt");
        println!("Res: {block_pattern:?}");
        assert_eq!(block_pattern, 6427437134372);
    }

    #[test]
    fn test_p2ex() {
        let block_pattern = p2("inputs/input_d9ex.txt");
        println!("Res: {block_pattern:?}");
        assert_eq!(block_pattern, 2858);
    }
}
