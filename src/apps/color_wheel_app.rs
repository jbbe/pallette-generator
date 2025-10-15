use crate::core::color_names::ColorNames;
use crate::{core::color_detail::ColorDetail, widgets::custom_color_edit_button_srgba};
use eframe::egui;
use image::Rgb;

pub struct ColorApp {
    color: ColorDetail,
}

const palette_BUTTON_SIZE: egui::Vec2 = egui::vec2(100., 100.);
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

impl ColorApp {}
