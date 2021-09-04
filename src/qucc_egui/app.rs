use crate::qucc::QuccBMS;
use crate::qucc_egui::bms_cache::BMS;
use eframe::egui::Label;
use eframe::{egui, epi};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[cfg_attr(feature = "persistence", serde(skip))]
    value: f32,

    bms: BMS,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            bms: BMS::new(QuccBMS::new("/dev/tty.usbserial-110", 8)),
        }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "QUCC BMS"
    }

    /// Called by the framework to load old app state (if any).
    #[cfg(feature = "persistence")]
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        storage: Option<&dyn epi::Storage>,
    ) {
        if let Some(storage) = storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self { label, value, bms } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            // we want continuous UI updates, so the circle can smoothly follow the arrow's origin:
            ui.ctx().request_repaint();

            // ui.heading("Side Panel");

            // ui.horizontal(|ui| {
            //     ui.label("Write something: ");
            //     ui.text_edit_singleline(label);
            // });
            //
            // ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
            // if ui.button("Increment").clicked() {
            //     *value += 1.0;
            // }

            ui.heading("Cell voltages:");
            ui.vertical(|ui| {
                if let Ok(v) = bms.get_cell_v() {
                    let max = v.iter().max().unwrap_or(&0).clone();
                    let min = v.iter().min().unwrap_or(&0).clone();
                    let output: Vec<(u16, String)> = v
                        .into_iter()
                        .enumerate()
                        .map(|(i, v)| (v, format!("Cell{:02}: {}mV", i + 1, v)))
                        .collect();
                    output.iter().for_each(|(v, s)| {
                        if max - min <= 5 {
                            ui.label(s);
                            return;
                        }
                        if &max == v {
                            ui.label(Label::new(s).text_color(egui::Color32::RED));
                        } else if &min == v {
                            ui.label(Label::new(s).text_color(egui::Color32::BLUE));
                        } else {
                            ui.label(s);
                        }
                    });
                }
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add(
                    egui::Hyperlink::new("https://github.com/emilk/egui/").text("powered by egui"),
                );
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("QUCC BMS");
            ui.label(format!(
                "Device connected to: {}",
                bms.get_bms().get_device()
            ));
            ui.hyperlink_to("Author: Fuyang Liu", "https://liufuyang.github.io/cv/");
            ui.add(egui::github_link_file!(
                "https://github.com/liufuyang/usb-1/blob/master/",
                "Source code."
            ));
            egui::warn_if_debug_build(ui);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}
