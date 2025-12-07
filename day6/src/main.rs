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
    solve_part_2(&lines);
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

fn solve_part_2(lines: &[String]) {
    let mut nums: Vec<i64> = Vec::new();
    let mut char_matrix: Vec<Vec<char>> = Vec::new();
    for line in lines.iter() {
        char_matrix.push(line.chars().collect());
    }

    let mut result: i64 = 0;
    let mut col = (char_matrix[0].len() - 1) as i32;
    while col >= 0 {
        let mut num = 0;
        let mut row = 0;

        while row < char_matrix.len() {
            let char = char_matrix[row][col as usize];

            if char == '*' {
                nums.push(num);
                result += nums
                    .iter()
                    .copied()
                    .reduce(|res, value| res * value)
                    .unwrap();
                println!("{}", result);

                nums.clear();
                col -= 1;
            } else if char == '+' {
                nums.push(num);
                result += nums.iter().sum::<i64>();
                nums.clear();
                println!("{}", result);
                col -= 1;
            } else {
                if row == char_matrix.len() - 1 {
                    if num != 0 {
                        nums.push(num);
                    }
                    col -= 1;
                } else if char != ' ' {
                    num *= 10;
                    num += char.to_digit(10).unwrap() as i64;
                }
            }
            row += 1;
        }
    }
    println!("{}", result)
}
