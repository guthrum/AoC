use std::collections::HashMap;

fn part_1(starting_sequence: &Vec<i64>, target_turn: i64) -> i64 {
    let mut tracker: HashMap<i64, (i64, i64)> = starting_sequence
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, v)| (v, (i as i64 + 1, -1)))
        .collect();
    let mut turn = starting_sequence.len() as i64;
    let mut last_number = *starting_sequence.last().unwrap();
    while turn < target_turn {
        turn += 1;
        let say = match tracker.get(&last_number) {
            Some((_, -1)) => 0,
            Some((recent_turn, previous_turn)) => recent_turn - previous_turn,
            None => 0,
        };
        tracker
            .entry(say)
            .and_modify(|(recent_turn, previous_turn)| {
                *previous_turn = *recent_turn;
                *recent_turn = turn;
            })
            .or_insert((turn, -1));
        last_number = say;
    }
    last_number
}

fn main() {
    //let input = vec![0, 3, 6];
    let input = vec![6, 4, 12, 1, 20, 0, 16];
    println!("Part 1 = {}", part_1(&input, 2020));
    println!("Part 2 = {}", part_1(&input, 30000000));
}
