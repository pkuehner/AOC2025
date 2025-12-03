use core::cmp::PartialOrd;
use std::{collections::hash_set::Difference, fs::read_to_string};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn main() {
    let lines: Vec<String> = read_lines("input_1");
    let mut res: i64 = 0;
    for line in lines.iter() {
        let mut max_first = -1;
        let mut max_second = -1;
        for (i, c) in line.chars().enumerate() {
            let digit = (c.to_string()).parse::<i64>().unwrap();
            if i != line.len()-1 && digit > max_first {
                max_first = digit;
                max_second = -1;
            }
            else if digit > max_second {
                max_second = digit;
            }
        }
        println!("{}{}", max_first, max_second);

        res += max_first * 10 + max_second;
    }
    println!("{}", res);
}
