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
    solve_part_1(&lines);
}

fn solve_part_1(lines: &[String]) {
    let mut equations: Vec<Vec<i64>> = Vec::new();
    let mut result: i64 = 0;
    for line in lines.iter() {
        let split: Vec<&str> = line.split_whitespace().collect();

        //First one
        if equations.len() == 0 {
            for _ in 0..split.len() {
                equations.push(Vec::new());
            }
        }
        let mut counter = 0;

        for value in split.iter() {
            println!("{}", value);
            if *value == "*" {
                result += equations[counter]
                    .iter()
                    .copied()
                    .reduce(|res, value| res * value)
                    .unwrap();
            } else if *value == "+" {
                result += equations[counter].iter().sum::<i64>();
            } else {
                equations[counter].push(value.parse::<i64>().unwrap());
            }
            counter += 1;
        }
    }

    println!("{}", result)
}
