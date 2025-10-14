use eframe::egui;

use egui::{ColorImage, UserData, ViewportCommand};
use image::{DynamicImage, Rgb, RgbaImage};

use crate::{
    core::{
        color::ColorUtil, color_detail::ColorDetail, color_names::ColorNames, pallette::Pallette,
        similar::Similar,
    },
    widgets::custom_color_edit_button_srgba,
};

enum AppState {
    NoPallette,
    PalletteFromImgGenerated,
    PalletteGenerated,
}

enum SourceFileState {
    NoFile,
    File,
}

pub struct PalletteApp {
    app_state: AppState,
    source_file_state: SourceFileState,
    picked_path: Option<String>,
    pallette: Pallette,
    pallette_name: String,
    texture_id: Option<egui::TextureHandle>,
    similar: Option<Similar>,
    panel_width: f32,
    show_details: Option<ColorDetail>,
    new_color: ColorDetail,
    color_picking: bool,
    last_color_picked: Option<Rgb<u8>>,
    pallette_list: Vec<Pallette>,
}

const PALLETTE_BUTTON_SIZE: egui::Vec2 = egui::vec2(100., 100.);
impl Default for PalletteApp {
    fn default() -> Self {
        Self {
            app_state: AppState::NoPallette,
            source_file_state: SourceFileState::NoFile,
            picked_path: None,
            pallette: Pallette::default(),
            pallette_name: "New Pallette".to_string(),
            texture_id: None,
            similar: None,
            panel_width: 400.,
            show_details: None,
            new_color: ColorDetail::default(),
            color_picking: false,
            last_color_picked: None,
            pallette_list: Vec::new(),
        }
    }
}

impl eframe::App for PalletteApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let is_open = true;
        egui::SidePanel::left("pallette_panel")
            .resizable(false)
            .show_animated(ctx, is_open, |ui| {
                ui.add_space(4.0);
                ui.vertical_centered(|ui| {
                    ui.heading("Pallettes");
                });

                ui.separator();
                self.pallette_list_panel(ui);
                // self.backend_panel_contents(ui, frame, &mut cmd);
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.panel_width = (ui.available_width() - 20.0) / 2.0;
            ui.horizontal(|ui| {
                ui.group(|ui| {
                    ui.set_min_size(egui::Vec2::new(self.panel_width, 700.));
                    ui.vertical(|ui| match self.app_state {
                        AppState::NoPallette => {
                            self.no_file_view(ui, ctx);
                        }
                        AppState::PalletteGenerated | AppState::PalletteFromImgGenerated => {
                            self.pallette_control_buttons(ui);
                            self.pallette_panel(ui, ctx);
                            self.color_options_panel(ui, ctx);
                            self.similar_selector(ui, ctx);

                            ui.text_edit_singleline(&mut self.pallette.pallette_name);

                            self.save_buttons(ui);
                            self.reset_button(ui);
                        }
                    });
                });
                ui.add_space(20.);
                ui.group(|ui| {
                    ui.set_min_size(egui::Vec2::new(self.panel_width, 700.0));
                    self.image_panel(ui, ctx);
                });
            });

            let img_start = egui::Vec2::new(870., 60.);
            if self.color_picking {
                ui.ctx()
                    .send_viewport_cmd(ViewportCommand::Screenshot(UserData::default()));
                ctx.set_cursor_icon(egui::CursorIcon::Crosshair);
                let mouse_pos = ctx.pointer_latest_pos().unwrap_or_default();
                if mouse_pos != egui::pos2(0.0, 0.0) {
                    // Draw the panel at the mouse position
                    let panel_size = egui::Vec2::new(50., 50.);
                    let panel_anchor = egui::Pos2::new(mouse_pos.x + 20., mouse_pos.y + 20.);
                    let panel_rect = egui::Rect::from_min_size(panel_anchor, panel_size);
                    // let text_position = panel_rect.center();

                    let x = mouse_pos.x;
                    let y = mouse_pos.y;

                    if x > img_start[0] && y > img_start[1] {
                        let pixel = self.get_pixel_at(ui, x, y);
                        if let Some(p) = pixel {
                            self.last_color_picked = Some(p);
                            let r = p[0];
                            let g = p[1];
                            let b = p[2];
                            // let str = format!("({x}, {y})\n ({r}, {g}, {b})");
                            // Draw the text
                            let stroke = egui::Stroke::new(3.0, egui::Color32::from_rgb(0, 0, 0)); // Red
                            ui.painter().rect_stroke(
                                panel_rect,
                                5.0,
                                stroke,
                                egui::StrokeKind::Outside,
                            );
                            ui.painter().rect_filled(
                                panel_rect,
                                5.0,
                                egui::Color32::from_rgb(r, g, b),
                            );
                            // ui.painter().text(
                            //     text_position,
                            //     egui::Align2::CENTER_CENTER,
                            //     str,
                            //     FontId::proportional(14.0),
                            //     Color32::WHITE.gamma_multiply(1.),
                            // );
                        }
                    }
                }
            }
        });

        preview_files_being_dropped(ctx);
    }
}

