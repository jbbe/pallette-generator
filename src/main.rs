#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example
use eframe::egui;

mod color;
mod pallette;
mod similar;
use egui::ColorImage;
use image::{DynamicImage, RgbaImage};
use pallette::Pallette;

use crate::similar::Similar;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1240.0, 840.0]) // wide enough for the drag-drop overlay text
            .with_drag_and_drop(true),
        ..Default::default()
    };
    eframe::run_native(
        "Pallette Generator",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<PalletteApp>::default())
        }),
    )
}

enum AppState {
    NoFile,
    FileSelected,
    PalletteGenerated,
}

struct PalletteApp {
    app_state: AppState,
    picked_path: Option<String>,
    pallette: Pallette,
    pallette_name: String,
    texture_id: Option<egui::TextureHandle>,
    similar: Option<Similar>,
}

impl Default for PalletteApp {
    fn default() -> Self {
        Self {
            app_state: AppState::NoFile,
            picked_path: None,
            pallette: Pallette::default(),
            pallette_name: "New Pallette".to_string(),
            texture_id: None,
            similar: None,
        }
    }
}

impl eframe::App for PalletteApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.group(|ui| {
                    ui.set_min_size(egui::Vec2::new(500., 700.));
                    ui.vertical(|ui| match self.app_state {
                        AppState::NoFile => {
                            ui.centered_and_justified(|ui| {
                                file_picker(ui, self);
                                // Collect dropped files:
                                ctx.input(|i| {
                                    if !i.raw.dropped_files.is_empty() {
                                        if let Some(f_path) = &i.raw.dropped_files[0].path {
                                            self.picked_path = Some(f_path.display().to_string())
                                        }
                                        self.app_state = AppState::FileSelected;
                                    }
                                });
                            });
                        }
                        AppState::FileSelected => {
                            file_info(ui, self, ctx);
                        }
                        AppState::PalletteGenerated => {
                            pallette_control_buttons(ui, self);
                            pallette_panel(ui, self, ctx);
                            similar_selector(ui, self, ctx);

                            ui.text_edit_singleline(&mut self.pallette_name);

                            save_buttons(ui, self);
                            reset_button(ui, self);
                        }
                    });
                });
                image_panel(ui, self);
            })
        });

        preview_files_being_dropped(ctx);
    }
}

fn pallette_panel(ui: &mut egui::Ui, app: &mut PalletteApp, ctx: &egui::Context) {
    let pallette_button_size = egui::vec2(100., 100.);
    // Create a grid and add items to it
    ui.horizontal(|ui| {
        ui.set_min_height(500.);
        let num_columns = 4;
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.vertical(|ui| {
                ui.set_min_height(500.);
                egui::Grid::new("Image Pallette").show(ui, |ui| {
                    for i in 0..app.pallette.top_rgb.len() {
                        let c = app.pallette.top_rgb[i];
                        let color = egui::Color32::from_rgb(c[0], c[1], c[2]);
                        let hex = &app.pallette.top_hex[i];
                        if ui
                            .add_sized(
                                pallette_button_size,
                                egui::Button::new(egui::RichText::new(hex))
                                    .fill(color)
                                    .sense(egui::Sense::click()),
                            )
                            .clicked()
                        {
                            ctx.copy_text(hex.to_owned());
                        }
                        if ui
                            .add(egui::Button::new(egui::RichText::new("Swap")))
                            .clicked()
                        {
                            app.pallette.swap_top_color(i);
                        }
                        if ui
                            .add(egui::Button::new(egui::RichText::new("Similar")))
                            .clicked()
                        {
                            app.similar = Some(Similar::new(c, &app.pallette.all_entries, 10))
                        }
                        if (i + 1) % num_columns == 0 {
                            ui.end_row(); // End the row after the specified number of columns
                        }
                    }
                });
            });
        })
    });
}

