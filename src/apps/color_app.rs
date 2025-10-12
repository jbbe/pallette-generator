
use eframe::egui;


use crate::{core::pallette::Pallette, core::color::ColorUtil, core::color_detail::ColorDetail, core::similar::Similar};

enum AppState {
    NoPallette,
}



pub struct ColorApp {
    app_state: AppState,
    // source_file_state: SourceFileState,
    // pallette: Pallette,
    // pallette_name: String,
    // texture_id: Option<egui::TextureHandle>,
    // similar: Option<Similar>,
    panel_width: f32,
    // show_details: Option<(usize, ColorDetail)>,
    color: ColorDetail,
}

const PALLETTE_BUTTON_SIZE: egui::Vec2 = egui::vec2(100., 100.);
impl Default for ColorApp {
    fn default() -> Self {
        Self {
            app_state: AppState::NoPallette,
            // source_file_state: SourceFileState::NoFile,
            // pallette: Pallette::default(),
            // pallette_name: "New Pallette".to_string(),
            // texture_id: None,
            // similar: None,
            panel_width: 400.,
            // show_details: None,
            color: ColorDetail::default(),
        }
    }
}

impl eframe::App for ColorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.panel_width = (ui.available_width() - 20.0) / 2.0;
            ui.horizontal(|ui| {
                ui.label("Colors")
            });
        });

    }
}

impl ColorApp {}
 