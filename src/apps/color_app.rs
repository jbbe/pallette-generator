
use eframe::egui;


use crate::{core::pallette::Pallette, core::color::ColorUtil, core::color_detail::ColorDetail, core::similar::Similar};

// enum AppState {
//     NoPallette,
// }



pub struct ColorApp {
    // app_state: AppState,
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
            // app_state: AppState::NoPallette,
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
                ui.label("Colors");
                self.color_options_panel(ui, ctx);
            });
        });

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
            // if let Some((_idx, detail)) = &self.show_details {
        let detail = &self.color;
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Color");
                    if Self::color_button(ui, detail.egui_color, &detail.hex).clicked() {
                        ctx.copy_text(detail.hex.to_owned());
                    }
                    if ui
                        .add(egui::Button::new(egui::RichText::new("Add")))
                        .clicked()
                    {
                        println!("clicking this does nothing")
                    }
                });
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
                ui.vertical(|ui| {
                    ui.label("Complement");
                    if Self::color_button(ui, detail.compliment_egui, &detail.complement_hex)
                        .clicked()
                    {
                        ctx.copy_text(detail.complement_hex.to_owned());
                    }
                    // if ui
                    //     .add(egui::Button::new(egui::RichText::new("Add")))
                    //     .clicked()
                    // {
                    //     self.pallette.add_new_color(detail.complement);
                    // }
                });
            });
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
        }
}
 