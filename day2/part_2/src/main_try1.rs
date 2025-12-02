use std::fs::read_to_string;
use std::time::Instant;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn main() {
    let start = Instant::now();
    let lines: Vec<String> = read_lines("input_1");
    println!("Time elapsed reading: {:?}", start.elapsed());

    let mut res: i64 = 0;
    for line in lines.iter() {
        for id_tuple in line.split(",") {
            let (id_1,id_2) = split_ids(id_tuple);
            res += check_range(id_1.parse().unwrap(), id_2.parse().unwrap())
        }
    }
    println!("Result: {}", res);
    println!("Time elapsed: {:?}", start.elapsed());
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
    for i in start..end+1 {
        res += check_num(i);
    }
    return res;
}


fn check_num(num_org: i64) -> i64 {
    let mut values: Vec<i64> = Vec::new();
    let mut num = num_org;
    while num > 0 {
        values.push(num % 10);
        num = num/10;
    }

    for i in 1..values.len() / 2 +1 {
        if values.len() % i != 0 {
            continue;
        }

        let mut start = i;
        let mut stop: bool = false;
        while !stop && start < values.len() {
            for j in 0..i {
                if values[j] != values[start+j] {
                    stop = true;
                    break;
                }
            }
            
            start += i;
        }

        if !stop {
            return num_org;
        }

    }

    return 0;
}

