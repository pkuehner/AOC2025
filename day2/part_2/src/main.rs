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
            let nums: Vec<&str> = id_tuple.split("-").collect();
            res += check_range(nums[0].parse().unwrap(), nums[1].parse().unwrap())
        }
    }
    println!("Result: {}", res);
    println!("Time elapsed: {:?}", start.elapsed());
}
fn check_range(start: i64, end: i64) -> i64 {
    let mut res:i64 = 0;
    for i in start..end+1 {
        res += check_num(i);
    }
    return res;
}


fn check_num(num_org: i64) -> i64 {
    for i in 1..num_org {
        let exp = i as u32;
        let divisor = i64::pow(10, exp);
        let mut num_div = num_org;
        let pattern = num_div % divisor;
        let mut stop = false;
        if num_div <= divisor {
            return 0;
        }
        if pattern < divisor/10 {
            continue;
        }
        num_div /= divisor;
        while num_div > 0 {
            if num_div % divisor != pattern {
                stop = true;
                break;
            }
            num_div /= divisor;
        }

        if !stop {
            return num_org;
        }

    }

    return 0;
}

