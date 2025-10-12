
use eframe::egui;

// mod core;
// mod apps;
// mod debug;
use crate::apps::{ColorApp, PalletteApp};

use crate::debug::backend_panel;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Anchor {
    #[default]
    PalletteEditor,
    ColorEditor,
}
#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct State {
    pallette_app: PalletteApp,
    color_app: ColorApp,
    // demo: DemoApp,
    // easy_mark_editor: EasyMarkApp,
    // #[cfg(feature = "http")]
    // http: crate::apps::HttpApp,
    // #[cfg(feature = "image_viewer")]
    // image_viewer: crate::apps::ImageViewer,
    // pub clock: FractalClockApp,
    // rendering_test: ColorTestApp,

    selected_anchor: Anchor,
    backend_panel: backend_panel::BackendPanel,
}

/// Wraps many demo/test apps into one.
pub struct WrapApp {
    pub state: State,
}


impl WrapApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This gives us image support:
        egui_extras::install_image_loaders(&cc.egui_ctx);

        #[allow(unused_mut, clippy::allow_attributes)]
        let mut slf = Self {
            state: State::default(),
        };

        // #[cfg(feature = "persistence")]
        // if let Some(storage) = cc.storage
        //     && let Some(state) = eframe::get_value(storage, eframe::APP_KEY)
        // {
        //     slf.state = state;
        // }

        slf
    }

    pub fn apps_iter_mut(
        &mut self,
    ) -> impl Iterator<Item = (&'static str, Anchor, &mut dyn eframe::App)> {
        let vec = vec![
            (
                "Pallette Editor",
                Anchor::PalletteEditor,
                &mut self.state.pallette_app as &mut dyn eframe::App,
            ),
            (
                "Color Editor",
                Anchor::ColorEditor,
                &mut self.state.color_app as &mut dyn eframe::App,
            ),
        ];

        vec.into_iter()
    }
}
#[derive(Clone, Copy, Debug)]
#[must_use]
enum Command {
    Nothing,
    ResetEverything,
}


impl eframe::App for WrapApp {
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.state);
    }

    fn clear_color(&self, visuals: &egui::Visuals) -> [f32; 4] {
        // Give the area behind the floating windows a different color, because it looks better:
        let color = egui::lerp(
            egui::Rgba::from(visuals.panel_fill)..=egui::Rgba::from(visuals.extreme_bg_color),
            0.5,
        );
        let color = egui::Color32::from(color);
        color.to_normalized_gamma_f32()
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        #[cfg(target_arch = "wasm32")]
        if let Some(anchor) = frame
            .info()
            .web_info
            .location
            .hash
            .strip_prefix('#')
            .and_then(Anchor::from_str_case_insensitive)
        {
            self.state.selected_anchor = anchor;
        }

        #[cfg(not(target_arch = "wasm32"))]
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::F11)) {
            let fullscreen = ctx.input(|i| i.viewport().fullscreen.unwrap_or(false));
            ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(!fullscreen));
        }

        let mut cmd = Command::Nothing;
        egui::TopBottomPanel::top("wrap_app_top_bar")
            .frame(egui::Frame::new().inner_margin(4))
            .show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.visuals_mut().button_frame = false;
                    self.bar_contents(ui, frame, &mut cmd);
                });
            });

        self.state.backend_panel.update(ctx, frame);

        cmd = self.backend_panel(ctx, frame);

        self.show_selected_app(ctx, frame);

        self.state.backend_panel.end_of_frame(ctx);

        // self.ui_file_drag_and_drop(ctx);

        self.run_cmd(ctx, cmd);
    }

}

