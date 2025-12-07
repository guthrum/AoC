use std::fmt::format;

#[derive(Copy, Clone)]
enum Op {
    Add,
    Subtract,
    Divide,
    Multiply,
}

impl Op {
    fn reducer<
        T: std::ops::Sub<Output = T>
            + std::ops::Add<Output = T>
            + std::ops::Mul<Output = T>
            + std::ops::Div<Output = T>,
    >(
        &self,
    ) -> impl Fn(T, T) -> T {
        match *self {
            Self::Add => |a, b| a + b,
            Self::Subtract => |a, b| a - b,
            Self::Multiply => |a, b| a * b,
            Self::Divide => |a, b| a / b,
        }
    }
}

impl From<char> for Op {
    fn from(value: char) -> Self {
        match value {
            '+' => Self::Add,
            '-' => Self::Subtract,
            '*' => Self::Multiply,
            '/' => Self::Divide,
            v => panic!("invalid op code {}", v),
        }
    }
}

struct Problem {
    op: Op,
    values: Vec<i64>,
}

impl Problem {
    fn calc(&self) -> i64 {
        self.values
            .iter()
            .cloned()
            .reduce(self.op.reducer())
            .unwrap_or(0)
    }
}

fn read_input_1(raw: &String) -> Vec<Problem> {
    let mut lines: Vec<Vec<_>> = raw
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();
    let operators = lines.pop().expect("require atleast 1 line");
    let mut res = vec![];
    for (idx, op_code) in operators.iter().enumerate() {
        let op = Op::from(op_code.chars().next().unwrap());
        let values = lines
            .iter()
            .map(|v| v[idx])
            .map(|v| i64::from_str_radix(v.trim(), 10).expect("invalid number"))
            .collect();
        res.push(Problem { op, values });
    }
    res
}

fn read_input_2(raw: &String) -> Vec<Problem> {
    let length = raw.lines().next().unwrap().len();
    let mut operators: Vec<(Op, usize)> = vec![];
    let mut lines: Vec<Vec<(usize, char)>> =
        raw.lines().map(|l| l.char_indices().collect()).collect();
    for (idx, c) in lines.pop().unwrap() {
        if c != ' ' {
            operators.push((Op::from(c), idx));
        }
    }
    let mut res = vec![];
    for (idx, (op, starting_idx)) in operators.iter().enumerate() {
        let end = operators.get(idx + 1).map(|v| v.1).unwrap_or(length);
        let mut values: Vec<i64> = vec![];
        for cidx in *starting_idx..end {
            let number: String = lines
                .iter()
                .map(|l| l[cidx])
                .map(|c| c.1)
                .filter(|c| *c != ' ')
                .collect();
            if number.is_empty() {
                break;
            }
            values.push(
                i64::from_str_radix(&number, 10).expect(&format!("{} is invalid number", number)),
            );
        }
        res.push(Problem { op: *op, values });
    }

    res
}

fn main() {
    let file = std::env::args()
        .skip(1)
        .next()
        .expect("missing file as first argument");
    let contents = std::fs::read_to_string(file).expect("failed to read file");
    let problems = read_input_1(&contents);
    let p1: i64 = problems.iter().map(|p| p.calc()).sum();
    println!("Part 1 = {}", p1);
    let problems = read_input_2(&contents);
    let p2: i64 = problems.iter().map(|p| p.calc()).sum();
    println!("Part 2 = {}", p2);
}
