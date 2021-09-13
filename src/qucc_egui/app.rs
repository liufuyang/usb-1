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

            ui.heading("Basic info");
            ui.vertical(|ui| {
                if let Ok(info2) = bms.get_info2() {
                    if info2.soc > 30 {
                        ui.label(
                            Label::new(format!("SOC: {:0.1}% 🔋", info2.soc))
                                .heading()
                                .text_color(egui::Color32::GREEN),
                        );
                    } else if info2.soc > 15 {
                        ui.label(
                            Label::new(format!("SOC: {:0.1}% 🔋", info2.soc))
                                .text_color(egui::Color32::GOLD),
                        );
                    } else {
                        ui.label(
                            Label::new(format!("SOC: {:0.1}% 🔋", info2.soc))
                                .text_color(egui::Color32::RED),
                        );
                    }
                }

                if let Ok(info) = bms.get_info() {
                    let color;
                    let icon_current;
                    if info.current < -0.2 {
                        // discharge
                        color = egui::Color32::GOLD;
                        icon_current = "💡";
                    } else if info.current > -0.2 && info.current < 0.2 {
                        // idle
                        color = egui::Color32::GRAY;
                        icon_current = "";
                    } else {
                        // charge
                        color = egui::Color32::GREEN;
                        icon_current = "🔌";
                    }
                    ui.label(
                        Label::new(format!("Current: {:0.1}A {}", info.current, icon_current))
                            .heading()
                            .text_color(color),
                    );
                    ui.label(
                        Label::new(format!("Voltage: {:0.1}V", info.voltage))
                            .text_color(egui::Color32::LIGHT_GRAY),
                    );
                    ui.label(format!("Cell count: {}", info.cell_count));
                    ui.label(format!("Running time: {}h", info.running_time));
                    ui.label(format!("SOH: {}%", info.soh));

                    if let Ok(cell_v) = bms.get_cell_v() {
                        let max = cell_v.iter().max().unwrap_or(&0u16);
                        let min = cell_v.iter().min().unwrap_or(&0u16);
                        ui.label(format!("Max cell voltage diff: {}mv", max - min));
                    }
                    if let Ok(info2) = bms.get_info2() {
                        ui.label(format!("Max cell voltage number: {}", info2.v_max_mi_no.0,));
                        ui.label(format!("Min cell voltage number: {}", info2.v_max_mi_no.1));
                    }
                }
            });

            egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add(
                        egui::Hyperlink::new("https://github.com/emilk/egui/")
                            .text("powered by egui"),
                    );
                });
            });

            egui::TopBottomPanel::bottom("cell_panel").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.heading("Cell voltages:");
                        ui.vertical(|ui| match bms.get_cell_v() {
                            Ok(v) => {
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
                                        ui.label(Label::new(s).text_color(egui::Color32::GOLD));
                                    } else if &min == v {
                                        ui.label(
                                            Label::new(s).text_color(egui::Color32::LIGHT_BLUE),
                                        );
                                    } else {
                                        ui.label(s);
                                    }
                                });
                            }
                            Err(e) => {
                                println!("Error when getting cell v: {:?}", e);
                            }
                        });
                    });

                    ui.vertical(|ui| {
                        ui.heading("Temperatures:");
                        if let Ok(info) = bms.get_info() {
                            let output: Vec<String> = info
                                .temperature
                                .iter()
                                .enumerate()
                                .map(|(i, c)| format!("Temp{:02}: {:0.1}C", i + 1, c))
                                .collect();
                            output.iter().for_each(|s| {
                                ui.label(s);
                            });
                        }
                    });
                });
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
