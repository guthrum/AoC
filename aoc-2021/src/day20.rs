use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

const CELL_OFFSETS: [(i64, i64); 9] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (0, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn read_input(path: &str) -> (Vec<bool>, Vec<Vec<bool>>) {
    let raw = read_to_string(path).unwrap();
    let mut lines = raw.lines().peekable();

    let enhancement_data: Vec<bool> = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| {
            if c == '.' {
                false
            } else if c == '#' {
                true
            } else {
                panic!()
            }
        })
        .collect();
    if lines.next().unwrap() != "" {
        panic!();
    }

    let mut image = Vec::new();
    while let Some(line) = lines.next() {
        let row = line
            .chars()
            .map(|c| {
                if c == '.' {
                    false
                } else if c == '#' {
                    true
                } else {
                    panic!()
                }
            })
            .collect();
        image.push(row);
    }

    (enhancement_data, image)
}

fn get_pixels(image: &Vec<Vec<bool>>, x: i64, y: i64, default: bool) -> [bool; 9] {
    let mut res = [default; 9];
    for (pos, (dx, dy)) in CELL_OFFSETS.iter().enumerate() {
        let nx = x + dx;
        let ny = y + dy;
        // perform a hack where if the pixel is outside of the core image we assume its the default
        if 0 < ny && ny < (image.len() as i64) - 1 && 0 < nx && nx < (image[0].len() as i64) - 1 {
            res[pos] = image[ny as usize][nx as usize];
        }
    }
    res
}

fn pad_image(image: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    // if any of the edges have a true value then we need to pad
    let mut pad = false;
    for v in image.first().unwrap() {
        pad = pad || *v;
    }
    for v in image.last().unwrap() {
        pad = pad || *v;
    }
    for row in &image {
        pad = pad || *row.first().unwrap();
        pad = pad || *row.last().unwrap();
    }

    if !pad {
        image
    } else {
        let mut new_image = Vec::with_capacity(image.len() + 2);
        let x = image[0].len() + 2;
        new_image.push(vec![false; x]);

        for row in image {
            let mut new_row = Vec::with_capacity(x);
            new_row.push(false);
            new_row.extend(row);
            new_row.push(false);
            new_image.push(new_row);
        }

        new_image.push(vec![false; x]);
        new_image
    }
}

fn enhance_image(
    enhancement_data: &Vec<bool>,
    image: Vec<Vec<bool>>,
    default: bool,
) -> Vec<Vec<bool>> {
    let mut new_image = Vec::with_capacity(image.len());
    for y in 0..image.len() {
        let mut row = Vec::new();
        for x in 0..image[0].len() {
            let mut ptr = 0;
            let offsets = get_pixels(&image, x as i64, y as i64, default);
            for v in offsets {
                ptr = ptr << 1;
                if v {
                    ptr += 1;
                }
            }
            //println!("y = {}, x = {}, offsets = {:?} ptr = {}", x, y, offsets, ptr);
            row.push(enhancement_data[ptr]);
        }
        new_image.push(row);
    }
    pad_image(new_image)
}

fn print(image: &Vec<Vec<bool>>) {
    for row in image {
        for v in row {
            let c = if *v { '#' } else { '.' };
            print!("{}", c);
        }
        println!("")
    }
}

fn solve(input: (Vec<bool>, Vec<Vec<bool>>), rounds: u32) -> usize {
    let enhancement_data = input.0;
    let mut image = pad_image(input.1);
    //print(&image);
    for i in 1..=rounds {
        let default = i % 2 != 1;
        println!("Round = {}, default ={}", i, default);
        image = enhance_image(&enhancement_data, image, default);
    }
    image.iter().flat_map(|r| r.iter()).filter(|v| **v).count()
}

fn main() {
    let file_path = std::env::args().skip(1).next().unwrap();
    let input = read_input(&file_path);
    println!("Part 1 = {}", solve(input.clone(), 2));
    println!("Part 2 = {}", solve(input.clone(), 50));
}