impl WrapApp {
    fn backend_panel(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) -> Command {
        // The backend-panel can be toggled on/off.
        // We show a little animation when the user switches it.
        let is_open =
            self.state.backend_panel.open || ctx.memory(|mem| mem.everything_is_visible());

        let mut cmd = Command::Nothing;

        egui::SidePanel::left("backend_panel")
            .resizable(false)
            .show_animated(ctx, is_open, |ui| {
                ui.add_space(4.0);
                ui.vertical_centered(|ui| {
                    ui.heading("💻 Backend");
                });

                ui.separator();
                self.backend_panel_contents(ui, frame, &mut cmd);
            });

        cmd
    }

    fn run_cmd(&mut self, ctx: &egui::Context, cmd: Command) {
        match cmd {
            Command::Nothing => {}
            Command::ResetEverything => {
                self.state = Default::default();
                ctx.memory_mut(|mem| *mem = Default::default());
            }
        }
    }

    fn backend_panel_contents(
        &mut self,
        ui: &mut egui::Ui,
        frame: &mut eframe::Frame,
        cmd: &mut Command,
    ) {
        self.state.backend_panel.ui(ui, frame);

        ui.separator();

        ui.horizontal(|ui| {
            if ui
                .button("Reset egui")
                .on_hover_text("Forget scroll, positions, sizes etc")
                .clicked()
            {
                ui.ctx().memory_mut(|mem| *mem = Default::default());
                ui.close();
            }

            if ui.button("Reset everything").clicked() {
                *cmd = Command::ResetEverything;
                ui.close();
            }
        });
    }

    fn show_selected_app(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let selected_anchor = self.state.selected_anchor;
        for (_name, anchor, app) in self.apps_iter_mut() {
            if anchor == selected_anchor || ctx.memory(|mem| mem.everything_is_visible()) {
                app.update(ctx, frame);
            }
        }
    }

    fn bar_contents(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame, cmd: &mut Command) {
        egui::widgets::global_theme_preference_switch(ui);

        ui.separator();

        // if is_mobile(ui.ctx()) {
        //     ui.menu_button("💻 Backend", |ui| {
        //         ui.set_style(ui.ctx().style()); // ignore the "menu" style set by `menu_button`.
        //         self.backend_panel_contents(ui, frame, cmd);
        //     });
        // } else {
            ui.toggle_value(&mut self.state.backend_panel.open, "💻 Backend");
        // }

        ui.separator();

        let mut selected_anchor = self.state.selected_anchor;
        for (name, anchor, _app) in self.apps_iter_mut() {
            if ui
                .selectable_label(selected_anchor == anchor, name)
                .clicked()
            {
                selected_anchor = anchor;
                // if frame.is_web() {
                //     ui.ctx()
                //         .open_url(egui::OpenUrl::same_tab(format!("#{anchor}")));
                // }
            }
        }
        self.state.selected_anchor = selected_anchor;

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            egui::warn_if_debug_build(ui);
        });
    }

    // fn ui_file_drag_and_drop(&mut self, ctx: &egui::Context) {
    //     use egui::{Align2, Color32, Id, LayerId, Order, TextStyle};
    //     use std::fmt::Write as _;

    //     // Preview hovering files:
    //     if !ctx.input(|i| i.raw.hovered_files.is_empty()) {
    //         let text = ctx.input(|i| {
    //             let mut text = "Dropping files:\n".to_owned();
    //             for file in &i.raw.hovered_files {
    //                 if let Some(path) = &file.path {
    //                     write!(text, "\n{}", path.display()).ok();
    //                 } else if !file.mime.is_empty() {
    //                     write!(text, "\n{}", file.mime).ok();
    //                 } else {
    //                     text += "\n???";
    //                 }
    //             }
    //             text
    //         });

    //         let painter =
    //             ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

    //         let content_rect = ctx.content_rect();
    //         painter.rect_filled(content_rect, 0.0, Color32::from_black_alpha(192));
    //         painter.text(
    //             content_rect.center(),
    //             Align2::CENTER_CENTER,
    //             text,
    //             TextStyle::Heading.resolve(&ctx.style()),
    //             Color32::WHITE,
    //         );
    //     }
    // }
}




