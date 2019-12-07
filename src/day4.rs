fn check_same(v1: &u32, v2: &u32) -> bool {
    v1 == v2
}

fn v1_less_than_or_equal(v1: &u32, v2: &u32) -> bool {
    v1 <= v2
}


fn make_checks_p1(password: Vec<u32>) -> bool {
    let mut iter = password.iter().peekable();

    let mut adjacent = false;
    let mut non_decreasing = true;

    while let Some(value) = iter.next() {
        if let Some(next) = iter.peek() {
            adjacent = adjacent || check_same(next, &value);
            non_decreasing = non_decreasing && v1_less_than_or_equal(&value, next);
        }
        if !non_decreasing {
            return false;
        }
    }

    adjacent && non_decreasing
}

fn make_checks_p2(password: Vec<u32>) -> bool {
    let mut adjacent = false;

    for i in 0..password.len() {
        if let Some(current) = password.get(i) {
            if let Some(next) = password.get(i+1) {
                // check non decreasing
                if next < current {
                    return false;
                }
                if next == current {
                    if let Some(next_2) = password.get(i+2) {
                        if next == next_2 {
                            continue;
                        }
                    }
                    if i > 0 {
                        if let Some(prev_1) = password.get(i-1) {
                            if prev_1 == current {
                                continue
                            } 
                        }
                    }
                    adjacent = adjacent || true;
                }
            }
        }
    }

    adjacent
}

fn deserialize_password(int_form: i32) -> Vec<u32> {
    let str_form: String = int_form.to_string();
    str_form.chars()
        .map(|x| x.to_digit(10).expect("failed to parse character"))
        .collect()
}

fn solve_part1(lower: i32, upper: i32) -> u32 {
    let mut count = 0;
    for n in lower..upper+1 {
        if make_checks_p1(deserialize_password(n)) {
            count += 1;
        }
    }
    count
}

fn solve_part2(lower: i32, upper: i32) -> u32 {
    let mut count = 0;
    for n in lower..upper+1 {
        if make_checks_p2(deserialize_password(n)) {
            count += 1;
        }
    }
    count
}

fn main() {
    println!("{}", solve_part1(278384, 824795));
    println!("{}", solve_part2(278384, 824795));
}