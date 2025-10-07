#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example
use eframe::egui;

mod color;
mod pallette;
use pallette::Pallette;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1240.0, 840.0]) // wide enough for the drag-drop overlay text
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
    pallette: Pallette,
    display: bool,
    pallette_name: String,
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
                if ui.button("Extract Pallette").clicked() {
                    self.pallette.update(picked_path);
                    self.display = true
                }
            }
            if self.display {
                let p_size = self.pallette.pallette_size;
                ui.horizontal(|ui| {
                    ui.label(format!("Pallette Size: {p_size}"));
                    if ui.button("-").clicked() {
                        self.pallette.decrement_pallette_size();
                    }
                    if ui.button("+").clicked() {
                        self.pallette.increment_pallette_size();
                    }
                });

                let num_columns = 4; // Set the desired number of columns
                let pallette_button_size = egui::vec2(100., 100.);
                // Create a grid and add items to it
                ui.horizontal(|ui| {
                    ui.group(|ui| {
                        egui::Grid::new("my_grid").show(ui, |ui| {
                            for i in 0..self.pallette.top_colors.len() {
                                let c = self.pallette.top_colors[i];
                                let color = egui::Color32::from_rgb(c[0], c[1], c[2]);
                                if ui
                                    .add_sized(
                                        pallette_button_size,
                                        egui::Button::new(egui::RichText::new("Copy"))
                                            .fill(color)
                                            .sense(egui::Sense::click()),
                                    )
                                    .clicked()
                                {
                                    let hex = Pallette::rgb_to_hex(c);
                                    println!("Copy {hex}");
                                    ctx.copy_text(hex.to_owned());
                                }
                                if ui
                                    .add(egui::Button::new(egui::RichText::new("Swap")))
                                    .clicked()
                                {
                                    self.pallette.swap_top_color(i);
                                }
                                if (i + 1) % num_columns == 0 {
                                    ui.end_row(); // End the row after the specified number of columns
                                }
                            }
                        });
                    });
                });

                ui.text_edit_singleline(&mut self.pallette_name);

                if ui.button("Save").clicked() {
                    println!("Save clicked");
                    self.pallette.save(self.pallette_name.clone())
                }
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
                        if ui.button(format!("Extract Pallette from {info}")).clicked() {
                            self.pallette.update(&info);
                            self.display = true
                        }

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
