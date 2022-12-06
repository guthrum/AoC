use std::fs::read_to_string;

fn parse_number(l: &str) -> (usize, &str) {
    if let Some(idx) = l.find(" ") {
        // println!("'{}' '{}' '{}'", l, idx, &l[0..idx]);
        let u = usize::from_str_radix(&l[0..idx], 10).unwrap();
        (u, &l[idx..])
    } else {
        (usize::from_str_radix(&l, 10).unwrap(), "")
    }
}

struct Instruction {
    amount: usize,
    source: usize,
    dest: usize,
}

impl From<&str> for Instruction {
    fn from(mut l: &str) -> Self {
        l = l.trim_start_matches("move ");
        let (amount, mut l) = parse_number(l);
        l = l.trim_start_matches(" from ");
        let (source, mut l) = parse_number(l);
        l = l.trim_start_matches(" to ");
        let (dest, _) = parse_number(l);

        Self {
            amount,
            source,
            dest,
        }
    }
}

#[derive(Debug, Clone)]
struct Stack {
    letters: Vec<Vec<char>>,
}

impl Stack {
    fn handle_1(&mut self, instruction: &Instruction) {
        for i in 0..instruction.amount {
            let c = self
                .letters
                .get_mut(instruction.source - 1)
                .unwrap()
                .pop()
                .unwrap();
            self.letters.get_mut(instruction.dest - 1).unwrap().push(c);
        }
    }

    fn handle_2(&mut self, instruction: &Instruction) {
        let mut temp = Vec::new();
        for i in 0..instruction.amount {
            let c = self
                .letters
                .get_mut(instruction.source - 1)
                .unwrap()
                .pop()
                .unwrap();
            temp.push(c);
        }
        temp.reverse();
        for c in temp {
            self.letters.get_mut(instruction.dest - 1).unwrap().push(c);
        }
    }

    fn read_top(&self) -> String {
        let mut res = String::new();
        for letter in &self.letters {
            if let Some(l) = letter.last() {
                res.push(*l);
            }
        }
        res
    }

    fn ensure_setup(&mut self, i: usize) {
        while self.letters.len() < i {
            self.letters.push(Vec::new());
        }
    }

    fn append_line(&mut self, line: &[char]) {
        self.ensure_setup(line.len());
        for (i, c) in line.iter().enumerate() {
            if *c != ' ' {
                self.letters.get_mut(i).unwrap().push(*c);
            }
        }
    }
}

fn read_stack_line(l: &str) -> Vec<char> {
    let mut res = Vec::new();
    let mut chars = l.chars().into_iter().peekable();
    while chars.peek().is_some() {
        chars.next().unwrap();
        res.push(chars.next().unwrap());
        chars.next().unwrap();
        if chars.peek().is_some() {
            chars.next().unwrap();
        }
    }
    res
}

fn read_input(s: &str) -> (Stack, Vec<Instruction>) {
    let mut lines: Vec<&str> = s.lines().take_while(|l| *l != "").collect();
    let mut stack = Stack {
        letters: Vec::new(),
    };
    lines.reverse();
    lines
        .iter()
        .skip(1)
        .map(|l| read_stack_line(l))
        .for_each(|c| stack.append_line(&c));

    let instructions = s
        .lines()
        .skip_while(|l| *l != "")
        .skip(1)
        .map(|l| Instruction::from(l))
        .collect();
    (stack, instructions)
}

fn part_1(mut stack: Stack, instructions: &[Instruction]) -> String {
    for x in instructions {
        stack.handle_1(x);
    }

    stack.read_top()
}

fn part_2(mut stack: Stack, instructions: &[Instruction]) -> String {
    for x in instructions {
        stack.handle_2(x);
    }

    stack.read_top()
}

fn solve(input: &str) -> (String, String) {
    let (mut stack, instructions) = read_input(input);

    let p1 = part_1(stack.clone(), &instructions);
    let p2 = part_2(stack, &instructions);

    (p1, p2)
}

fn main() {
    let file_path = std::env::args().nth(1).unwrap();
    let (p1, p2) = solve(&read_to_string(file_path).unwrap());
    println!("Part 1 = {}", p1);
    println!("Part 2 = {}", p2);
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn example_input() {
        let input = r#"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;
        assert_eq!(solve(input), ("CMZ".to_string(), "MCD".to_string()));
    }
}
