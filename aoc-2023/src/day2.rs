use std::cmp::max;
use std::collections::HashMap;
use std::fs::read_to_string;

type Hand = HashMap<String, usize>;
type Game = Vec<Hand>;

fn read_line(mut line: &str) -> (usize, Game) {
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    line = line.trim().strip_prefix("Game ").unwrap();
    let (id_str, raw_hands) = line.split_once(":").unwrap();
    let read_colour = |raw_colour: &str| {
        let (count, colour) = raw_colour.trim().split_once(" ").unwrap();
        (
            colour.to_string(),
            usize::from_str_radix(count, 10).expect(&format!("failed to parse count {count}")),
        )
    };
    let read_hand = |raw_hand: &str| raw_hand.trim().split(",").map(read_colour).collect();

    let hand = raw_hands.split(";").map(|l| read_hand(l)).collect();
    (
        usize::from_str_radix(id_str, 10).expect(&format!("failed to parse game id {id_str}")),
        hand,
    )
}

fn read_input(input: &str) -> HashMap<usize, Game> {
    input.lines().map(read_line).collect()
}

fn solve(input: &str) -> (usize, usize) {
    let games = read_input(input);

    let part_1_hand_valid = |hand: &Hand| {
        let mut res = true;
        for (color, count) in hand.iter() {
            let test = match color.as_str() {
                "red" => 12,
                "blue" => 14,
                "green" => 13,
                _ => 100000,
            };
            res = res & (*count <= (test as usize))
        }
        res
    };
    let part_1_valid = |game: &Game| game.iter().all(part_1_hand_valid);
    let mut res1 = 0;
    for (id, game) in &games {
        if part_1_valid(game) {
            res1 += *id;
        }
    }

    let game_power_value = |game: &Game| {
        let mut max_counts: HashMap<String, usize> = HashMap::new();

        for hand in game {
            for (color, count) in hand {
                let v = (*max_counts.get(color).unwrap_or(&0)).max(*count);
                max_counts.insert(color.to_string(), v);
            }
        }

        max_counts
            .into_values()
            .filter(|v| v > &0)
            .product::<usize>()
    };
    let res2 = games.values().map(game_power_value).sum();

    (res1, res2)
}

fn main() {
    let file_path = std::env::args().nth(1).unwrap();
    let (p1, p2) = solve(&read_to_string(file_path).unwrap());
    println!("Part 1 = {}", p1);
    println!("Part 2 = {}", p2);
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn example_input() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        assert_eq!(solve(input), (8, 2286));
    }
}