impl PalletteApp {
    fn no_file_view(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        match self.source_file_state {
            SourceFileState::NoFile => (),
            SourceFileState::File => {
                if let Some(picked_path) = &self.picked_path {
                    self.pallette.update(picked_path);
                    self.app_state = AppState::PalletteFromImgGenerated;
                }
            }
        }
        // ui.centered_and_justified(|ui| {
        ui.vertical_centered(|ui| {
            ui.set_min_width(400.);
            self.file_picker(ui);
            // Collect dropped files:
            ctx.input(|i| {
                if !i.raw.dropped_files.is_empty()
                    && let Some(f_path) = &i.raw.dropped_files[0].path
                {
                    self.picked_path = Some(f_path.display().to_string());
                    // self.
                    self.source_file_state = SourceFileState::File;
                }
            });

            if Self::base_button(ui, "New Pallette").clicked() {
                self.pallette = Pallette::rand_pallette();
                self.app_state = AppState::PalletteGenerated;
            }
        });
    }

    fn get_pixel_at(&mut self, ui: &mut egui::Ui, x: f32, y: f32) -> Option<image::Rgb<u8>> {
        let image = ui.ctx().input(|i| {
            i.events
                .iter()
                .filter_map(|e| {
                    if let egui::Event::Screenshot { image, .. } = e {
                        Some(image.clone())
                    } else {
                        None
                    }
                })
                .next_back()
        });

        match image {
            Some(img) => {
                let x_u = x as usize;
                let y_u = y as usize;
                // img.pixels()
                // let [width, height] = img.();
                let idx = img.width() * y_u + x_u;

                if x_u < img.width() && y_u < img.height() {
                    let pixel = img.pixels[idx].clone();
                    // return (pixel[0], pixel[1], pixel[2], pixel[3]); // RGBA
                    Some(image::Rgb([pixel[0], pixel[1], pixel[2]]))
                } else {
                    None
                }
            }
            None => None,
        }
        //     Some(tex_handle) => {
        //         // let data = tex_handle();
        //         let [width, height] = tex_handle.size();

        //         if x_u < width && y_u < height {
        //             let pixel = tex_handle.pixel(x, y);
        //             // return (pixel[0], pixel[1], pixel[2], pixel[3]); // RGBA
        //             Some(image::Rgb([pixel[0], pixel[1], pixel[2]]))
        //         } else {
        //             None
        //         }
        //     }
        //     None => None,
        // }
        // image::Rgb([0, 0, 0])
    }

