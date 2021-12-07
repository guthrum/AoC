use std::fs::read_to_string;

fn calculate_number(i: u8, days: u32) -> (u8, usize) {
    let mut state = Vec::with_capacity(6703087164);
    state.push(i);
    for _i in 1..=days {
        let mut append = Vec::with_capacity(state.len()/2);
        for v in &mut state {
            if *v == 0 {
                *v = 6;
                append.push(8);
            } else {
                *v -= 1;
            }
        }
        state.append(&mut append);
    }

    (i, state.len())
}


fn solve(input: &str, days: u32) -> usize {
    let mut counts = [0 as usize; 9];
    for i in 0..=8 {
        let (_, val) = calculate_number(i, days);
        counts[i as usize] = val;
        println!("\t{} for {} days = {}", i, days, val);
    }
    println!("calculations complete");

    input.lines().next().unwrap().split(",")
        .map(|v| u8::from_str_radix(v, 10).expect(&format!("{} is invalid", v)))
        .map(|v| counts[v as usize])
        .sum()
}

fn main() {
    let file_path = std::env::args().skip(1).next().unwrap();
    let contents = read_to_string(file_path).unwrap();
    println!("Part 1 = {}", solve(&contents, 80));
    println!("Part 2 = {}", solve(&contents, 256));
}
