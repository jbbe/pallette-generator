use image::{ImageReader, Rgb};
use raqote::*;
use std::{collections::HashMap, env, fs};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct PColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl PColor {
    pub fn from_string(color: String) -> Self {
        let r_in = u8::from_str_radix(&color[1..3], 16).unwrap();
        let g_in = u8::from_str_radix(&color[3..5], 16).unwrap();
        let b_in = u8::from_str_radix(&color[5..7], 16).unwrap();
        // println!("color {r_in}, {g_in}, {b_in}");
        return Self {
            r: r_in,
            g: g_in,
            b: b_in,
            // frequency: 0,
        };
    }
    pub fn new(r_in: u8, g_in: u8, b_in: u8) -> Self {
        Self {
            r: r_in,
            g: g_in,
            b: b_in,
            // frequency: 0,
        }
    }

}

// impl Eq for PColor {
//     fn eq(&self, other: &Self) -> bool {
//         self.r == other.r && self.g == other.g && self.b == other.b
//     }
// }  

// fn rgb_eq(p_color: &PColor, rgb: &Rgb<u8>) -> bool {
//     p_color.r == rgb[0] && p_color.g == rgb[1] && p_color.b == rgb[2]
// }

fn rgb_from_str(color: &str) -> Rgb<u8> {
    let r_in = u8::from_str_radix(&color[1..3], 16).unwrap();
    let g_in = u8::from_str_radix(&color[3..5], 16).unwrap();
    let b_in = u8::from_str_radix(&color[5..7], 16).unwrap();
    Rgb([r_in, g_in, b_in])
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let prog = &args[1];
    let pal_name = &args[2];
    let file_path = &args[3];

    if prog == "gen" {
        println!("Generating pallette {pal_name}");

        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        let colors: Vec<&str> = contents.split("\n").collect();
        let p_colors = colors
            .iter()
            // .map(|&color| PColor::from_string(color.to_string()))
            .map(|&color| rgb_from_str(color))
            .collect();
        output_pallette(p_colors, pal_name)
    }
    if prog == "extract" {
        let full_pallette = extract_pallete(pal_name, &file_path).unwrap();

        let top_colors = get_top_colors(full_pallette, 50);
        // let top_colors = reduce_pallette(full_pallette, 10);
        output_pallette(top_colors, pal_name);
    }
}

fn get_top_colors(pallette: HashMap<Rgb<u8>, usize>, top_n: usize) -> Vec<Rgb<u8>> {
     let mut entries: Vec<(&Rgb<u8>, &usize)> = pallette.iter().collect();

    // Sort by the count in descending order
    entries.sort_by(|a, b| b.1.cmp(a.1));

    // Take the top N entries
    entries.into_iter().map(|e| { e.0 }).take(top_n).cloned().collect() 
}

fn extract_pallete(pal_name: &str, path: &str) -> Option<HashMap<Rgb<u8>, usize>> {
    println!("Extracting Pallette from {pal_name} ");
    let img = ImageReader::open(path).unwrap().decode().unwrap();
    let rgb = img.to_rgb8();
    // let mut pixels = Vec::<PColor>::new();
    let mut pix = HashMap::<Rgb<u8>, usize>::new();
    for pixel in rgb.pixels() {
        *pix.entry(*pixel).or_insert(0) += 1
    }

    Some(pix)
}

fn output_pallette(colors: Vec<Rgb<u8>>, pal_name: &str) {
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

fn reduce_pallette(mut pallette: Vec<PColor>, color_count: usize) -> Vec<PColor> {
    let plen = pallette.len();
    println!("reduce pallette {color_count} {plen}");
    if pallette.len() < color_count {
        // pallette.sort_by(|a, b| a.frequency.cmp(&b.frequency));
        // pallette
        // .sort_by(|a, b| { a.frequency })
        pallette
    } else {
        // let mut ret = Vec::<PColor>::new();
        pallette[0..color_count].to_vec()
        // ret.copy_from_slice(&pallette[0..color_count]);
        // ret
    }
}

