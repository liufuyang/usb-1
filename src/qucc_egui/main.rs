#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

use eframe::egui::Vec2;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = usb_1::qucc_egui::TemplateApp::default();
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(400f32, 800f32));
    eframe::run_native(Box::new(app), native_options);
}
