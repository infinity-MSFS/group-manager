#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::{self, Color32, ComboBox, Id, Image, Label, Pos2, TextEdit, Vec2, Window};
use std::collections::HashMap;
use std::io::Write;
use types::{GroupData, Project};
mod types;
use egui::ViewportCommand;
use std::fs::File;
use std::sync::{Arc, Mutex};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 800.0])
            .with_decorations(false)
            .with_transparent(true),
        ..Default::default()
    };

    eframe::run_native(
        "Group Manager",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<JsonApp>::default()
        }),
    )
}

#[derive(Debug, Clone)]
struct JsonApp {
    group_data: Arc<Mutex<HashMap<String, GroupData>>>,
    selected_group: String,
    selected_project: Option<usize>,
    new_group_name: String,
    new_project_name: String,
}

impl Default for JsonApp {
    fn default() -> Self {
        Self {
            group_data: Arc::new(Mutex::new(HashMap::new())),
            selected_group: String::new(),
            selected_project: None,
            new_group_name: String::new(),
            new_project_name: String::new(),
        }
    }
}

impl eframe::App for JsonApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();
        style.visuals.window_fill = Color32::from_rgb(0, 0, 0);

        custom_window_frame(ctx, "Infinity Manager", |ui| {
            egui::ScrollArea::both()
                .drag_to_scroll(true)
                .animated(true)
                .enable_scrolling(true)
                .show(ui, |ui| {
                    let group_data = self.group_data.clone();

                    ui.heading("Infinity Groups Manager");

                    if ui.button("Fetch group data from repo").clicked() {
                        let callback = move |result: Result<HashMap<String, GroupData>, String>| {
                            match result {
                                Ok(data) => {
                                    let mut data_locked = group_data.lock().unwrap();
                                    *data_locked = data;
                                    drop(data_locked);
                                }
                                Err(e) => {
                                    eprintln!("error fetching: {}", e)
                                }
                            }
                        };

                        std::thread::spawn(move || {
                            let runtime = tokio::runtime::Runtime::new().unwrap();
                            runtime.block_on(async move {
                                let result = fetch_data().await;
                                callback(result);
                            })
                        });
                    }
                    let mut locked_data = self.group_data.lock().unwrap();
                    if !locked_data.is_empty() {
                        ui.horizontal(|ui| {
                            ui.label("New group name:");
                            ui.text_edit_singleline(&mut self.new_group_name);
                            if ui.button("Add Group").clicked() {
                                locked_data.insert(
                                    self.new_group_name.clone(),
                                    GroupData::new(self.new_group_name.clone()),
                                );
                                self.new_group_name.clear();
                            }
                        });

                        let mut selected_item = self.selected_group.clone();
                        ui.horizontal(|ui: &mut egui::Ui| {
                            if ui.button("Output group.json file").clicked() {
                                write_locked_data(locked_data.clone())
                            }

                            ComboBox::from_id_source(Id::new("Groups"))
                                .selected_text(format!("Select Group"))
                                .show_ui(ui, |ui| {
                                    for (name, _) in locked_data.iter() {
                                        ui.selectable_value(&mut selected_item, name.clone(), name);
                                    }
                                });
                        });

                        if self.selected_group != selected_item {
                            self.selected_group = selected_item.clone();
                            self.selected_project = None;
                        }

                        self.selected_group = selected_item;

                        if !self.selected_group.is_empty() {
                            let data = locked_data.get_mut(&self.selected_group).unwrap();
                            ui.heading(format!("Group: {}", data.name));
                            ui.separator();
                            ui.heading("Projects");

                            ui.label("New Project Name:");
                            ui.text_edit_singleline(&mut self.new_project_name);
                            if ui.button("Add Project").clicked() {
                                data.projects
                                    .push(Project::new(self.new_project_name.clone()));
                                self.new_project_name.clear();
                            }

                            let mut selected_project = self.selected_project.unwrap_or_default();
                            ComboBox::from_id_source(Id::new("Projects"))
                                .selected_text("Select Project")
                                .show_ui(ui, |ui| {
                                    for (index, project) in data.projects.iter().enumerate() {
                                        ui.selectable_value(
                                            &mut selected_project,
                                            index,
                                            project.name.clone(),
                                        );
                                    }
                                });
                            self.selected_project = Some(selected_project);

                            if let Some(index) = self.selected_project {
                                if !data.projects.is_empty() {
                                    if index < data.projects.len() {
                                        ui.separator();
                                        ui.heading(format!(
                                            "Project: {}",
                                            data.projects[index].name
                                        ));

                                        ui.horizontal(|ui| {
                                            ui.label("Version");
                                            ui.text_edit_singleline(
                                                &mut data.projects[index].version,
                                            );
                                        });
                                        ui.horizontal(|ui| {
                                            ui.label("Date");
                                            ui.text_edit_singleline(&mut data.projects[index].date);
                                        });
                                        ui.horizontal(|ui| {
                                            ui.label("Changelog");
                                            ui.text_edit_multiline(
                                                &mut data.projects[index].changelog,
                                            );
                                        });
                                        ui.horizontal(|ui| {
                                            ui.label("Overview");
                                            ui.text_edit_multiline(
                                                &mut data.projects[index].overview,
                                            );
                                        });
                                        ui.horizontal(|ui| {
                                            ui.label("Description");
                                            ui.text_edit_multiline(
                                                &mut data.projects[index].description,
                                            );
                                        });
                                        ui.horizontal(|ui| {
                                            ui.set_width(500.0);
                                            ui.label("Background");
                                            ui.text_edit_singleline(
                                                &mut data.projects[index].background,
                                            );
                                            ui.add(
                                                Image::new(
                                                    data.projects[index]
                                                        .background
                                                        .clone()
                                                        .replace("webp", "png"),
                                                )
                                                .max_width(400.0),
                                            );
                                        });

                                        if let Some(package) = data.projects[index].package.as_mut()
                                        {
                                            ui.heading("Package");
                                            if ui.button("Add Package").clicked() {}
                                            ui.separator();
                                            ui.horizontal(|ui| {
                                                ui.label("Owner");
                                                ui.text_edit_singleline(&mut package.owner);
                                            });
                                            ui.horizontal(|ui| {
                                                ui.label("Repo Name");
                                                ui.text_edit_singleline(&mut package.repoName);
                                            });
                                            ui.horizontal(|ui| {
                                                ui.label("Version");
                                                ui.text_edit_singleline(&mut package.version);
                                            });
                                            ui.horizontal(|ui| {
                                                ui.label("File Name");
                                                ui.text_edit_singleline(&mut package.fileName);
                                            });
                                        }
                                    }
                                }
                            }

                            ui.separator();
                            ui.heading("Beta");
                            ui.horizontal(|ui| {
                                ui.text_edit_multiline(&mut data.beta.background);
                                ui.add(
                                    Image::new(data.beta.background.clone().replace("webp", "png"))
                                        .max_width(100.0),
                                );
                            });

                            ui.separator();
                            ui.heading(format!("Logo"));
                            ui.horizontal(|ui| {
                                ui.text_edit_multiline(&mut data.logo);
                                ui.add(
                                    Image::new(data.logo.clone().replace("webp", "png"))
                                        .max_width(100.0),
                                );
                            });

                            ui.separator();
                            if let Some(mut data) = data.update.as_mut() {
                                ui.checkbox(&mut data, "Update");
                            }
                            ui.separator();
                            ui.horizontal(|ui| {
                                ui.label("Path");
                                ui.text_edit_singleline(&mut data.path);
                            });
                            ui.separator();
                            ui.heading("Palette");
                            ui.horizontal(|ui| {
                                ui.label(format!("Primary:"));
                                ui.text_edit_singleline(&mut data.palette.primary);
                                if !data.palette.primary.is_empty() {
                                    let rgb = hex_to_rgb(&data.palette.primary);
                                    egui::widgets::color_picker::show_color(
                                        ui,
                                        Color32::from_rgb(rgb.0, rgb.1, rgb.2),
                                        Vec2::new(50.0, 50.0),
                                    );
                                }
                            });
                            ui.horizontal(|ui| {
                                ui.label(format!("Secondary:"));
                                ui.text_edit_singleline(&mut data.palette.secondary);

                                if !data.palette.secondary.is_empty() {
                                    let rgb = hex_to_rgb(&data.palette.secondary);
                                    egui::widgets::color_picker::show_color(
                                        ui,
                                        Color32::from_rgb(rgb.0, rgb.1, rgb.2),
                                        Vec2::new(50.0, 50.0),
                                    );
                                }
                            });
                        }
                        drop(locked_data);
                    }
                });
        });
    }
}

