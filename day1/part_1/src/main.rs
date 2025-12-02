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
    let mut val: i64 = 50;
    let mut result = 0;
    for line in lines.iter() {
        let line_val = line[1..].parse::<i64>().unwrap();
        match &line[0..1] {
            "R" => val += line_val,
            "L" => val -= line_val,
            &_ => panic!("Must not happen"),
        }

        val = val % 100;
        if val == 0 {
            result += 1;
        }
    }
    println!("{}", result);


}
