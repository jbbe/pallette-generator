use crate::core::color_names::ColorNames;
use crate::{core::color_detail::ColorDetail, widgets::custom_color_edit_button_srgba};
use eframe::egui;
use image::Rgb;

pub struct ColorApp {
    color: ColorDetail,
}

const PALLETTE_BUTTON_SIZE: egui::Vec2 = egui::vec2(100., 100.);
impl Default for ColorApp {
    fn default() -> Self {
        Self {
            color: ColorDetail::default(),
        }
    }
}

impl eframe::App for ColorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| self.color_options_panel(ui, ctx));
    }
}

impl ColorApp {
    fn color_button(ui: &mut egui::Ui, color: egui::Color32, text: &str) -> egui::Response {
        ui.add_sized(
            PALLETTE_BUTTON_SIZE,
            egui::Button::new(egui::RichText::new(text))
                .fill(color)
                .sense(egui::Sense::click()),
        )
    }
    fn color_options_panel(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        custom_color_edit_button_srgba(ui, &mut self.color.egui_color);
        ui.vertical(|ui| {
            ui.vertical(|ui| {
                ui.label("Color");
                self.color.update_from_egui_color(true);
                if Self::color_button(ui, self.color.egui_color, &self.color.hex).clicked() {
                    ctx.copy_text(self.color.hex.to_owned());
                }
                self.color_info(ui, &self.color.color);
            });
            ui.add_space(50.);
            // if ui
            //     .add(egui::Button::new(egui::RichText::new("Similar")))
            //     .clicked()
            // {
            //     self.similar = Some(Similar::new_similar(
            //         detail.color,
            //         &self.pallette.all_entries,
            //         10,
            //         80.,
            //     ))
            // }
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Complement");
                    if Self::color_button(
                        ui,
                        self.color.compliment_egui,
                        &self.color.complement_hex,
                    )
                    .clicked()
                    {
                        ctx.copy_text(self.color.complement_hex.to_owned());
                    }
                    self.color_info(ui, &self.color.complement);
                });
            });
            ui.add_space(50.);
            ui.horizontal(|ui| {
                ui.label("Split Compliment");
                ui.vertical(|ui| {
                    if Self::color_button(
                        ui,
                        self.color.split_complement_egui.0,
                        &self.color.split_complement_hex.0,
                    )
                    .clicked()
                    {
                        ctx.copy_text(self.color.split_complement_hex.0.to_owned());
                    }
                    self.color_info(ui, &self.color.split_complement.0);
                });
                ui.vertical(|ui| {
                    if Self::color_button(
                        ui,
                        self.color.split_complement_egui.1,
                        &self.color.split_complement_hex.1,
                    )
                    .clicked()
                    {
                        ctx.copy_text(self.color.split_complement_hex.1.to_owned());
                    }
                    self.color_info(ui, &self.color.split_complement.1);
                });
            });
        });
    }

    fn color_info(&self, ui: &mut egui::Ui, color: &Rgb<u8>) {
        let c_name = ColorNames::get_color_name(color);
        if let Some(name) = c_name {
            ui.label(name);
        }
    }
}
