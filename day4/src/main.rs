use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

#[derive(Debug)]
enum Directions {
    T,
    TR,
    R,
    DR,
    D,
    DL,
    L,
    TL,
}
#[derive(Clone)]
struct Position {
    x: i32,
    y: i32,
}

fn main() {
    let lines: Vec<String> = read_lines("input_1");
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    for line in lines.iter() {
        let mut row = Vec::new();
        for c in line.chars() {
            let val = match c {
                '@' => 1,
                '.' => 0,
                _ => panic!("Cant be"),
            };
            row.push(val);
        }
        matrix.push(row);
    }
    println!("{}", solve_part_1(&matrix));
    println!("{}", solve_part_2(&mut matrix));
}

fn solve_part_1(matrix: &Vec<Vec<i32>>) -> i32 {
    let num_rows = matrix.len() as i32;
    let num_cols = matrix[0].len() as i32;
    let directions = vec![
        Directions::T,
        Directions::TR,
        Directions::R,
        Directions::DR,
        Directions::D,
        Directions::DL,
        Directions::L,
        Directions::TL,
    ];

    let mut value = 0;

    for y in 0..num_rows {
        for x in 0..num_cols {
            let mut papers_around = 0;

            let pos = Position { x: x, y: y };
            if matrix[y as usize][x as usize] == 0 {
                continue;
            };
            for direction in directions.iter() {
                let new_pos = step(direction, &pos);
                if new_pos.x >= 0 && new_pos.x < num_rows {
                    if new_pos.y >= 0 && new_pos.y < num_cols {
                        papers_around += matrix[new_pos.y as usize][new_pos.x as usize];
                    }
                }
            }
            if papers_around < 4 {
                value += 1;
            }
        }
    }

    return value;
}

fn solve_part_2(matrix: &mut Vec<Vec<i32>>) -> i32 {
    let num_rows = matrix.len() as i32;
    let num_cols = matrix[0].len() as i32;
    let directions = vec![
        Directions::T,
        Directions::TR,
        Directions::R,
        Directions::DR,
        Directions::D,
        Directions::DL,
        Directions::L,
        Directions::TL,
    ];

    let mut value = 0;
    let mut value_changed = true;

    while value_changed {
        value_changed = false;
        let mut pos_to_rm: Vec<Position> = Vec::new();
        for y in 0..num_rows {
            for x in 0..num_cols {
                let mut papers_around = 0;

                let pos = Position { x: x, y: y };
                if matrix[y as usize][x as usize] == 0 {
                    continue;
                };
                for direction in directions.iter() {
                    let new_pos = step(direction, &pos);
                    if new_pos.x >= 0 && new_pos.x < num_rows {
                        if new_pos.y >= 0 && new_pos.y < num_cols {
                            papers_around += matrix[new_pos.y as usize][new_pos.x as usize];
                        }
                    }
                }
                if papers_around < 4 {
                    value += 1;
                    pos_to_rm.push(pos);
                    value_changed = true;
                }
            }
        }

        for pos_rm in pos_to_rm.iter() {
            matrix[pos_rm.y as usize][pos_rm.x as usize] = 0;
        }
        
    }

    return value;
}

fn step(dir: &Directions, pos: &Position) -> Position {
    match dir {
        Directions::T => Position {
            x: pos.x,
            y: pos.y - 1,
        },
        Directions::TR => Position {
            x: pos.x + 1,
            y: pos.y - 1,
        },
        Directions::R => Position {
            x: pos.x + 1,
            y: pos.y,
        },
        Directions::DR => Position {
            x: pos.x + 1,
            y: pos.y + 1,
        },
        Directions::D => Position {
            x: pos.x,
            y: pos.y + 1,
        },
        Directions::DL => Position {
            x: pos.x - 1,
            y: pos.y + 1,
        },
        Directions::L => Position {
            x: pos.x - 1,
            y: pos.y,
        },
        Directions::TL => Position {
            x: pos.x - 1,
            y: pos.y - 1,
        },
    }
}
