use std::fs;

#[derive(Copy, Clone, Debug)]
enum Command {
    END,
    ADD(usize, usize, usize),
    MULTIPLY(usize, usize, usize),
}

fn read_file(path: &str) -> std::io::Result<Vec<usize>> {
    Ok(fs::read_to_string(path)?
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect())
}

fn parse_command(slice: &[usize]) -> Option<Command> {
    match slice[0] {
        1 => Some(Command::ADD(slice[1], slice[2], slice[3])),
        2 => Some(Command::MULTIPLY(slice[1], slice[2], slice[3])),
        99 => Some(Command::END),
        _ => None,
    }
}

fn run_machine(mut input: Vec<usize>) -> usize {
    let mut program_counter = 0;
    let mut parsed_command = parse_command(&input[program_counter..program_counter + 4]);
    while let Some(command) = parsed_command {
        match command {
            Command::END => return input[0],
            Command::ADD(v1, v2, res) => input[res] = input[v1] + input[v2],
            Command::MULTIPLY(v1, v2, res) => input[res] = input[v1] * input[v2],
        }
        program_counter += 4;
        parsed_command =
            parse_command(&input[program_counter..std::cmp::min(program_counter + 4, input.len())]);
    }
    0
}

fn execute(mut input: Vec<usize>, noun: usize, verb: usize) -> usize {
    input[1] = noun;
    input[2] = verb;
    run_machine(input)
}

fn main() {
    // let numbers = ;
    let mut noun = 0;
    let mut verb = 0;
    while execute(
        read_file("/home/tim/projects/AoC19/resources/day2input").expect("unable to load numbers"),
        noun,
        verb,
    ) != 19690720
    {
        if noun == 99 {
            noun = 0;
            verb += 1;
        } else {
            noun += 1;
        }
    }
    println!("{}", 100 * noun + verb);
}
