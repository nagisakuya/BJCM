#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

const VERSION:&str = "1.2.1";

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).

    let mut native_options = eframe::NativeOptions::default();
    native_options.min_window_size = Some(egui::emath::Vec2 { x: 500.0, y: 400.0 });
    eframe::run_native(
        &("Blackjack Counting Master ".to_owned() + VERSION),
        native_options,
        Box::new(|cc| Box::new(blackjack_calculator::AppMain::new(cc))),
    );
}