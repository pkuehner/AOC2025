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
    let mut matrix: Vec<Vec<i64>> = Vec::new();
    for line in lines.iter() {
        let mut row = Vec::new();
        for c in line.chars() {
            let val = match c {
                'S' => 1,
                '.' => 0,
                '^' => -1,
                _ => panic!("Cant be"),
            };
            row.push(val);
        }
        matrix.push(row);
    }
    //DONT RUN BOTH, Side Effects
    //println!("{}", solve_part_1(&mut matrix));
    println!("{}", solve_part_2(&mut matrix));
}

fn solve_part_1(matrix: &mut Vec<Vec<i64>>) -> i64 {
    let num_rows = matrix.len() as i32;
    let num_cols = matrix[0].len() as i32;
    let mut splits = 0;

    for y in 0..num_rows {
        for x in 0..num_cols {
            let pos = Position { x: x, y: y };
            if matrix[y as usize][x as usize] > 0 {
                let bottom = step(&Directions::D, &pos);
                if bottom.x >= 0 && bottom.x < num_cols && bottom.y >= 0 && bottom.y < num_rows {
                    if matrix[bottom.y as usize][bottom.x as usize] != -1 {
                        matrix[bottom.y as usize][bottom.x as usize] +=
                            matrix[y as usize][x as usize] // For 2
                    }
                }
            } else if matrix[y as usize][x as usize] == -1 {
                let left = step(&Directions::DL, &pos);
                let right = step(&Directions::DR, &pos);
                let top = step(&Directions::T, &pos);

                if matrix[top.y as usize][top.x as usize] != 1 {
                    continue;
                }
                splits += 1;

                for new_pos in vec![left, right] {
                    if new_pos.x >= 0
                        && new_pos.x < num_cols
                        && new_pos.y >= 0
                        && new_pos.y < num_rows
                    {
                        if matrix[new_pos.y as usize][new_pos.x as usize] != -1 {
                            matrix[new_pos.y as usize][new_pos.x as usize] = 1; //For 2
                        }
                    }
                }
            }
        }
    }

    //for x in 0..num_cols {
    //    value += matrix[(num_rows - 1) as usize][x as usize];
    //}

    return splits;
}

fn solve_part_2(matrix: &mut Vec<Vec<i64>>) -> i64 {
    let num_rows = matrix.len() as i32;
    let num_cols = matrix[0].len() as i32;
    let mut value = 0;

    for y in 0..num_rows {
        for x in 0..num_cols {
            let pos = Position { x: x, y: y };
            if matrix[y as usize][x as usize] > 0 {
                let bottom = step(&Directions::D, &pos);
                if bottom.x >= 0 && bottom.x < num_cols && bottom.y >= 0 && bottom.y < num_rows {
                    if matrix[bottom.y as usize][bottom.x as usize] != -1 {
                        matrix[bottom.y as usize][bottom.x as usize] +=
                            matrix[y as usize][x as usize]
                    }
                }
            } else if matrix[y as usize][x as usize] == -1 {
                let left = step(&Directions::DL, &pos);
                let right = step(&Directions::DR, &pos);
                let top = step(&Directions::T, &pos);

                if matrix[top.y as usize][top.x as usize] < 1 {
                    continue;
                }

                for new_pos in vec![left, right] {
                    if new_pos.x >= 0
                        && new_pos.x < num_cols
                        && new_pos.y >= 0
                        && new_pos.y < num_rows
                    {
                        if matrix[new_pos.y as usize][new_pos.x as usize] != -1 {
                            matrix[new_pos.y as usize][new_pos.x as usize] +=
                                matrix[top.y as usize][top.x as usize];
                        }
                    }
                }
            }
        }
    }

    for x in 0..num_cols {
        value += matrix[(num_rows - 1) as usize][x as usize];
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
