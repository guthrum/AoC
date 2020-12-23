use structopt::StructOpt;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Token {
    Operator(char),
    Value(i64),
}

impl Token {
    fn numerical_value(&self) -> i64 {
        match *self {
            Token::Value(v) => v,
            _ => panic!("Token is a not a number"),
        }
    }

    fn apply(&self, a: i64, b: i64) -> i64 {
        match *self {
            Token::Operator('+') => a + b,
            Token::Operator('*') => a * b,
            _ => panic!("{:?} not able to apply to numbers", *self),
        }
    }

    /// Precedence value lower is less precedence
    fn precedence(&self) -> i64 {
        match *self {
            Token::Operator('*') => 2,
            Token::Operator('+') => 3,
            Token::Operator('(') => 5,
            Token::Operator(')') => 5,
            _ => 1,
        }
    }

    fn is_operator(&self) -> bool {
        match *self {
            Token::Operator(_) => true,
            _ => false,
        }
    }

    fn is_closed_paranthesis(&self) -> bool {
        match *self {
            Token::Operator(')') => true,
            _ => false,
        }
    }

    fn is_open_paranthesis(&self) -> bool {
        match *self {
            Token::Operator('(') => true,
            _ => false,
        }
    }

    fn is_paranthesis(&self) -> bool {
        self.is_open_paranthesis() || self.is_closed_paranthesis()
    }
}

impl From<char> for Token {
    fn from(c: char) -> Token {
        match c {
            '*' => Token::Operator(c),
            '+' => Token::Operator(c),
            '(' => Token::Operator(c),
            ')' => Token::Operator(c),
            _ => {
                let n: i64 = c
                    .to_digit(10)
                    .expect(&format!("unable to parse {} as int", c))
                    as i64;
                Token::Value(n)
            }
        }
    }
}

type Expression = Vec<Token>;

fn postfix_notation(expression: &Expression) -> Expression {
    let mut output: Vec<Token> = Vec::new();
    let mut operators: Vec<Token> = Vec::new();
    for token in expression {
        if !token.is_operator() {
            output.push(*token);
        } else if token.is_open_paranthesis() {
            operators.push(*token);
        } else if token.is_closed_paranthesis() {
            let mut found_opening = false;
            while let Some(op) = operators.pop() {
                // we want to stop once we get the first '(' since this token is closing that one
                if op.is_open_paranthesis() {
                    found_opening = true;
                    break;
                }
                output.push(op);
            }
            if !found_opening {
                panic!("Opening paranthesis do not match.");
            }
        } else {
            loop {
                // while opt_op.is_some() {
                let pop_stack = {
                    let last = operators.last();
                    let stack_precedence = last.map(|o| o.precedence()).unwrap_or(0);
                    last.is_some()
                        && !last.unwrap().is_open_paranthesis()
                        && (token.precedence() <= stack_precedence)
                };
                if !pop_stack {
                    break;
                }
                let opt_op = operators.pop();
                let op = opt_op.unwrap();
                output.push(op);
            }
            operators.push(*token);
        }
    }
    while let Some(op) = operators.pop() {
        if op.is_paranthesis() {
            panic!("Closing paranthesis do not match.");
        }
        output.push(op);
    }
    output
}

fn evaluate_expression(expression: &Expression) -> i64 {
    let postfix = postfix_notation(expression);
    let mut stack = Vec::new();
    for token in postfix {
        if token.is_operator() {
            let a: Token = stack.pop().expect("expect 2 values on stack");
            let b: Token = stack.pop().expect("epxect 1 value on stack");
            stack.push(Token::Value(
                token.apply(a.numerical_value(), b.numerical_value()),
            ));
        } else {
            stack.push(token);
        }
    }
    assert_eq!(stack.len(), 1);
    stack.pop().unwrap().numerical_value()
}

fn part(expressions: &Vec<Expression>) -> i64 {
    expressions.iter().map(|e| evaluate_expression(e)).sum()
}

fn main() {
    let options = Options::from_args();
    let reader = BufReader::new(File::open(options.input).unwrap());
    let expressions: Vec<Expression> = reader
        .lines()
        .map(|l| l.unwrap())
        .map(|s| s.replace(" ", ""))
        .map(|s| s.chars().map(|c| Token::from(c)).collect())
        .collect();
    println!("Result = {}", part(&expressions));
}
