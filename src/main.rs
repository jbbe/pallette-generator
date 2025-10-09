#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example
use eframe::egui;

mod color;
mod pallette;
use egui::ColorImage;
use image::{DynamicImage, RgbaImage};
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
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<MyApp>::default())
        }),
    )
}

#[derive(Default)]
struct MyApp {
    dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
    pallette: Pallette,
    img_display: bool,
    pallette_display: bool,
    pallette_name: String,
    texture_id: Option<egui::TextureHandle>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.group(|ui| {
                    ui.set_min_size(egui::Vec2::new(500., 700.));

                    ui.vertical(|ui| {
                        if let Some(picked_path) = &self.picked_path {
                            if let Ok(img) = load_image(&picked_path) {
                                let color_image = convert_img_for_display(img);
                                self.texture_id = Some(ctx.load_texture(
                                    "my_image",
                                    color_image,
                                    Default::default(),
                                ));
                            }
                            ui.horizontal(|ui| {
                                ui.label("Picked file:");
                                ui.monospace(picked_path);
                            });
                            if ui.button("Extract Pallette").clicked() {
                                self.pallette.update(&picked_path);
                                self.pallette_display = true;
                                self.img_display = true;
                            }
                        } else {
                            ui.label("Drag-and-drop files onto the window!");

                            if ui.button("Open file…").clicked()
                                && let Some(path) = rfd::FileDialog::new().pick_file()
                            {
                                self.picked_path = Some(path.display().to_string());
                                self.img_display = true;
                            }
                        }
                        if self.pallette_display {
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

                            let pallette_button_size = egui::vec2(100., 100.);
                            // Create a grid and add items to it
                            ui.horizontal(|ui| {
                                ui.set_min_height(500.);
                                let num_columns = 4;
                                egui::ScrollArea::vertical().show(ui, |ui| {
                                    ui.vertical(|ui| {
                                        ui.set_min_height(500.);
                                        egui::Grid::new("Image Pallette").show(ui, |ui| {
                                            for i in 0..self.pallette.top_colors.len() {
                                                let c = self.pallette.top_colors[i];
                                                let color =
                                                    egui::Color32::from_rgb(c[0], c[1], c[2]);
                                                let hex = Pallette::rgb_to_hex(c);
                                                if ui
                                                    .add_sized(
                                                        pallette_button_size,
                                                        egui::Button::new(egui::RichText::new(hex))
                                                            .fill(color)
                                                            .sense(egui::Sense::click()),
                                                    )
                                                    .clicked()
                                                {
                                                    let hex = Pallette::rgb_to_hex(c);
                                                    // println!("Copy {hex}");
                                                    ctx.copy_text(hex.to_owned());
                                                }
                                                if ui
                                                    .add(egui::Button::new(egui::RichText::new(
                                                        "Swap",
                                                    )))
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
                                })
                            });

                            ui.text_edit_singleline(&mut self.pallette_name);

                            if ui.button("Save as PNG").clicked() {
                                println!("Save clicked");
                                self.pallette.save_pallette_img(self.pallette_name.clone())
                            }
                            if ui.button("Save as Text").clicked() {
                                println!("Save clicked");
                                self.pallette.save_pallette_text(self.pallette_name.clone())
                            }

                            if ui.button("Reset").clicked() {
                                println!("reset");
                                self.pallette.reset();
                                self.picked_path = None;
                                self.pallette_display = false;
                                self.img_display = false;
                                self.texture_id = None;
                                self.dropped_files = Vec::new();
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
                                    if ui.button(format!("Extract Pallette from {info}")).clicked()
                                    {
                                        // handle_extract_click(&info)
                                        self.pallette.update(&info);
                                        self.pallette_display = true;
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
                });
                image_panel(ui, self);
            })
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

fn image_panel(ui: &mut egui::Ui, app: &MyApp) {
    ui.horizontal(|ui| {
        ui.set_min_width(400.);
        if app.img_display {
            ui.horizontal(|ui| {
                if let Some(texture_id) = &app.texture_id {
                    let desired_size = egui::vec2(400.0, 500.0);
                    ui.add(egui::Image::new(texture_id).fit_to_exact_size(desired_size));
                } else {
                    ui.label("Loading image...");
                }
            });
        } else {
            ui.image(egui::include_image!("assets/pallette.svg"));
        }
    });
}

// Function to load an image and return it as an Rgba image
fn load_image(path: &str) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    let img = image::open(std::path::Path::new(path))?;
    Ok(img)
}

fn convert_img_for_display(img: DynamicImage) -> ColorImage {
    let rgba_image: RgbaImage = img.to_rgba8();
    let color_image = ColorImage::from_rgba_unmultiplied(
        [rgba_image.width() as usize, rgba_image.height() as usize],
        rgba_image.as_raw(),
    );
    return color_image;
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
