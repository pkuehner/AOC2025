use core::{clone::Clone, num, option::Option::None};
use std::{collections::HashMap, fs::read_to_string};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

#[derive(Clone, Debug)]
struct Position {
    x: i64,
    y: i64,
}
#[derive(Clone, Debug, PartialEq)]
enum Direction {
    X,
    Y,
}

#[derive(Clone, Debug)]
struct Range {
    start: Position,
    end: Position,
    dir: Direction,
}

fn main() {
    let lines: Vec<String> = read_lines("input_1");
    let mut positions: Vec<Position> = Vec::new();
    for line in lines.iter() {
        let split: Vec<&str> = line.split(",").collect();
        let x = split[0].to_string().parse::<i64>().unwrap();
        let y = split[1].to_string().parse::<i64>().unwrap();
        positions.push(Position { x: x, y: y })
    }

    println!("{}", solve_part_1(&positions));
    println!("{}", solve_part_2(&positions));
}

fn solve_part_1(positions: &[Position]) -> i64 {
    let mut max: i64 = 0;
    for a in positions.iter() {
        for b in positions.iter() {
            let size = ((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1);
            if size >= max {
                max = size;
                println!("{:?} {:?} {}", a, b, size)
            }
        }
    }
    return max;
}

fn solve_part_2(positions: &[Position]) -> i64 {
    let mut marked: Vec<Range> = Vec::new();

    let mut max_x = 0;
    let mut max_y = 0;

    for i in 0..positions.len() {
        let pos = &positions[i];
        if pos.x > max_x {
            max_x = pos.x;
        }

        if pos.y > max_y {
            max_y = pos.y;
        }
        let next_pos = &positions[(i + 1) % positions.len()];

        let mut start = pos;
        let mut end = next_pos;
        let mut dir = Direction::X;
        if pos.x == next_pos.x {
            dir = Direction::Y;
            if start.y > end.y {
                let tmp_start = start;
                start = end;
                end = tmp_start;
            }
        } else {
            if start.x > end.x {
                let tmp_start = start;
                start = end;
                end = tmp_start;
            }
        }

        marked.push(Range {
            start: start.clone(),
            end: end.clone(),
            dir: dir,
        })
    }

    //Initial square done;
    let mut max: i64 = 0;
    for a in positions.iter() {
        for b in positions.iter() {
            let mut x = a.x;
            let mut y = b.y;
            let mut found = false;

            for range in &marked {
                if range.dir == Direction::Y {
                    let mut range_correct = false;

                    if b.x >= a.x && range.start.x <= x {
                        range_correct = true;
                    } else if b.x <= a.x && range.start.x >= x {
                        range_correct = true;
                    }
                    if range.start.y <= y && range.end.y >= y && range_correct {
                        found = true;
                        break;
                    }
                }
            }

            if !found {
                continue;
            }

            x = b.x;
            y = a.y;
            found = false;

            for range in &marked {
                if range.dir == Direction::X {
                    let mut range_correct = false;
                    if b.y >= a.y && range.start.y <= y {
                        range_correct = true;
                    } else if b.y <= a.y && range.start.y >= y {
                        range_correct = true;
                    }
                    if range.start.x <= x && range.end.x >= x && range_correct {
                        found = true;
                        break;
                    }
                }
            }

            if !found {
                continue;
            }

            let size = ((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1);
            if size >= max {
                max = size;
                println!("{:?} {:?} {}", a, b, size)
            }
        }
    }
    return max;
}
