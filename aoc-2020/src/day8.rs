use structopt::StructOpt;

use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

#[derive(Debug, Clone)]
enum Instruction {
    NoOp(i32),
    Acc(i32),
    Jmp(i32),
}

type Program = Vec<Instruction>;

fn parse_instruction(raw: &String) -> Option<Instruction> {
    let mut parts = raw.split(" ");
    let instruction = parts.next()?;
    let argument: i32 = parts.next()?.parse().ok()?;
    match instruction {
        "nop" => Some(Instruction::NoOp(argument)),
        "acc" => Some(Instruction::Acc(argument)),
        "jmp" => Some(Instruction::Jmp(argument)),
        _ => None,
    }
}

fn part_1(program: &Program) -> (bool, i32) {
    let mut acc = 0;
    let mut ptr = 0;
    let mut executed_instructions = HashSet::new();
    while !executed_instructions.contains(&ptr) {
        if ptr == program.len() {
            return (true, acc);
        }
        executed_instructions.insert(ptr);
        let instruction = program.get(ptr).unwrap();
        match instruction {
            Instruction::NoOp(_) => {
                ptr += 1;
            }
            Instruction::Acc(amount) => {
                acc += amount;
                ptr += 1;
            }
            Instruction::Jmp(offset) => {
                ptr = ((ptr as i32) + offset) as usize;
            }
        }
    }
    (false, acc)
}

fn part_2(program: Program) -> Option<i32> {
    for i in 0..program.len() - 1 {
        let mut p2 = program.clone();
        let run = match program.get(i).unwrap() {
            Instruction::NoOp(v) => {
                *p2.get_mut(i)? = Instruction::Jmp(*v);
                true
            }
            Instruction::Jmp(offset) => {
                *p2.get_mut(i)? = Instruction::NoOp(*offset);
                true
            }
            _ => false,
        };
        if run {
            let (complete, acc) = part_1(&p2);
            if complete {
                return Some(acc);
            }
        }
    }
    None
}

fn main() {
    let options = Options::from_args();
    let reader = BufReader::new(File::open(options.input).unwrap());
    let program: Program = reader
        .lines()
        .map(|x| x.unwrap())
        .flat_map(|l| parse_instruction(&l).into_iter())
        .collect();
    println!("Part 1 = {}", part_1(&program).1);
    println!("Part 2 = {:?}", part_2(program))
}
