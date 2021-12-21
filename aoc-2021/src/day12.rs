use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

const START: &str = "start";
const END: &str = "end";

struct Graph {
    adjacency_map: HashMap<String, HashSet<String>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            adjacency_map: HashMap::new(),
        }
    }

    fn insert(&mut self, source: String, destination: String) {
        self.adjacency_map
            .entry(source.clone())
            .or_insert(HashSet::new())
            .insert(destination.clone());
        self.adjacency_map
            .entry(destination)
            .or_insert(HashSet::new())
            .insert(source);
    }

    fn adjacent(&self, node: &str) -> Option<&HashSet<String>> {
        self.adjacency_map.get(node)
    }
}

fn read_input(path: &str) -> Graph {
    let initial_states: Vec<(String, String)> = read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split('-');
            let source = parts.next().unwrap().to_string();
            let dest = parts.next().unwrap().to_string();
            (source, dest)
        })
        .collect();
    let mut graph = Graph::new();
    for (source, dest) in initial_states {
        graph.insert(source, dest)
    }
    graph
}

fn is_lowercase(s: &str) -> bool {
    s.to_lowercase() == s
}

#[derive(Debug, Clone)]
struct Path {
    can_visit_small: bool,
    path: HashSet<String>,
    small: Option<String>,
}

impl Path {
    fn new(can_visit_small: bool) -> Self {
        Self {
            can_visit_small,
            path: HashSet::new(),
            small: None,
        }
    }

    fn visit(&mut self, node: &str) {
        if is_lowercase(node) && self.path.contains(node) {
            if self.small.is_some() || !self.can_visit_small {
                panic!("cannot visit {:?} ++ {}", self, node);
            }
            self.small = Some(node.to_string());
        }
        self.path.insert(node.to_string());
    }

    fn can_visit(&self, node: &str) -> bool {
        if node == START {
            return false;
        }

        if !self.can_visit_small {
            if is_lowercase(node) {
                !self.path.contains(node)
            } else {
                true
            }
        } else if !is_lowercase(node) {
            true
        } else if self.path.contains(node) {
            self.small.is_none()
        } else {
            true
        }
    }
}

fn find_end(from: &str, graph: &Graph, mut current_path: Path) -> usize {
    if from == END {
        return 1;
    }
    current_path.visit(from);
    let mut count = 0;
    for neighbour in graph.adjacent(from).unwrap() {
        if neighbour == from {
            panic!("cycle");
        }
        if current_path.can_visit(neighbour) {
            count += find_end(neighbour, graph, current_path.clone());
        }
    }
    count
}

fn solve(graph: Graph) -> (usize, usize) {
    let p1 = find_end(START, &graph, Path::new(false));
    let p2 = find_end(START, &graph, Path::new(true));

    (p1, p2)
}

fn main() {
    let file_path = std::env::args().nth(1).unwrap();
    let input = read_input(&file_path);
    let (p1, p2) = solve(input);
    println!("Part 1 = {}", p1);
    println!("Part 2 = {}", p2);
}
