#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

const VERSION: &str = "1.5.0";

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size([500.0, 400.0])
            .with_icon(eframe::icon_data::from_png_bytes(&include_bytes!("../icon.png")[..])
            .unwrap()),
        ..Default::default()
    };
    eframe::run_native(
        &("Blackjack Solver ".to_owned() + VERSION),
        native_options,
        Box::new(|cc| Box::new(blackjack_calculator::AppMain::new(cc))),
    ).unwrap();
}