async fn fetch_data() -> Result<HashMap<String, GroupData>, String> {
    let link = "https://raw.githubusercontent.com/infinity-MSFS/groups/main/groups.json";

    match reqwest::get(link).await {
        Ok(request) => match request.json::<HashMap<String, GroupData>>().await {
            Ok(data) => Ok(data),
            Err(e) => Err(format!("Error deserializing: {}", e)),
        },
        Err(e) => Err(format!("Error fetching: {}", e)),
    }
}

fn write_locked_data(data: HashMap<String, GroupData>) {
    let json_data = serde_json::to_vec_pretty(&data).unwrap();

    let mut file = File::create("groups.json").expect("failed to create file");

    file.write_all(&json_data).expect("write error");
}

fn custom_window_frame(ctx: &egui::Context, title: &str, add_contents: impl FnOnce(&mut egui::Ui)) {
    use egui::*;

    let panel_frame = egui::Frame {
        fill: Color32::from_rgb(0, 0, 0),
        rounding: 10.0.into(),
        stroke: ctx.style().visuals.widgets.noninteractive.fg_stroke,
        outer_margin: 0.5.into(), // so the stroke is within the bounds
        ..Default::default()
    };

    CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
        let app_rect = ui.max_rect();

        let title_bar_height = 32.0;
        let title_bar_rect = {
            let mut rect = app_rect;
            rect.max.y = rect.min.y + title_bar_height;
            rect
        };
        title_bar_ui(ui, title_bar_rect, title);

        // Add the contents:
        let content_rect = {
            let mut rect = app_rect;
            rect.min.y = title_bar_rect.max.y;
            rect
        }
        .shrink(4.0);
        let mut content_ui = ui.child_ui(content_rect, *ui.layout());
        add_contents(&mut content_ui);
    });
}

