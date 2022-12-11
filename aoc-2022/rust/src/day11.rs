use std::fs::read_to_string;

#[derive(Debug, Clone)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

impl Operation {
    fn evaluate(&self, v: u64) -> u64 {
        match *self {
            Self::Add(x) => x + v,
            Self::Multiply(x) => x * v,
            Self::Square => v * v,
        }
    }
}

impl From<&str> for Operation {
    fn from(s: &str) -> Self {
        if let Some(add) = s.strip_prefix("old + ") {
            Self::Add(u64::from_str_radix(add, 10).expect(&format!("{} is invalid add ", add)))
        } else if s == "old * old" {
            Self::Square
        } else if let Some(multiply) = s.strip_prefix("old * ") {
            Self::Multiply(
                u64::from_str_radix(multiply, 10)
                    .expect(&format!("{} is invalid multiply", multiply)),
            )
        } else {
            panic!("{} is not known", s);
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    id: usize,
    items: Vec<u64>,
    operation: Operation,
    divisible_test: u64,
    true_monkey: usize,
    false_monkey: usize,
    inspection_count: usize,
}

impl From<&str> for Monkey {
    fn from(l: &str) -> Self {
        let lines: Vec<&str> = l.lines().collect();
        let id = lines
            .get(0)
            .unwrap()
            .strip_prefix("Monkey ")
            .unwrap()
            .split(":")
            .next()
            .unwrap();
        let items = lines
            .get(1)
            .unwrap()
            .strip_prefix("  Starting items:")
            .unwrap()
            .replace(" ", "")
            .split(",")
            .map(|i| u64::from_str_radix(i, 10).unwrap())
            .collect();
        let operation = lines
            .get(2)
            .unwrap()
            .strip_prefix("  Operation: new = ")
            .unwrap();
        let divisible_test = lines
            .get(3)
            .unwrap()
            .strip_prefix("  Test: divisible by ")
            .unwrap();
        let true_monkey = lines
            .get(4)
            .unwrap()
            .strip_prefix("    If true: throw to monkey ")
            .unwrap();
        let false_monkey = lines
            .get(5)
            .unwrap()
            .strip_prefix("    If false: throw to monkey ")
            .unwrap();

        Self {
            id: usize::from_str_radix(id, 10).unwrap(),
            items,
            operation: Operation::from(operation),
            divisible_test: u64::from_str_radix(divisible_test, 10).unwrap(),
            true_monkey: usize::from_str_radix(true_monkey, 10).unwrap(),
            false_monkey: usize::from_str_radix(false_monkey, 10).unwrap(),
            inspection_count: 0,
        }
    }
}

impl Monkey {
    fn run(&mut self) -> Vec<(usize, u64)> {
        let mut res = Vec::new();
        while let Some(item) = self.items.pop() {
            self.inspection_count += 1;
            let new_value = self.operation.evaluate(item) / 3;
            let monkey = if new_value % self.divisible_test == 0 {
                self.true_monkey
            } else {
                self.false_monkey
            };
            res.push((monkey, new_value));
        }
        res
    }
}

fn read_input(input: &str) -> Vec<Monkey> {
    input.split("\n\n").map(|l| Monkey::from(l)).collect()
}

fn solve_1(mut monkeys: Vec<Monkey>) -> usize {
    for i in 1..=20 {
        for mut idx in 0..monkeys.len() {
            let destinations = monkeys.get_mut(idx).unwrap().run();
            for (monkey, value) in destinations {
                monkeys.get_mut(monkey).unwrap().items.push(value);
            }
        }
    }

    let mut res: Vec<usize> = monkeys.iter().map(|m| m.inspection_count).collect();
    res.sort_by(|a, b| b.cmp(a));
    res.get(0).unwrap() * res.get(1).unwrap()
}

fn solve(lines: &str) -> (usize, i64) {
    let input = read_input(lines);
    (solve_1(input.clone()), 0)
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
        let input = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#;
        assert_eq!(solve(input), (10605, 0));
    }
}
