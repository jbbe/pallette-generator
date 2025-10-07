use std::collections::HashMap;

use image::{ImageReader, Rgb};
use raqote::*;

// #[derive(Default)]
pub(crate) struct Pallette {
    pub top_colors: Vec<Rgb<u8>>,
    pub current_path: Option<String>,
    pub all_entries: Vec<(Rgb<u8>, usize)>,
    pub pallette_size: usize,
}

impl Default for Pallette {
    fn default() -> Self {
        Self {
            top_colors: Vec::new(),
            current_path: None,
            all_entries: Vec::new(),
            pallette_size: 5,
        }
    }
}

impl Pallette {

    // pub fn new() -> Self {
    // }
    pub fn update(&mut self, path: &String) {
        if let Some(cur) = &self.current_path {
            // Return early if they are the same
            if cur == path {
                return;
            }
        }
        // if let Some(ref p) = path && let Some(cur) = self.current_path {
        //     if *cur == *p  {
        //         return
        //     }
        // }
        self.current_path = Some(path.clone());
        let map = Self::extract_pallete(&self.current_path.clone().unwrap()).unwrap();

        // let full_pallette = extract_pallete(pal_name, &file_path).unwrap();
        self.all_entries = Self::get_sorted_entries(map);
        self.update_top_colors();
    }

    pub fn update_top_colors(&mut self) {
        let top_colors = Self::get_top_colors(self.all_entries.clone(), self.pallette_size);
        self.top_colors = top_colors;
    }

    pub fn get_unused_entry(&mut self) -> Option<Rgb<u8>> {
        for e in self.all_entries.iter() {
            if !self.top_colors.contains(&e.0) {
                return Some(e.0.clone());
            }
        }
        None
    }

    pub fn swap_top_color(&mut self, idx: usize) {
        println!("Swap top color {idx}");
        let e = self.get_unused_entry();
        // println!("Swap top color {idx}");
        match e {
            Some(c) => self.top_colors[idx] = c,
            None => (),
        }
        // self.top_colors[idx] =
    }

    pub fn save(&mut self, pallette_name: String) {
        // let i =rand
        Self::output_pallette(self.top_colors.clone(), &pallette_name);
    }

    pub fn rgb_to_hex(color: Rgb<u8>) -> String {
        format!("#{:02X}{:02X}{:02X}", color[0], color[1], color[2])
    }

    fn get_sorted_entries(pallette: HashMap<Rgb<u8>, usize>) -> Vec<(Rgb<u8>, usize)> {
        let mut entries: Vec<(Rgb<u8>, usize)> = pallette.into_iter().collect();

        // Sort by the count in descending order
        entries.sort_by(|a, b| b.1.cmp(&a.1));
        entries
    }

    pub fn get_top_colors(entries: Vec<(Rgb<u8>, usize)>, top_n: usize) -> Vec<Rgb<u8>> {
        // Take the top N entries
        entries
            .into_iter()
            .map(|e| e.0)
            .take(top_n)
            // .cloned()
            .collect()
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
    fn output_pallette(colors: Vec<Rgb<u8>>, pal_name: &str) {
        println!("Output pallette");
        let square_size = 64.;
        let margin = 16.;
        let width = 512;
        let height = ((margin + square_size) * ((colors.len() as f32) / 5.) + margin) as i32;

        let mut dt = DrawTarget::new(width, height);

        let mut pb = PathBuilder::new();
        // pb.move_to(current_x, current_y);
        pb.rect(0., 0., width as f32, height as f32);
        pb.close();
        let path = pb.finish();
        // let solid = SolidSource::from_unpremultiplied_argb(0xff, 0, 0, 0);
        let solid = SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0xff, 0xff);
        dt.fill(&path, &&Source::Solid(solid), &DrawOptions::new());

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
            dt.fill(&path, &&Source::Solid(solid), &DrawOptions::new());
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
