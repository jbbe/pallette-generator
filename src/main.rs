#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example
use eframe::egui;

mod color;
mod color_detail;
mod pallette;
mod similar;
use egui::ColorImage;
use image::{DynamicImage, Rgb, RgbaImage};
use pallette::Pallette;

use crate::{color::ColorUtil, color_detail::ColorDetail, similar::Similar};

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
    panel_width: f32,
    pallette_button_size: egui::Vec2,
    show_details: Option<usize>,
    new_color: ColorDetail,
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
            panel_width: 400.,
            pallette_button_size: egui::vec2(100., 100.),
            show_details: None,
            new_color: ColorDetail::default(),
        }
    }
}

impl eframe::App for PalletteApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.panel_width = (ui.available_width() - 20.0) / 2.0;
            ui.horizontal(|ui| {
                ui.group(|ui| {
                    ui.set_min_size(egui::Vec2::new(self.panel_width, 700.));
                    ui.vertical(|ui| match self.app_state {
                        AppState::NoFile => {
                            self.no_file_view(ui, ctx);
                        }
                        AppState::FileSelected => {
                            self.file_info(ui, ctx);
                        }
                        AppState::PalletteGenerated => {
                            self.pallette_control_buttons(ui);
                            self.pallette_panel(ui, ctx);
                            self.color_options_panel(ui, ctx);
                            self.similar_selector(ui, ctx);

                            ui.text_edit_singleline(&mut self.pallette_name);

                            self.save_buttons(ui);
                            self.reset_button(ui);
                        }
                    });
                });
                ui.add_space(20.);
                ui.group(|ui| {
                    ui.set_min_size(egui::Vec2::new(self.panel_width, 700.0));
                    self.image_panel(ui);
                });
            });
        });

        preview_files_being_dropped(ctx);
    }
}

