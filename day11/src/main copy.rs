use core::clone::Clone;
use core::option::Option;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn parse_input(lines: &[String]) -> HashMap<String, Vec<String>> {
    let mut result = HashMap::new();
    for line in lines {
        if let Some((start, targets)) = line.split_once(": ") {
            let target_list: Vec<String> =
                targets.split_whitespace().map(|s| s.to_string()).collect();
            result.insert(start.to_string(), target_list);
        }
    }
    result
}

#[derive(Clone, Debug)]
struct Path {
    current: String,
    path: HashSet<String>,
}

struct Queue {
    data: Vec<Path>,
}

impl Queue {
    fn new() -> Self {
        Queue { data: Vec::new() }
    }

    fn enqueue(&mut self, item: Path) {
        self.data.push(item);
    }

    fn dequeue(&mut self) -> Option<Path> {
        self.data.pop()
    }

    fn peek(&self) -> Option<&Path> {
        self.data.first()
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

fn main() {
    let lines: Vec<String> = read_lines("input_1");
    println!("{}", solve_part_1(lines.clone()));
    println!("{}", solve_part_2(lines));
}

fn solve_part_1(lines: Vec<String>) -> i64 {
    let hs = parse_input(&lines);
    let mut count = 0;
    let mut hs: HashMap<str>
    let mut queue: Queue = Queue::new();
    queue.enqueue(Path {
        current: "you".to_string(),
        path: HashSet::new(),
    });

    while !queue.is_empty() {
        let mut path = queue.dequeue().unwrap();
        if path.current == "out" {
            count += 1;
            continue;
        }

        let nbs = hs.get(&path.current).unwrap();
        path.path.insert(path.current);

        for nb in nbs {
            if !path.path.contains(nb) {
                let new_path = path.path.clone();
                queue.enqueue(Path {
                    current: nb.to_string(),
                    path: new_path,
                });
            }
        }
    }

    return count;
}

fn solve_part_2(lines: Vec<String>) -> i64 {
    let hs = parse_input(&lines);
    let mut count = 0;
    let mut queue: Queue = Queue::new();
    queue.enqueue(Path {
        current: "svr".to_string(),
        path: HashSet::new(),
    });

    while !queue.is_empty() {
        let mut path = queue.dequeue().unwrap();
        if path.current == "out" {
            if path.path.contains("fft") && path.path.contains("dac") {
                count += 1;
                println!("{}", count)
            }
            continue;
        }

        let nbs = hs.get(&path.current).unwrap();
        path.path.insert(path.current);

        for nb in nbs {
            if !path.path.contains(nb) {
                let new_path = path.path.clone();
                queue.enqueue(Path {
                    current: nb.to_string(),
                    path: new_path,
                });
            }
        }
    }

    return count;
}
