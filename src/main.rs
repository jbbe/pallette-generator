use raqote::*;
use std::{env, fs};

pub struct PColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl PColor {
    pub fn default(color: String) -> Self {
        let r_in = u8::from_str_radix(&color[1..3], 16).unwrap();
        let g_in = u8::from_str_radix(&color[3..5], 16).unwrap();
        let b_in = u8::from_str_radix(&color[5..7], 16).unwrap();
        // println!("color {r_in}, {g_in}, {b_in}");
        return Self {
            r: r_in,
            g: g_in,
            b: b_in,
        };
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // let query = &args[1];
    let pal_name = &args[1];
    let file_path = &args[2];

    // println!("In file {file_path}");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // println!("With text:\n{contents}");

    let colors: Vec<&str> = contents.split("\n").collect();

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
        let p_col = PColor::default(color.to_string());
        let solid = SolidSource::from_unpremultiplied_argb(0xff, p_col.r, p_col.g, p_col.b);
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
