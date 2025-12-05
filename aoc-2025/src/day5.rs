use std::ops::RangeInclusive;

type Ingredient = u64;
type Ranges = Vec<RangeInclusive<Ingredient>>;

fn read_input(contents: String) -> (Ranges, Vec<Ingredient>) {
    let (fresh_ranges, ingredient_list) = contents.split_once("\n\n").expect("no blank line");

    let parse_range = |raw: &str| {
        let (start, end) = raw.split_once("-").unwrap();
        RangeInclusive::new(
            Ingredient::from_str_radix(start.trim(), 10).expect("invalid start"),
            Ingredient::from_str_radix(end.trim(), 10).expect("invalid end"),
        )
    };
    let ranges: Vec<_> = fresh_ranges
        .lines()
        .filter(|v| !v.is_empty())
        .map(parse_range)
        .collect();
    let ingredients = ingredient_list
        .lines()
        .filter(|v| !v.is_empty())
        .map(|v| Ingredient::from_str_radix(v.trim(), 10).expect("invalid ingredient"))
        .collect();
    (ranges, ingredients)
}

fn part_1(ranges: &Ranges, ingredients: &[Ingredient]) -> usize {
    let mut count = 0;
    for ingredient in ingredients {
        for range in ranges {
            if range.contains(ingredient) {
                count += 1;
                break;
            }
        }
    }
    count
}

fn part_2(mut ranges: Ranges) -> u64 {
    // sort by start of the range
    ranges.sort_by(|r1, r2| r1.start().cmp(r2.start()));
    let mut last_range = &ranges[0];
    let mut count = (last_range.end() - last_range.start()) + 1;
    for range in &ranges[1..] {
        if range.end() <= last_range.end() {
            continue;
        }
        let cmp_end = last_range.end() + 1;
        let start = range.start().max(&cmp_end);
        let end = range.end().max(last_range.end());
        count += (end - start) + 1;
        last_range = range;
    }

    count
}

fn main() {
    let file = std::env::args()
        .skip(1)
        .next()
        .expect("missing file as first argument");
    let contents = std::fs::read_to_string(file).expect("failed to read file");
    let (ranges, ingredients) = read_input(contents);
    println!("Part 1 = {}", part_1(&ranges, &ingredients));
    println!("Part 2 = {}", part_2(ranges));
}
