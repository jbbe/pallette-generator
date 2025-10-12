#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// it's an example
use eframe::egui;

mod core;
mod apps;
mod debug;
use apps::WrapApp;

use crate::debug::backend_panel;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    // let options = eframe::NativeOptions {
    //     viewport: egui::ViewportBuilder::default()
    //         .with_inner_size([1240.0, 840.0]) // wide enough for the drag-drop overlay text
    //         .with_drag_and_drop(true),
    //     ..Default::default()
    // };
    // eframe::run_native(
    //     "Pallette Generator",
    //     options,
    //     Box::new(|cc| {
    //         egui_extras::install_image_loaders(&cc.egui_ctx);
    //         Ok(Box::<PalletteApp>::default())
    //     }),
    // )
        let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 1024.0])
            .with_drag_and_drop(true),

        #[cfg(feature = "wgpu")]
        renderer: eframe::Renderer::Wgpu,

        ..Default::default()
    };

    let result = eframe::run_native(
        "Color App",
        options,
        Box::new(|cc| Ok(Box::new(WrapApp::new(cc)))),
    );

    match result {
        Ok(()) => {
            Ok(())
        }
        Err(err) => {
            // This produces a nicer error message than returning the `Result`:
            print_error_and_exit(&err);
        }
    }
}

fn print_error_and_exit(err: &eframe::Error) -> ! {
    #![expect(clippy::print_stderr)]
    #![expect(clippy::exit)]

    eprintln!("Error: {err}");
    std::process::exit(1)
}
