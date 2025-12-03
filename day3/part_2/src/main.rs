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
        let mut digits: Vec<i64> = Vec::new();
        let mut value = 0;
        for c in line.chars() {
            let digit = (c.to_string()).parse::<i64>().unwrap();
            digits.push(digit);
        }
        let mut max_pos: usize = 0;

        for i in 0..12 {
            let mut max = -1;
            for j in max_pos ..(digits.len() -(12-i-1)){
                if digits[j] > max {
                    max = digits[j];
                    max_pos = j+1;
                }
                if max == 9 {
                    break;
                }
            }
            value *= 10;
            value += max;
        }
        res += value;
        println!("{}", value);

    }
    println!("{}", res);
}
