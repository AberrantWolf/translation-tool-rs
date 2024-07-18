use indexmap::IndexMap;
use rfd;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

type TextMapping = IndexMap<String, HashMap<String, String>>;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct DqxTextApp {
    // Example stuff:
    search_path: String,

    #[serde(skip)]
    json_text: String,

    #[serde(skip)]
    values: TextMapping,
}

impl Default for DqxTextApp {
    fn default() -> Self {
        Self {
            search_path: String::default(),
            json_text: String::default(),
            values: TextMapping::default(),
        }
    }
}

impl DqxTextApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        Self::setup_fonts(&cc.egui_ctx);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn setup_fonts(ctx: &egui::Context) {
        // Start with the default fonts (we will be adding to them rather than replacing them).
        let mut fonts = egui::FontDefinitions::default();

        // Install my own font (maybe supporting non-latin characters).
        // .ttf and .otf files supported.
        fonts.font_data.insert(
            "noto_sans_jp".to_owned(),
            egui::FontData::from_static(include_bytes!(
                "../assets/NotoSansJP-VariableFont_wght.ttf"
            )),
        );
        // Put my font first (highest priority) for proportional text:
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "noto_sans_jp".to_owned());

        // Put my font as last fallback for monospace:
        // fonts
        //     .families
        //     .entry(egui::FontFamily::Monospace)
        //     .or_default()
        //     .push("noto_sans_jp".to_owned());

        // Tell egui to use these fonts:
        ctx.set_fonts(fonts);
    }

    pub fn open_file(&mut self, path: &Path) {
        if let Some(parent) = path.parent() {
            self.search_path = parent
                .to_str()
                .expect("No string made from parent path")
                .to_owned();
            let mut file = match File::open(path) {
                Err(err) => {
                    println!("Error opening {:?}: {err}", path);
                    return;
                }
                Ok(file) => file,
            };

            self.json_text = String::new();
            match file.read_to_string(&mut self.json_text) {
                Err(err) => {
                    println!("Error reading file {:?}: {err}", path);
                    return;
                }
                Ok(_) => {}
            }

            match serde_json::from_str(&self.json_text) {
                Err(err) => {
                    println!("Error reading json: {err}");
                    return;
                }
                Ok(values) => self.values = values,
            }
        }
    }
}

impl eframe::App for DqxTextApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                ui.menu_button("File", |ui| {
                    // OPEN menu item ------------------------------------------
                    if ui.button("Open").clicked() {
                        println!("Open!");
                        if let Some(open_path) = rfd::FileDialog::new()
                            .add_filter("JSON Translation", &["json"])
                            .pick_file()
                        {
                            self.open_file(&open_path);
                        }
                    }

                    // SAVE menu item ------------------------------------------
                    if ui.button("Save").clicked() {
                        println!("Saving!");
                    }

                    // SAVE AS menu item ------------------------------------------
                    if ui.button("Save As...").clicked() {
                        println!("Save as...");
                    }

                    if !is_web {
                        ui.separator();
                        // QUIT menu item ------------------------------------------
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    }
                });
                ui.add_space(16.0);

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("eframe template");

            for (id, submap) in &mut self.values {
                ui.horizontal(|ui| {
                    ui.label(id);

                    submap.iter_mut().for_each(|(jp, en)| {
                        let mut jp_str = jp.as_str();
                        ui.text_edit_multiline(&mut jp_str);
                        ui.text_edit_multiline(en);
                    });
                });
            }

            ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/main/",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
