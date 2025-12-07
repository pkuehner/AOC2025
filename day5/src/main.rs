use std::{collections::btree_map::Values, fs::read_to_string};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

#[derive(Debug, Eq, PartialEq)]
struct Range {
    start: i64,
    end: i64,
}

fn main() {
    let lines: Vec<String> = read_lines("input_1");
    let mut is_ranges = true;
    let mut ranges: Vec<Range> = Vec::new();
    let mut nums: Vec<i64> = Vec::new();
    for line in lines.iter() {
        if line == "" {
            is_ranges = false;
            continue;
        }

        if is_ranges {
            let split: Vec<&str> = line.split("-").collect();
            let range = Range {
                start: split[0].parse::<i64>().unwrap(),
                end: split[1].parse::<i64>().unwrap(),
            };

            ranges.push(range);
        } else {
            nums.push(line.parse::<i64>().unwrap());
        }
    }

    ranges.sort_by_key(|range| range.start);

    solve_part_1(&nums, &ranges);
    solve_part_2(&ranges);
}

fn solve_part_1(nums: &[i64], ranges: &[Range]) {
    let mut result = 0;

    for num in nums.iter() {
        let mut found = false;
        for range in ranges.iter() {
            //println!("{:?} {:?}", num, range);
            if num > &range.end {
                continue;
            }
            if num < &range.start {
                break;
            }
            result += 1;
            break;
        }
    }

    println!("{}", result)
}

fn solve_part_2(ranges: &[Range]) {
    let mut result = 0;
    let mut last_range: Option<&Range> = None;

    for range in ranges.iter() {
        match last_range {
            Some(x) => {
                if range.start <= x.end && range.end <= x.end {
                    continue;
                }
                if range.start <= x.end {
                    result += range.end - x.end;
                } else {
                    result += range.end - range.start + 1
                };
            }
            None => {
                result += range.end - range.start + 1;
            }
        }
        last_range = Some(range);
        println!("{:?} {:?}", result, range)
    }

    println!("{}", result)
}