fn title_bar_ui(ui: &mut egui::Ui, title_bar_rect: eframe::epaint::Rect, title: &str) {
    use egui::*;

    let painter = ui.painter();

    let title_bar_response = ui.interact(title_bar_rect, Id::new("title_bar"), Sense::click());

    // Paint the title:
    painter.text(
        title_bar_rect.center(),
        Align2::CENTER_CENTER,
        title,
        FontId::proportional(20.0),
        ui.style().visuals.text_color(),
    );

    // Paint the line under the title:
    painter.line_segment(
        [
            title_bar_rect.left_bottom() + vec2(1.0, 0.0),
            title_bar_rect.right_bottom() + vec2(-1.0, 0.0),
        ],
        ui.visuals().widgets.noninteractive.bg_stroke,
    );

    if title_bar_response.double_clicked() {
        let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
        ui.ctx()
            .send_viewport_cmd(ViewportCommand::Maximized(!is_maximized));
    }

    if title_bar_response.is_pointer_button_down_on() {
        ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
    }

    ui.allocate_ui_at_rect(title_bar_rect, |ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.visuals_mut().button_frame = false;
            ui.add_space(8.0);
            close_maximize_minimize(ui);
        });
    });
}

fn close_maximize_minimize(ui: &mut egui::Ui) {
    use egui::{Button, RichText};

    let button_height = 24.0;

    let close_response = ui
        .add(Button::new(RichText::new("❌").size(button_height)))
        .on_hover_text("Close the window");
    if close_response.clicked() {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
    }

    let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
    if is_maximized {
        let maximized_response = ui
            .add(Button::new(RichText::new("🗗").size(button_height)))
            .on_hover_text("Restore window");
        if maximized_response.clicked() {
            ui.ctx()
                .send_viewport_cmd(ViewportCommand::Maximized(false));
        }
    } else {
        let maximized_response = ui
            .add(Button::new(RichText::new("🗗").size(button_height)))
            .on_hover_text("Maximize window");
        if maximized_response.clicked() {
            ui.ctx().send_viewport_cmd(ViewportCommand::Maximized(true));
        }
    }

    let minimized_response = ui
        .add(Button::new(RichText::new("🗕").size(button_height)))
        .on_hover_text("Minimize the window");
    if minimized_response.clicked() {
        ui.ctx().send_viewport_cmd(ViewportCommand::Minimized(true));
    }
}

fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
    let r = u8::from_str_radix(&hex[1..3], 16).ok().unwrap();
    let g = u8::from_str_radix(&hex[3..5], 16).ok().unwrap();
    let b = u8::from_str_radix(&hex[5..7], 16).ok().unwrap();

    (r, g, b)
}
