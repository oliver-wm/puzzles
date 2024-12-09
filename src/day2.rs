use std::{
    fs::File,
    io::{self, BufRead},
};

#[derive(Debug, PartialEq)]
enum Report {
    Safe,
    Unsafe,
}

fn find_safe(l1: &Vec<i64>, skip_index: Option<usize>) -> Report {
    let mut increasing = false;
    let mut decreasing = false;

    let mut i = 0;
    while i < l1.len() - 1 {
        if let Some(skip) = skip_index {
            if i == skip {
                i += 1;
                continue;
            }
        }

        let next_index = if let Some(skip) = skip_index {
            if i + 1 == skip {
                i + 2
            } else {
                i + 1
            }
        } else {
            i + 1
        };

        if next_index >= l1.len() {
            break;
        }

        let diff = l1[i] - l1[next_index];
        if diff < 0 {
            increasing = true;
        } else if diff > 0 {
            decreasing = true;
        }

        if !(diff.abs() >= 1 && diff.abs() <= 3) {
            return Report::Unsafe;
        }

        i += 1;
    }

    if increasing && decreasing {
        return Report::Unsafe;
    }

    Report::Safe
}

fn count_safe_dampen(reports: Vec<Vec<i64>>) -> i64 {
    reports
        .iter()
        .map(|report| {
            if let Report::Safe = find_safe(report, None) {
                1
            } else {
                (0..report.len())
                    .find_map(|i| match find_safe(report, Some(i)) {
                        Report::Safe => Some(1),
                        Report::Unsafe => None,
                    })
                    .unwrap_or(0)
            }
        })
        .sum()
}

fn count_safe(reports: Vec<Vec<i64>>) -> i64 {
    let safe_counts = reports
        .iter()
        .map(|report| match find_safe(report, None) {
            Report::Safe => 1,
            Report::Unsafe => 0,
        })
        .sum();

    safe_counts
}

fn input_to_vec() -> Result<Vec<Vec<i64>>, io::Error> {
    let file_path = "inputs/input_d2.txt";

    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut reports: Vec<Vec<i64>> = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let numbers: Vec<i64> = line
                .split_whitespace()
                .filter_map(|num| num.parse::<i64>().ok())
                .collect();
            if !numbers.is_empty() {
                reports.push(numbers);
            }
        }
    }

    Ok(reports)
}

#[cfg(test)]
mod tests {
    use crate::day2::{count_safe, count_safe_dampen, find_safe, input_to_vec, Report};

    #[test]
    fn test_count_safe_w_dampen() {
        let reports = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        assert_eq!(count_safe_dampen(reports), 4);
    }

    #[test]
    fn test_safe_dampen() {
        let safe = vec![7, 6, 4, 2, 1];

        assert_eq!(find_safe(&safe, None), Report::Safe);

        let un_safe = vec![1, 2, 7, 8, 9];
        for i in 0..un_safe.len() {
            assert_eq!(
                find_safe(&un_safe, Some(i)),
                Report::Unsafe,
                "index {} is safe",
                i
            ); // For all skip indexes this is unsafe
        }

        let un_safe = vec![9, 7, 6, 2, 1];
        for i in 0..un_safe.len() {
            assert_eq!(find_safe(&un_safe, Some(i)), Report::Unsafe); // For all skip indexes this is unsafe
        }

        let un_safe = vec![1, 3, 2, 4, 5];

        assert_eq!(find_safe(&un_safe, Some(1)), Report::Safe);

        let un_safe = vec![8, 6, 4, 4, 1];

        assert_eq!(find_safe(&un_safe, Some(2)), Report::Safe);

        let safe = vec![1, 3, 6, 7, 9];

        assert_eq!(find_safe(&safe, None), Report::Safe);
    }

    #[test]
    fn test_safe() {
        let safe = vec![7, 6, 4, 2, 1];

        assert_eq!(find_safe(&safe, None), Report::Safe);

        let un_safe = vec![1, 2, 7, 8, 9];
        assert_eq!(find_safe(&un_safe, None), Report::Unsafe);

        let un_safe = vec![9, 7, 6, 2, 1];

        assert_eq!(find_safe(&un_safe, None), Report::Unsafe);

        let un_safe = vec![1, 3, 2, 4, 5];

        assert_eq!(find_safe(&un_safe, None), Report::Unsafe);

        let un_safe = vec![8, 6, 4, 4, 1];

        assert_eq!(find_safe(&un_safe, None), Report::Unsafe);

        let safe = vec![1, 3, 6, 7, 9];

        assert_eq!(find_safe(&safe, None), Report::Safe);
    }

    #[test]
    fn test_count_safe() {
        let reports = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        assert_eq!(count_safe(reports), 2);
    }

    #[test]
    fn test_p1_answer() {
        let reports = input_to_vec().expect("input error");
        println!("Number reports: {}", reports.len());

        println!("Safe reports: {}", count_safe(reports))
    }

    #[test]
    fn test_p2_answer() {
        let reports = input_to_vec().expect("input error");
        println!("Number reports: {}", reports.len());

        println!("Safe reports: {}", count_safe_dampen(reports))
    }
}
