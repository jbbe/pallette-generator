use eframe::egui;

use crate::core::color_detail::ColorDetail;

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
        egui::CentralPanel::default().show(ctx, |ui| {
            // self.panel_width = (ui.available_width() - 20.0) / 2.0;
            // ui.centered_and_justified(|ui| {
                self.color_options_panel(ui, ctx)

            // });
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
        ui.color_edit_button_srgba(&mut self.color.egui_color);
        ui.vertical(|ui| {
            ui.vertical(|ui| {
                ui.label("Color");
                self.color.update_from_egui_color(true);
                if Self::color_button(ui, self.color.egui_color, &self.color.hex).clicked() {
                    ctx.copy_text(self.color.hex.to_owned());
                }
                if ui
                    .add(egui::Button::new(egui::RichText::new("Add")))
                    .clicked()
                {
                    println!("clicking this does nothing")
                }
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
                });
            })
        });
    }
}
