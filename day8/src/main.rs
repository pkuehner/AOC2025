use core::{clone::Clone};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fs::read_to_string,
    rc::Rc,
};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

#[derive(Clone, Debug)]
struct Node {
    x: i64,
    y: i64,
    z: i64,
    id: String,
}

#[derive(Clone, Debug)]
struct Edge {
    a_id: String,
    b_id: String,
    dist: f64,
}

fn main() {
    let lines: Vec<String> = read_lines("input_1");
    let mut nodes: Vec<Box<Node>> = Vec::new();
    let mut edges: Vec<Edge> = Vec::new();
    let mut id = 0;

    for line in lines.iter() {
        let split: Vec<&str> = line.split(",").collect();
        let node = Node {
            x: split[0].parse::<i64>().unwrap(),
            y: split[1].parse::<i64>().unwrap(),
            z: split[2].parse::<i64>().unwrap(),
            id: id.to_string(),
        };
        id += 1;
        nodes.push(Box::new(node));
    }

    for i in 0..nodes.len()-1 {
        for j in i+1..nodes.len() {
            let a = &nodes[i];
            let b = &nodes[j];
            if a.id == b.id {
                continue;
            }
            edges.push(Edge {
                a_id: a.id.clone(),
                b_id: b.id.clone(),
                dist: dist(&*a, &*b),
            });
        }
    }

    edges.sort_by(|x, y| x.dist.total_cmp(&y.dist));
    solve_part_1(&edges);
    solve_part_2(&edges, &nodes);

}

fn dist(a: &Node, b: &Node) -> f64 {
    let dx = (b.x - a.x) as f64;
    let dy = (b.y - a.y) as f64;
    let dz = (b.z - a.z) as f64;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn solve_part_1(edges: &[Edge]) {
    let mut edge_hm: HashMap<String, Rc<RefCell<HashSet<String>>>> = HashMap::new();
    let mut count = 0;
    for edge in edges {
        if count >= 1000 {
            break;
        }
        
        let a_in = edge_hm.contains_key(&edge.a_id);
        let b_in = edge_hm.contains_key(&edge.b_id);

        if a_in && b_in {
            let set_a = edge_hm[&edge.a_id].clone();
            let set_b = edge_hm[&edge.b_id].clone();

            if Rc::ptr_eq(&set_a, &set_b) {
                count += 1;
                continue;
            }

            let b_ids: Vec<String> = set_b.borrow().iter().cloned().collect();
            for id in b_ids {
                set_a.borrow_mut().insert(id.clone());
                edge_hm.insert(id, set_a.clone());
            }
        } else if a_in {
            let set_a = edge_hm[&edge.a_id].clone();
            set_a.borrow_mut().insert(edge.b_id.clone());
            edge_hm.insert(edge.b_id.clone(), set_a);
        } else if b_in {
            let set_b = edge_hm[&edge.b_id].clone();
            set_b.borrow_mut().insert(edge.a_id.clone());
            edge_hm.insert(edge.a_id.clone(), set_b);
        } else {
            let mut hs = HashSet::new();
            hs.insert(edge.a_id.clone());
            hs.insert(edge.b_id.clone());
            let rc = Rc::new(RefCell::new(hs));
            edge_hm.insert(edge.a_id.clone(), rc.clone());
            edge_hm.insert(edge.b_id.clone(), rc);
        }

        count += 1;
    }

    let mut unique_sets = HashSet::new();
    let mut sizes = Vec::new();

    for hm in edge_hm.values() {
        let ptr = Rc::as_ptr(hm);
        if unique_sets.insert(ptr) {
            sizes.push(hm.borrow().len());
        }
    }

    sizes.sort_by(|a, b| b.cmp(a));
    println!("Top 3 sizes: {:?}", sizes.iter().take(3).collect::<Vec<_>>());
}

fn solve_part_2(edges: &[Edge], nodes: &Vec<Box<Node>>) {
    let mut edge_hm: HashMap<String, Rc<RefCell<HashSet<String>>>> = HashMap::new();
    for edge in edges {
        let a_in = edge_hm.contains_key(&edge.a_id);
        let b_in = edge_hm.contains_key(&edge.b_id);

        if a_in && b_in {
            let set_a = edge_hm[&edge.a_id].clone();
            let set_b = edge_hm[&edge.b_id].clone();

            if Rc::ptr_eq(&set_a, &set_b) {
                continue;
            }

            let b_ids: Vec<String> = set_b.borrow().iter().cloned().collect();
            for id in b_ids {
                set_a.borrow_mut().insert(id.clone());
                edge_hm.insert(id, set_a.clone());
            }
            if edge_hm.len() == nodes.len() {
                println!("Last Edge {} - {}", &edge.a_id, &edge.b_id);
                break;
            }
        } else if a_in {
            let set_a = edge_hm[&edge.a_id].clone();
            set_a.borrow_mut().insert(edge.b_id.clone());
            edge_hm.insert(edge.b_id.clone(), set_a);
            if edge_hm.len() == nodes.len() {
                println!("Last Edge {} - {}", &edge.a_id, &edge.b_id);
                break;
            }
        } else if b_in {
            let set_b = edge_hm[&edge.b_id].clone();
            set_b.borrow_mut().insert(edge.a_id.clone());
            edge_hm.insert(edge.a_id.clone(), set_b);
            if edge_hm.len() == nodes.len() {
                println!("Last Edge {} - {}", &edge.a_id, &edge.b_id);
                break;
            }
        } else {
            let mut hs = HashSet::new();
            hs.insert(edge.a_id.clone());
            hs.insert(edge.b_id.clone());
            let rc = Rc::new(RefCell::new(hs));
            edge_hm.insert(edge.a_id.clone(), rc.clone());
            edge_hm.insert(edge.b_id.clone(), rc);
        }
    }

    let mut unique_sets = HashSet::new();
    let mut sizes = Vec::new();

    for hm in edge_hm.values() {
        let ptr = Rc::as_ptr(hm);
        if unique_sets.insert(ptr) {
            sizes.push(hm.borrow().len());
        }
    }

    sizes.sort_by(|a, b| b.cmp(a));
}