extern crate itertools;

use itertools::Itertools;
use std::fs;
use std::io::{self};

pub fn read_file(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

fn draw_image(img: &Vec<Vec<i32>>) {
    for row in img {
        for pixel in row {
            if pixel == &1 {
                print!("■");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}

struct ImageDecoder {
    raw_data: String,
    width: i32,
    height: i32,
}

impl ImageDecoder {
    fn new(raw_data: String, width: i32, height: i32) -> Self {
        ImageDecoder {
            raw_data,
            width,
            height,
        }
    }

    fn _decode_layer(&self, raw_slice: &str) -> Option<Vec<Vec<i32>>> {
        if raw_slice.len() != (self.width * self.height) as usize {
            return None;
        }

        Some(
            raw_slice
                .chars()
                .map(|x| x.to_digit(10).unwrap())
                .map(|x| x as i32)
                .chunks(self.width as usize)
                .into_iter()
                .map(|x| x.collect())
                .collect(),
        )
    }

    fn _decode_layers(&self) -> Option<Vec<Vec<Vec<i32>>>> {
        let layer_size = (self.height * self.width) as usize;
        let number_of_layers = self.raw_data.len() / layer_size;
        let mut layers = Vec::new();
        for layer_num in 0..number_of_layers {
            let layer_start = layer_num * layer_size;
            let layer_end = ((layer_num + 1) * layer_size) - 1;
            layers.push(self._decode_layer(&self.raw_data[layer_start..=layer_end])?);
        }
        Some(layers)
    }

    fn _get_all_coords(&self) -> Vec<(usize, usize)> {
        let mut v = Vec::new();
        for h in 0..self.height {
            for w in 0..self.width {
                v.push((w.clone() as usize, h.clone() as usize));
            }
        }
        v
    }

    fn _blank_image(&self) -> Vec<Vec<i32>> {
        let mut img = Vec::new();
        for h in 0..self.height {
            let mut line = Vec::new();
            for w in 0..self.width {
                line.push(0)
            }
            img.push(line);
        }
        img
    }

    fn part_2(&self) -> Option<()> {
        let layers = self._decode_layers()?;
        let mut blank_cells = self._get_all_coords();
        let mut img = self._blank_image();
        for layer in layers {
            // draw_image(&img);
            let mut layers_blank_cells = Vec::new();
            for (x, y) in &blank_cells {
                let pixel = layer[y.clone()][x.clone()];
                if pixel == 2 {
                    layers_blank_cells.push((x.clone(), y.clone()));
                } else {
                    img[y.clone()][x.clone()] = pixel;
                }
            }
            blank_cells = layers_blank_cells;
        }

        draw_image(&img);

        None
    }

    fn part_1(&self) -> Option<i32> {
        let layer_size = (self.height * self.width) as usize;
        let decoded_layers: Vec<Vec<i32>> = self
            .raw_data
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .map(|x| x as i32)
            .chunks(layer_size)
            .into_iter()
            .map(|x| x.collect())
            .collect();

        let mut layer_zero_count = std::usize::MAX;
        let mut final_layer = Vec::new();
        for layer in decoded_layers {
            let zero_count = layer.iter().filter(|x| x == &&0).count();
            if zero_count < layer_zero_count {
                layer_zero_count = zero_count;
                final_layer = layer;
            }
        }

        let count_1 = final_layer.iter().filter(|x| x == &&1).count() as i32;
        let count_2 = final_layer.iter().filter(|x| x == &&2).count() as i32;
        Some(count_1 * count_2)
    }
}

fn main() {
    let raw_img =
        read_file("/home/tim/projects/AoC19/resources/day8input").expect("unable to load input");
    let image_decoder = ImageDecoder::new(raw_img, 25, 6);
    println!("{:?}", image_decoder.part_1());
    println!("{:?}", image_decoder.part_2());
}