fn similar_selector(ui: &mut egui::Ui, app: &mut PalletteApp, ctx: &egui::Context) {
    if let Some(sim) = &app.similar {
        let pallette_button_size = egui::vec2(100., 100.);
        let c = sim.color;
        let color = egui::Color32::from_rgb(c[0], c[1], c[2]);
        let hex = Pallette::rgb_to_hex(c);
        // sim.similar_colors
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
        egui::ScrollArea::horizontal().show(ui, |ui| {
            // similar_grid(ui, &app, ctx, &sim.similar_colors);

            ui.set_max_width(400.);
            egui::Grid::new("Image Pallette").show(ui, |ui| {
                let pallette_button_size = egui::vec2(100., 100.);
                // for i in 0..sim.similar_colors.len() {
                for entry in sim.similar_colors.iter() {
                    // let entry = sim.similar_colors[i];
                    let c = entry.0;
                    let color = egui::Color32::from_rgb(c[0], c[1], c[2]);
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
                        .add(egui::Button::new(egui::RichText::new("Replace")))
                        .clicked()
                    {
                        app.pallette.update_color(sim.color, c);
                        // app.similar = None
                    }
                }
            });
        });
    }
}

fn pallette_control_buttons(ui: &mut egui::Ui, app: &mut PalletteApp) {
    let p_size = app.pallette.pallette_size;
    ui.horizontal(|ui| {
        ui.label(format!("Pallette Size: {p_size}"));
        if ui.button("-").clicked() {
            app.pallette.decrement_pallette_size();
        }
        if ui.button("+").clicked() {
            app.pallette.increment_pallette_size();
        }
    });
}

fn save_buttons(ui: &mut egui::Ui, app: &mut PalletteApp) {
    if ui.button("Save as PNG").clicked() {
        println!("Save clicked");
        app.pallette.save_pallette_img(app.pallette_name.clone())
    }
    if ui.button("Save as Text").clicked() {
        println!("Save clicked");
        app.pallette.save_pallette_text(app.pallette_name.clone())
    }
}

fn reset_button(ui: &mut egui::Ui, app: &mut PalletteApp) {
    if ui.button("Reset").clicked() {
        println!("reset");
        app.pallette.reset();
        app.picked_path = None;
        app.app_state = AppState::NoFile;
        app.texture_id = None;
    }
}

fn file_info(ui: &mut egui::Ui, app: &mut PalletteApp, ctx: &egui::Context) {
    if let Some(picked_path) = &app.picked_path {
        if let Ok(img) = load_image(picked_path) {
            let color_image = convert_img_for_display(img);
            app.texture_id = Some(ctx.load_texture("my_image", color_image, Default::default()));
        }
        ui.horizontal(|ui| {
            ui.label("Picked file:");
            ui.monospace(picked_path);
        });
        if ui.button("Extract Pallette").clicked() {
            app.pallette.update(picked_path);
            app.app_state = AppState::PalletteGenerated;
        }
    }
}
fn file_picker(ui: &mut egui::Ui, app: &mut PalletteApp) {
    if app.picked_path.is_none() {
        ui.label("Drag and drop a file to create a pallette");

        if ui.button("Open file…").clicked()
            && let Some(path) = rfd::FileDialog::new().pick_file()
        {
            app.picked_path = Some(path.display().to_string());
            app.app_state = AppState::FileSelected;
        }
    }
}

fn image_panel(ui: &mut egui::Ui, app: &PalletteApp) {
    ui.horizontal(|ui| {
        ui.set_min_width(400.);
        ui.centered_and_justified(|ui| match app.app_state {
            AppState::NoFile => {
                ui.set_min_size(egui::Vec2::new(200., 200.));
                ui.image(egui::include_image!("assets/pallette.svg"));
            }
            AppState::FileSelected | AppState::PalletteGenerated => {
                ui.horizontal(|ui| {
                    if let Some(texture_id) = &app.texture_id {
                        let desired_size = egui::vec2(400.0, 500.0);
                        ui.add(egui::Image::new(texture_id).fit_to_exact_size(desired_size));
                    } else {
                        ui.label("Loading image...");
                    }
                });
            }
        });
    });
}

// Function to load an image and return it as an Rgba image
fn load_image(path: &str) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    let img = image::open(std::path::Path::new(path))?;
    Ok(img)
}

fn convert_img_for_display(img: DynamicImage) -> ColorImage {
    let rgba_image: RgbaImage = img.to_rgba8();
    ColorImage::from_rgba_unmultiplied(
        [rgba_image.width() as usize, rgba_image.height() as usize],
        rgba_image.as_raw(),
    )
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
