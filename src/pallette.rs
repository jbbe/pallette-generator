use image::{ImageReader, Rgb};
use rand::rng;
use rand::seq::SliceRandom; // For shuffling the array
use raqote::*;
use std::collections::HashMap;
use std::io::{self, Write};
use std::sync::mpsc;
use std::thread;

use crate::color::ColorUtil;

pub(crate) struct Pallette {
    pub top_rgb: Vec<Rgb<u8>>,
    pub top_hex: Vec<String>,
    pub current_path: Option<String>,
    pub all_entries: Vec<(Rgb<u8>, usize)>,
    pub pallette_size: usize,
}

impl Default for Pallette {
    fn default() -> Self {
        Self {
            top_rgb: Vec::new(),
            top_hex: Vec::new(),
            current_path: None,
            all_entries: Vec::new(),
            pallette_size: 16,
        }
    }
}

impl Pallette {
    pub fn update(&mut self, path: &str) {
        if let Some(cur) = &self.current_path {
            // Return early if they are the same
            if cur == path {
                return;
            }
        }
        self.current_path = Some(path.to_string());
        let p = path.to_string().clone();

        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            // let p= path.to_string();
            println!("in thread {p}");
            tx.send(Self::extract_pallete(&p).unwrap())
        });

        let map = rx.recv().unwrap();
        // println!("Got: {received}");
        // let s = map.size
        println!("map received ");

        // let full_pallette = extract_pallete(pal_name, &file_path).unwrap();
        self.all_entries = Self::get_sorted_entries(map);
        if self.all_entries.len() < self.pallette_size {
            self.pallette_size = self.all_entries.len()
        }
        self.update_top_colors();
    }

    pub fn update_top_colors(&mut self) {
        let res = Self::get_top_colors(self.all_entries.clone(), self.pallette_size);
        self.top_rgb = res.0;
        self.top_hex = res.1;
        self.pallette_size = self.top_rgb.len();
    }

    pub fn get_unused_entry(&mut self) -> Option<Rgb<u8>> {
        // Create an array of integers from `start` to `end`
        let start = 0;
        let end = if !self.all_entries.is_empty() {
            self.all_entries.len() - 1
        } else {
            0
        };
        let mut array: Vec<usize> = (start..=end).collect();

        // Shuffle the array
        let mut rng = rng();
        array.shuffle(&mut rng);
        for i in array {
            let e = self.all_entries[i];
            if !self.top_rgb.contains(&e.0) {
                return Some(e.0);
            }
        }
        None
    }

    pub fn swap_top_color(&mut self, idx: usize) {
        println!("Swap top color {idx}");
        let e = self.get_unused_entry();
        // println!("Swap top color {idx}");
        if let Some(c) = e {
            self.top_rgb[idx] = c
        }
    }

    pub fn decrement_pallette_size(&mut self) {
        if self.pallette_size > 1 {
            self.pallette_size -= 1;
            self.update_top_colors();
        }
    }

    pub fn increment_pallette_size(&mut self) {
        if self.pallette_size < self.all_entries.len() {
            self.pallette_size += 1;
            self.update_top_colors();
        }
    }

    pub fn save_pallette_img(&mut self, pallette_name: String) {
        self.output_pallette(&pallette_name);
    }

    pub fn save_pallette_text(&mut self, pallette_name: String) {
        if let Err(e) = self.output_pallette_txt(&pallette_name) {
            eprintln!("Error writing to file: {}", e);
        }
    }

    pub fn reset(&mut self) {
        self.top_rgb = Vec::new();
        self.top_hex = Vec::new();
        self.current_path = None;
        self.all_entries = Vec::new();
        self.pallette_size = 16;
    }

    fn get_sorted_entries(pallette: HashMap<Rgb<u8>, usize>) -> Vec<(Rgb<u8>, usize)> {
        let mut entries: Vec<(Rgb<u8>, usize)> = pallette.into_iter().collect();

        // Sort by the count in descending order
        entries.sort_by(|a, b| b.1.cmp(&a.1));
        entries
    }

    pub fn get_top_colors(
        entries: Vec<(Rgb<u8>, usize)>,
        top_n: usize,
    ) -> (Vec<Rgb<u8>>, Vec<String>) {
        let mut top_rgb = Vec::new();
        let mut top_hex = Vec::new();
        for e in entries {
            let mut should_add = true;
            if top_rgb.len() >= top_n {
                break;
            }
            for c in &top_rgb {
                if ColorUtil::color_distance(e.0, *c) < 20. {
                    should_add = false;
                    break;
                }
            }
            if should_add {
                top_rgb.push(e.0);
                top_hex.push(ColorUtil::rgb_to_hex(e.0));
            }
        }

        (top_rgb, top_hex)
    }

    pub fn update_color(&mut self, original: Rgb<u8>, new_color: Rgb<u8>) {
        for i in 0..self.top_rgb.len() {
            if self.top_rgb[i] == original {
                self.top_rgb[i] = new_color;
                self.top_hex[i] = ColorUtil::rgb_to_hex(new_color);
            }
        }
    }

    pub fn add_complementary(&mut self, c: Rgb<u8>) {
        let complement = Rgb([255 - c[0], 255 - c[1], 255 - c[2]]);
        self.add_new_color(complement);
    }

    pub fn add_new_color(&mut self, c: Rgb<u8>) {
        self.pallette_size += 1;
        self.top_rgb.push(c);
        self.top_hex.push(ColorUtil::rgb_to_hex(c));
    }

    pub fn extract_pallete(path: &str) -> Option<HashMap<Rgb<u8>, usize>> {
        println!("Extracting Pallette from {path} ");
        let img = ImageReader::open(path).unwrap().decode().unwrap();
        let rgb = img.to_rgb8();
        // let mut pixels = Vec::<PColor>::new();
        let mut pix = HashMap::<Rgb<u8>, usize>::new();
        for pixel in rgb.pixels() {
            *pix.entry(*pixel).or_insert(0) += 1
        }

        println!("Pallette extracted");

        Some(pix)
    }

    fn output_pallette_txt(&mut self, pal_name: &str) -> io::Result<()> {
        // let strs = colors.iter().map(|c| Self::rgb_to_hex(*c));
        let output_path = format!("pallettes/{pal_name}.txt");
        let mut output = std::fs::File::create(&output_path)?;
        for line in self.top_hex.clone().iter() {
            writeln!(output, "{}", line)?;
        }
        Ok(())
    }

    fn output_pallette(&mut self, pal_name: &str) {
        println!("Output pallette");
        let square_size = 64.;
        let margin = 16.;
        let width = 512;
        let colors = self.top_rgb.clone();
        let height = ((margin + square_size) * ((colors.len() as f32) / 5.) + margin) as i32;

        let mut dt = DrawTarget::new(width, height);

        let mut pb = PathBuilder::new();
        // pb.move_to(current_x, current_y);
        pb.rect(0., 0., width as f32, height as f32);
        pb.close();
        let path = pb.finish();
        // let solid = SolidSource::from_unpremultiplied_argb(0xff, 0, 0, 0);
        let solid = SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0xff, 0xff);
        dt.fill(&path, &Source::Solid(solid), &DrawOptions::new());

        let mut current_x = 0.;
        let mut current_y = margin;
        let col_count = 5;
        let mut current_col = 0;

        for color in colors {
            // println!("Draw color: {color}");
            current_x += margin;
            let mut pb = PathBuilder::new();
            // pb.move_to(current_x, current_y);
            pb.rect(current_x, current_y, square_size, square_size);
            pb.close();
            let path = pb.finish();
            let solid = SolidSource::from_unpremultiplied_argb(0xff, color[0], color[1], color[2]);
            dt.fill(&path, &Source::Solid(solid), &DrawOptions::new());
            current_col += 1;
            if current_col > col_count {
                current_col = 0;
                current_x = 0.;
                current_y += square_size + margin;
            } else {
                current_x += square_size;
            }
        }
        let _ = dt.write_png(format!("pallettes/{pal_name}.png"));
    }
}
