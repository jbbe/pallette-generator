#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#[macro_use]
extern crate lazy_static;
use std::{collections::HashMap, sync::Mutex};

use eframe::egui;

mod apps;
mod core;
mod debug;
mod widgets;
use apps::WrapApp;

use crate::core::{color::Rgb, color_names::ColorNames};

lazy_static! {
    static ref COLOR_DICT: Mutex<HashMap<Rgb<u8>, String>> = Mutex::new(HashMap::new());
}

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    init_color_dict();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 1024.0])
            .with_drag_and_drop(true),
        ..Default::default()
    };

    let result = eframe::run_native(
        "Color App",
        options,
        Box::new(|cc| Ok(Box::new(WrapApp::new(cc)))),
    );

    match result {
        Ok(()) => Ok(()),
        Err(err) => {
            // This produces a nicer error message than returning the `Result`:
            print_error_and_exit(&err);
        }
    }
}

fn init_color_dict() {
    let colors_res = ColorNames::load();

    match colors_res {
        Ok(map) => {
            let mut dict = COLOR_DICT.lock().unwrap();
            *dict = map;
        }
        Err(e) => println!("Error loading colors dictionary {}", e),
    }
}

fn print_error_and_exit(err: &eframe::Error) -> ! {
    #![expect(clippy::print_stderr)]
    #![expect(clippy::exit)]

    eprintln!("Error: {err}");
    std::process::exit(1)
}
