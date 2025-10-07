#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example
use image::{ImageReader, Rgb};
use raqote::*;
use std::{collections::HashMap, env, fs};
use eframe::egui;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct PColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

// fn color_dist(a: Rgb<u8>, b: Rgb<u8>) -> u32 {
    
// }

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

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([640.0, 240.0]) // wide enough for the drag-drop overlay text
            .with_drag_and_drop(true),
        ..Default::default()
    };
    eframe::run_native(
        "Native file dialogs and drag-and-drop files",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {
    dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Drag-and-drop files onto the window!");

            if ui.button("Open file…").clicked()
                && let Some(path) = rfd::FileDialog::new().pick_file()
            {
                self.picked_path = Some(path.display().to_string());
            }

            if let Some(picked_path) = &self.picked_path {
                ui.horizontal(|ui| {
                    ui.label("Picked file:");
                    ui.monospace(picked_path);
                });
            }

            // Show dropped files (if any):
            if !self.dropped_files.is_empty() {
                ui.group(|ui| {
                    ui.label("Dropped files:");

                    for file in &self.dropped_files {
                        let mut info = if let Some(path) = &file.path {
                            path.display().to_string()
                        } else if !file.name.is_empty() {
                            file.name.clone()
                        } else {
                            "???".to_owned()
                        };

                        let mut additional_info = vec![];
                        if !file.mime.is_empty() {
                            additional_info.push(format!("type: {}", file.mime));
                        }
                        if let Some(bytes) = &file.bytes {
                            additional_info.push(format!("{} bytes", bytes.len()));
                        }
                        if !additional_info.is_empty() {
                            info += &format!(" ({})", additional_info.join(", "));
                        }

                        ui.label(info);
                    }
                });
            }
        });

        preview_files_being_dropped(ctx);

        // Collect dropped files:
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                self.dropped_files.clone_from(&i.raw.dropped_files);
            }
        });
    }
}

/// Preview hovering files:
fn preview_files_being_dropped(ctx: &egui::Context) {
    use egui::{Align2, Color32, Id, LayerId, Order, TextStyle};
    use std::fmt::Write as _;

    if !ctx.input(|i| i.raw.hovered_files.is_empty()) {
        let text = ctx.input(|i| {
            let mut text = "Dropping files:\n".to_owned();
            for file in &i.raw.hovered_files {
                if let Some(path) = &file.path {
                    write!(text, "\n{}", path.display()).ok();
                } else if !file.mime.is_empty() {
                    write!(text, "\n{}", file.mime).ok();
                } else {
                    text += "\n???";
                }
            }
            text
        });

        let painter =
            ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

        let screen_rect = ctx.screen_rect();
        painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
        painter.text(
            screen_rect.center(),
            Align2::CENTER_CENTER,
            text,
            TextStyle::Heading.resolve(&ctx.style()),
            Color32::WHITE,
        );
    }
}

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     let prog = &args[1];
//     let pal_name = &args[2];
//     let file_path = &args[3];

//     if prog == "gen" {
//         println!("Generating pallette {pal_name}");

//         let contents =
//             fs::read_to_string(file_path).expect("Should have been able to read the file");

//         let colors: Vec<&str> = contents.split("\n").collect();
//         let p_colors = colors
//             .iter()
//             // .map(|&color| PColor::from_string(color.to_string()))
//             .map(|&color| rgb_from_str(color))
//             .collect();
//         output_pallette(p_colors, pal_name)
//     }
//     if prog == "extract" {
//         let full_pallette = extract_pallete(pal_name, &file_path).unwrap();

//         let top_colors = get_top_colors(full_pallette, 50);
//         // let top_colors = reduce_pallette(full_pallette, 10);
//         output_pallette(top_colors, pal_name);
//     }
// }

// #region core
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