impl PalletteApp {
    fn no_file_view(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        // ui.centered_and_justified(|ui| {
        ui.vertical_centered(|ui| {
            ui.set_min_width(400.);
            self.file_picker(ui);
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

    fn similar_selector(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let mut show_close = false;
        if let Some(sim) = &self.similar {
            show_close = true;
            let pallette_button_size = egui::vec2(100., 100.);
            let c = sim.color;
            let color = egui::Color32::from_rgb(c[0], c[1], c[2]);
            let hex = ColorUtil::rgb_to_hex(c);
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
                let hex = ColorUtil::rgb_to_hex(c);
                // println!("Copy {hex}");
                ctx.copy_text(hex.to_owned());
            }
            egui::ScrollArea::horizontal()
                .max_width(600.)
                .show(ui, |ui| {
                    egui::Grid::new("Similar Colors").show(ui, |ui| {
                        let pallette_button_size = egui::vec2(100., 100.);
                        // for i in 0..sim.similar_colors.len() {
                        for entry in sim.similar_colors.iter() {
                            let c = entry.0;
                            let color = egui::Color32::from_rgb(c[0], c[1], c[2]);
                            let hex = ColorUtil::rgb_to_hex(c);
                            if ui
                                .add_sized(
                                    pallette_button_size,
                                    egui::Button::new(egui::RichText::new(hex))
                                        .fill(color)
                                        .sense(egui::Sense::click()),
                                )
                                .clicked()
                            {
                                let hex = ColorUtil::rgb_to_hex(c);
                                // println!("Copy {hex}");
                                ctx.copy_text(hex.to_owned());
                            }
                            if ui
                                .add(egui::Button::new(egui::RichText::new("Replace")))
                                .clicked()
                            {
                                // self.update_similar_pallette_color(sim.color, c);
                                self.pallette.update_color(sim.color, c);
                                // app.similar = None
                            }
                        }
                    });
                });
        }
        if show_close {
            if ui
                .add(egui::Button::new(egui::RichText::new("Close")))
                .clicked()
            {
                self.similar = None;
            }
        }
    }

    fn pallette_panel(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        // Create a grid and add items to it
        ui.horizontal(|ui| {
            ui.set_min_height(500.);
            let num_columns = 4;
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.set_min_height(500.);
                    egui::Grid::new("Image Pallette").show(ui, |ui| {
                        for i in 0..self.pallette.top_rgb.len() {
                            self.pallette_color(ui, ctx, i);
                            if (i + 1) % num_columns == 0 {
                                ui.end_row(); // End the row after the specified number of columns
                            }
                        }
                    });
                });
            })
        });
    }

    fn color_options_panel(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        if let Some(idx) = self.show_details {
            let c = self.pallette.top_rgb[idx];
            if ui
                .add(egui::Button::new(egui::RichText::new("Complement")))
                .clicked()
            {
                self.pallette.add_complementary(c);
            }
            if ui
                .add(egui::Button::new(egui::RichText::new("Similar")))
                .clicked()
            {
                self.similar = Some(Similar::new_similar(c, &self.pallette.all_entries, 10, 80.))
            }
        } else {
            self.add_color(ui, ctx);
        }
    }

    fn add_color(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.label("Add Color");
        ui.color_edit_button_srgba(&mut self.new_color.egui_color);
        // ToDo Update rest of color info on color change
        if ui
            .add_sized(
                self.pallette_button_size,
                egui::Button::new(egui::RichText::new("Add"))
                    .fill(self.new_color.egui_color)
                    .sense(egui::Sense::click()),
            )
            .clicked()
        {
            // let new_col = Rgb([self.new_color.r(), self.new_color.g(), self.new_color.b()]);
            self.pallette.add_new_color(self.new_color.color);
        }

        if ui
            .add(egui::Button::new(egui::RichText::new("Copy")))
            .clicked()
        {
            ctx.copy_text(self.new_color.hex.to_owned());
        }
    }

    fn pallette_color(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, i: usize) {
        // let pallette_button_size = ;
        let c = self.pallette.top_rgb[i];
        let hex = &self.pallette.top_hex[i];
        let color = egui::Color32::from_rgb(c[0], c[1], c[2]);
        if ui
            .add_sized(
                self.pallette_button_size,
                egui::Button::new(egui::RichText::new(hex))
                    .fill(color)
                    .sense(egui::Sense::click()),
            )
            .clicked()
        {
            ctx.copy_text(hex.to_owned());
        }
        ui.vertical(|ui| {
            ui.set_min_width(90.);
            if ui
                .add(egui::Button::new(egui::RichText::new("Swap")))
                .clicked()
            {
                self.pallette.swap_top_color(i);
            }
            if ui
                .add(egui::Button::new(egui::RichText::new("Options")))
                .clicked()
            {
                self.show_details = Some(i);
            }
            // if ui
            //     .add(egui::Button::new(egui::RichText::new("Complement")))
            //     .clicked()
            // {
            //     self.pallette.add_complementary(c);
            // }
            // if ui
            //     .add(egui::Button::new(egui::RichText::new("Similar")))
            //     .clicked()
            // {
            //     self.similar = Some(Similar::new_similar(c, &self.pallette.all_entries, 10, 80.))
            // }
        });
    }

    fn pallette_control_buttons(&mut self, ui: &mut egui::Ui) {
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
    }

    fn save_buttons(&mut self, ui: &mut egui::Ui) {
        if ui.button("Save as PNG").clicked() {
            println!("Save clicked");
            self.pallette.save_pallette_img(self.pallette_name.clone())
        }
        if ui.button("Save as Text").clicked() {
            println!("Save clicked");
            self.pallette.save_pallette_text(self.pallette_name.clone())
        }
    }

    fn reset_button(&mut self, ui: &mut egui::Ui) {
        if ui.button("Reset").clicked() {
            println!("reset");
            self.pallette.reset();
            self.picked_path = None;
            self.app_state = AppState::NoFile;
            self.texture_id = None;
            self.similar = None
        }
    }

    fn file_info(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        if let Some(picked_path) = &self.picked_path {
            if let Ok(img) = load_image(picked_path) {
                let color_image = convert_img_for_display(img);
                self.texture_id =
                    Some(ctx.load_texture("my_image", color_image, Default::default()));
            }
            ui.horizontal(|ui| {
                ui.label("Picked file:");
                ui.monospace(picked_path);
            });
            if ui.button("Extract Pallette").clicked() {
                self.pallette.update(picked_path);
                self.app_state = AppState::PalletteGenerated;
            }
        }
    }
    fn file_picker(&mut self, ui: &mut egui::Ui) {
        if self.picked_path.is_none() {
            ui.label("Drag and drop a file to create a pallette");

            if ui.button("Open file…").clicked()
                && let Some(path) = rfd::FileDialog::new().pick_file()
            {
                self.picked_path = Some(path.display().to_string());
                self.app_state = AppState::FileSelected;
            }
        }
    }

    fn image_panel(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.set_min_width(self.panel_width);
            // ui.centered_and_justified(|ui|
            match self.app_state {
                AppState::NoFile => {
                    ui.set_min_size(egui::Vec2::new(200., 200.));
                    ui.image(egui::include_image!("assets/pallette.svg"));
                }
                AppState::FileSelected | AppState::PalletteGenerated => {
                    ui.horizontal(|ui| {
                        if let Some(texture_id) = &self.texture_id {
                            let desired_size = egui::vec2(400.0, 500.0);
                            ui.add(egui::Image::new(texture_id).fit_to_exact_size(desired_size));
                        } else {
                            ui.label("Loading image...");
                        }
                    });
                }
            }
        });
    }
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
