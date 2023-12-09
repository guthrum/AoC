use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Node {
    value: String,
    left: String,
    right: String,
}

impl Node {
    fn resolve(&self, instruction: &Instruction) -> &str {
        if *instruction == Instruction::Left {
            &self.left
        } else {
            &self.right
        }
    }

    fn is_starting(&self) -> bool {
        self.value.ends_with('A')
    }

    fn is_ending(&self) -> bool {
        self.value.ends_with('Z')
    }
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        let (id, parts) = value.split_once("=").unwrap();
        let binding = parts.replace('(', "").replace(')', "");
        let (left, right) = binding.split_once(",").unwrap();

        Self {
            value: id.trim().to_string(),
            left: left.trim().to_string(),
            right: right.trim().to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Left,
    Right,
}

impl From<char> for Instruction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!(),
        }
    }
}

fn read_input(input: &str) -> (Vec<Instruction>, HashMap<String, Node>) {
    let mut lines = input.lines();
    let instructions = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| Instruction::from(c))
        .collect();

    lines.next();
    let locations = lines
        .map(|line| Node::from(line))
        .map(|loc| (loc.value.clone(), loc))
        .collect();

    (instructions, locations)
}

fn run_cycle<'a>(
    raw_instructions: &[Instruction],
    locations: &'a HashMap<String, Node>,
    mut location: &'a Node,
    is_end: fn(&Node) -> bool,
) -> usize {
    let mut count = 0;
    let mut instructions = raw_instructions.iter().cycle();
    while !is_end(location) {
        let instruction = instructions.next().unwrap();
        location = locations.get(location.resolve(instruction)).unwrap();
        count += 1;
    }

    count
}

fn part1(raw_instructions: &[Instruction], locations: &HashMap<String, Node>) -> usize {
    run_cycle(
        raw_instructions,
        locations,
        locations.get("AAA").unwrap(),
        |n| n.value == "ZZZ",
    )
}

fn all_ending(nodes: &[&Node]) -> bool {
    !nodes.iter().find(|n| !n.is_ending()).is_some()
}

fn part2(raw_instructions: &[Instruction], locations: &HashMap<String, Node>) -> usize {
    let nodes: Vec<&Node> = locations
        .values()
        .filter(|node| node.is_starting())
        .collect();
    println!("running cycle with {} nodes", nodes.len());

    let cycle_lengths: Vec<usize> = nodes
        .iter()
        .map(|node| run_cycle(raw_instructions, &locations, &node, |n| n.is_ending()))
        .collect();
    println!("Part 2: compute LCM of {:?}", cycle_lengths);
    0
}

fn solve(input: &str) -> (usize, usize) {
    let (instructions, locations) = read_input(input);
    let p1 = part1(&instructions, &locations);
    let p2 = part2(&instructions, &locations);
    (p1, p2)
}

fn main() {
    let file_path = std::env::args().nth(1).unwrap();
    let (p1, _) = solve(&read_to_string(file_path).unwrap());
    println!("Part 1 = {}", p1);
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn example_input() {
        let input = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;
        assert_eq!(solve(input), (6, 0));
    }
}
