use core::cmp::PartialOrd;
use std::fs::read_to_string;

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
        for id_tuple in line.split(",") {
            let (id_1,id_2) = split_ids(id_tuple);
            res += check_range(id_1.parse().unwrap(), id_2.parse().unwrap())
        }
    }
    println!("{}", res);
}

fn split_ids(id_line: &str) -> (&str, &str) {
    let mut splitTuple: (&str, &str) = (&"", &"");
    for (i, c) in id_line.chars().enumerate() {
        match c {
            '-' => {
                splitTuple = (&id_line[0..i],&id_line[i+1..]);
            },
            _ => {

            }
        }
    }
    return splitTuple;
}

fn check_range(start: i64, end: i64) -> i64 {
    let mut res:i64 = 0;
    println!("Start {}, End {}", start, end);
    for i in start..end+1 {
        res += check_num(i);
    }
    println!("Res {}",res);
    return res;
}


fn check_num(mut num: i64) -> i64 {
    let mut values: Vec<i64> = Vec::new();
    while num > 0 {
        values.push(num % 10);
        num = num/10;
    }

    if values.len() % 2 != 0 {
        return 0;
    }

    if calc_value(&values[0..values.len() / 2]) == calc_value(&values[values.len() / 2..]) {
        println!("Equal");
        return calc_value(&values);
    }

    return 0;
}

fn calc_value(values: &[i64]) -> i64 {
    let mut value: i64 = 0;
    if values[values.len()-1] == 0 {
        return 0;
    }
    for i in 0..values.len(){
        value *= 10;
        value += values[values.len()-i-1];
    }
    return value;
}

