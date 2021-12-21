use std::collections::HashMap;

#[derive(Debug)]
struct Die {
    next: u64,
    rolls: u64,
}

impl Die {
    fn new() -> Self {
        Die { next: 1, rolls: 0 }
    }

    fn roll(&mut self) -> u64 {
        let v = self.next;
        self.next = (v % 100) + 1;
        self.rolls += 1;
        v
    }
}

fn solve_1(mut p1: u64, mut p2: u64) -> u64 {
    let mut die = Die::new();
    let mut p1_score = 0;
    let mut p2_score = 0;
    let mut player_1_turn = true;
    while p1_score < 1000 && p2_score < 1000 {
        let r1 = die.roll();
        let r2 = die.roll();
        let r3 = die.roll();
        let roll_sum = r1 + r2 + r3;
        if player_1_turn {
            p1 = (p1 - 1 + roll_sum) % 10 + 1;
            p1_score += p1;
        } else {
            p2 = (p2 - 1 + roll_sum) % 10 + 1;
            p2_score += p2;
        }

        player_1_turn = !player_1_turn;
    }

    let loosing_score = p1_score.min(p2_score);

    loosing_score * die.rolls
}

const POSSIBLE_DIE_ROLLS: [u64; 27] = [
    3, 4, 5, 4, 5, 6, 5, 6, 7, 4, 5, 6, 5, 6, 7, 6, 7, 8, 5, 6, 7, 6, 7, 8, 7, 8, 9,
];

fn solve_2_recursive(
    cache: &mut HashMap<(u64, u64, u64, u64), (u64, u64)>,
    position: u64,
    score: u64,
    alt_pos: u64,
    alt_score: u64,
) -> (u64, u64) {
    if score >= 21 {
        return (1, 0);
    }
    if let Some((won, lost)) = cache.get(&(position, score, alt_pos, alt_score)) {
        return (*won, *lost);
    }

    let mut wins = 0;
    let mut losses = 0;
    for roll in &POSSIBLE_DIE_ROLLS {
        let new_pos = (position - 1 + roll) % 10 + 1;
        let new_score = score + new_pos;
        if new_score >= 21 {
            wins += 1;
        } else {
            let (lost, won) = solve_2_recursive(cache, alt_pos, alt_score, new_pos, new_score);
            wins += won;
            losses += lost;
        }
    }
    cache.insert((position, score, alt_pos, alt_score), (wins, losses));

    (wins, losses)
}

fn solve_2(p1: u64, p2: u64) -> u64 {
    // caching the state of the game to the number of wins / losses for that player
    let mut cache: HashMap<(u64, u64, u64, u64), (u64, u64)> = HashMap::new();

    let (p1_wins, p1_losses) = solve_2_recursive(&mut cache, p1, 0, p2, 0);
    p1_wins.max(p1_losses)
}

fn main() {
    let p1 = u64::from_str_radix(&std::env::args().skip(1).next().unwrap(), 10).unwrap();
    let p2 = u64::from_str_radix(&std::env::args().skip(2).next().unwrap(), 10).unwrap();
    println!("Part 1 = {}", solve_1(p1, p2));
    println!("Part 2 = {}", solve_2(p1, p2));
}