    fn similar_selector(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let mut show_close = false;
        if let Some(sim) = &self.similar {
            show_close = true;
            let c = sim.color;
            let color = egui::Color32::from_rgb(c[0], c[1], c[2]);
            let hex = ColorUtil::rgb_to_hex(c);
            if Self::color_button(ui, color, &hex).clicked() {
                let hex = ColorUtil::rgb_to_hex(c);
                // println!("Copy {hex}");
                ctx.copy_text(hex.to_owned());
            }
            Self::color_info(ui, &c);
            egui::ScrollArea::horizontal()
                .max_width(600.)
                .show(ui, |ui| {
                    egui::Grid::new("Similar Colors").show(ui, |ui| {
                        for entry in sim.similar_colors.iter() {
                            let c = entry.0;
                            let color = egui::Color32::from_rgb(c[0], c[1], c[2]);
                            let hex = ColorUtil::rgb_to_hex(c);
                            if Self::color_button(ui, color, &hex).clicked() {
                                let hex = ColorUtil::rgb_to_hex(c);
                                // println!("Copy {hex}");
                                ctx.copy_text(hex.to_owned());
                            }
                            Self::color_info(ui, &c);
                            if Self::base_button(ui, "Replace").clicked() {
                                // self.update_similar_pallette_color(sim.color, c);
                                self.pallette.update_color(sim.color, c);
                                // app.similar = None
                            }
                            if Self::base_button(ui, "Add").clicked() {
                                self.pallette.add_new_color(c);
                            }
                        }
                    });
                });
        }
        if show_close
            && ui
                .add(egui::Button::new(egui::RichText::new("Close")))
                .clicked()
        {
            self.similar = None;
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
        if let Some(detail) = &self.show_details {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Color");
                    if Self::color_button(ui, detail.egui_color, &detail.hex).clicked() {
                        ctx.copy_text(detail.hex.to_owned());
                    }
                    Self::color_info(ui, &detail.color);
                });
                if Self::base_button(ui, "Similar").clicked() {
                    self.similar = Some(Similar::new_similar(
                        detail.color,
                        &self.pallette.all_entries,
                        10,
                        80.,
                    ))
                }
                ui.vertical(|ui| {
                    ui.label("Complement");
                    if Self::color_button(ui, detail.compliment_egui, &detail.complement_hex)
                        .clicked()
                    {
                        ctx.copy_text(detail.complement_hex.to_owned());
                    }
                    Self::color_info(ui, &detail.complement);
                    // self.add_color_btn(ui, detail.complement);
                });
                ui.vertical(|ui| {
                    ui.label("Split Compliment 1");
                    if Self::color_button(
                        ui,
                        detail.split_complement_egui.0,
                        &detail.split_complement_hex.0,
                    )
                    .clicked()
                    {
                        ctx.copy_text(detail.split_complement_hex.0.to_owned());
                    }
                    Self::color_info(ui, &detail.split_complement.0);
                    // self.add_color_btn(ui, detail.complement);
                    // self.add_color_btn(ui, detail.split_complement.0);
                });
                ui.vertical(|ui| {
                    ui.label("Split Compliment 2");
                    if Self::color_button(
                        ui,
                        detail.split_complement_egui.1,
                        &detail.split_complement_hex.1,
                    )
                    .clicked()
                    {
                        ctx.copy_text(detail.split_complement_hex.1.to_owned());
                    }
                    Self::color_info(ui, &detail.split_complement.1);
                    // self.add_color_btn(ui, detail.split_complem`nt.1);
                });
            });
            if Self::base_button(ui, "Similar").clicked() {
                self.similar = Some(Similar::new_similar(
                    detail.color,
                    &self.pallette.all_entries,
                    10,
                    80.,
                ))
            }
        } else {
            self.add_color(ui, ctx);
        }
        if self.show_details.is_some() {
            if ui
                .add(egui::Button::new(egui::RichText::new("Close")))
                .clicked()
            {
                self.show_details = None;
            }
        }
    }

    // fn add_color_btn(&mut self, ui: &mut egui::Ui, color: Rgb<u8>) {
    //     if Self::base_button(ui, "Add").clicked() {
    //         self.pallette.add_new_color(color);
    //     }
    // }

    fn pallette_list_panel(&mut self, ui: &mut egui::Ui) {
        ui.label("pallette list panel");
        for p in &self.pallette_list {
            if ui.button(p.pallette_name.clone()).clicked() {
                self.pallette = p.clone();
                self.texture_id = None;
                if p.current_path.is_some() {
                    self.source_file_state = SourceFileState::File;
                    self.picked_path = p.current_path.clone();
                } else {
                    self.source_file_state = SourceFileState::NoFile
                }
            }
        }
    }

    fn add_color(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.label("Add Color");
        custom_color_edit_button_srgba(ui, &mut self.new_color.egui_color);
        // ToDo Update rest of color info on color change
        if Self::base_button(ui, "Add").clicked() {
            self.new_color.update_from_egui_color(false);
            self.pallette.add_new_color(self.new_color.color);
            self.new_color = ColorDetail::default();
        }

        if Self::base_button(ui, "Copy").clicked() {
            self.new_color.update_from_egui_color(false);
            ctx.copy_text(self.new_color.hex.to_owned());
        }
    }

    fn base_button(ui: &mut egui::Ui, text: &str) -> egui::Response {
        ui.add(egui::Button::new(egui::RichText::new(text)))
    }

    fn pallette_color(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, i: usize) {
        let c = self.pallette.top_rgb[i];
        let hex = &self.pallette.top_hex[i];
        let color = egui::Color32::from_rgb(c[0], c[1], c[2]);

        if Self::color_button(ui, color, hex).clicked() {
            ctx.copy_text(hex.to_owned());
        }
        ui.vertical(|ui| {
            ui.set_min_width(90.);
            Self::color_info(ui, &c);
            if Self::base_button(ui, "Swap").clicked() {
                self.pallette.swap_top_color(i);
            }
            if Self::base_button(ui, "Options").clicked() {
                self.show_details = Some(ColorDetail::new(self.pallette.top_rgb[i]));
            }
        });
    }

    fn color_button(ui: &mut egui::Ui, color: egui::Color32, text: &str) -> egui::Response {
        ui.add_sized(
            PALLETTE_BUTTON_SIZE,
            egui::Button::new(egui::RichText::new(text))
                .fill(color)
                .sense(egui::Sense::click()),
        )
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
        if ui.button("Save to List").clicked() {
            println!("Save to list");
            let idx = self
                .pallette_list
                .iter()
                .position(|p| p.id == self.pallette.id);
            match idx {
                Some(idx) => self.pallette_list[idx] = self.pallette.clone(),
                None => self.pallette_list.push(self.pallette.clone()),
            };
        }
    }

    fn reset_button(&mut self, ui: &mut egui::Ui) {
        if ui.button("Reset").clicked() {
            println!("reset");
            self.pallette = Pallette::default();
            self.picked_path = None;
            self.app_state = AppState::NoPallette;
            self.source_file_state = SourceFileState::NoFile;
            self.texture_id = None;
            self.similar = None
        }
    }

    fn file_picker(&mut self, ui: &mut egui::Ui) {
        if self.picked_path.is_none() {
            ui.label("Drag and drop a file to create a pallette");

            if ui.button("Open file…").clicked()
                && let Some(path) = rfd::FileDialog::new()
                    .add_filter("Images", &["png", "jpg", "jpeg", "webp"])
                    .pick_file()
            {
                self.picked_path = Some(path.display().to_string());
                self.source_file_state = SourceFileState::File;
            }
        }
    }

    fn image_panel(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.vertical_centered(|ui| {
            ui.set_min_width(self.panel_width);
            // ui.centered_and_justified(|ui|
            match self.source_file_state {
                SourceFileState::NoFile => {
                    ui.set_min_size(egui::Vec2::new(200., 200.));
                    ui.image(egui::include_image!("../assets/pallette.svg"));
                }
                SourceFileState::File => {
                    self.color_selectable_img(ui, ctx);
                }
            }
        });
    }

    fn color_selectable_img(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                if ui
                    .add(egui::ImageButton::new(egui::include_image!(
                        "../assets/eye-dropper.svg"
                    )))
                    // .sense(Sense::click())
                    .clicked()
                {
                    self.color_picking = !self.color_picking;
                }
            });
            if let Some(texture_id) = &self.texture_id {
                let desired_size = egui::vec2(300.0, 500.0);

                if ui
                    .add(
                        egui::Image::new(texture_id)
                            .fit_to_exact_size(desired_size)
                            .sense(egui::Sense::click()),
                    )
                    .clicked()
                {
                    if self.color_picking
                        && let Some(c) = self.last_color_picked
                    {
                        self.pallette.add_new_color(c);
                        self.color_picking = false;
                    }
                }
            } else {
                ui.label("Loading image...");

                if let Some(picked_path) = &self.picked_path {
                    if let Ok(img) = load_image(picked_path) {
                        let color_image = convert_img_for_display(img);
                        // self.loaded_img = Some(color_image);
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
        });
    }

    fn color_info(ui: &mut egui::Ui, color: &Rgb<u8>) {
        let c_name = ColorNames::get_color_name(color);
        if let Some(name) = c_name {
            ui.label(name);
        }
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
