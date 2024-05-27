#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::io::Write;

use eframe::egui::{self, Color32, ComboBox, Id};
use reqwest::Error;
use serde::Serialize;
use types::GroupData;
mod types;
use egui::ViewportCommand;
use std::fs::{self, File};
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
        Box::new(|c| {
            egui_extras::install_image_loaders(&c.egui_ctx);

            Box::<JsonApp>::default()
        }),
    )
}

#[derive(Debug, Clone)]
struct JsonApp {
    group_data: Arc<Mutex<HashMap<String, GroupData>>>,
    data_fetched: bool,
    selected_group: String,
}

impl Default for JsonApp {
    fn default() -> Self {
        Self {
            group_data: Arc::new(Mutex::new(HashMap::new())),
            data_fetched: false,
            selected_group: String::new(),
        }
    }
}

impl JsonApp {
    fn add_group(&mut self, name: String, data: GroupData) {
        let mut data_locked = self
            .group_data
            .lock()
            .expect("failed to get a lock on group data - member function add_group()");
        data_locked.insert(name, data);
        drop(data_locked);
    }

    fn remove_group(&mut self, name: String) {
        let mut data_locked = self
            .group_data
            .lock()
            .expect("failed to get a lock on group data - member function remove_group()");
        data_locked.remove(&name);
        drop(data_locked);
    }
    fn edit_group(&mut self, name: String, data: GroupData) {}

    fn get_group_data(&self, name: String) -> Result<GroupData, String> {
        let mut data_locked = self
            .group_data
            .lock()
            .expect("failed to get a lock on group data - member function get_group_data()");
        match data_locked.get(&name) {
            Some(data) => Ok(data.clone()),
            None => Err(format!("No group with provided name found")),
        }
    }

    fn set_group_data(&mut self, data: HashMap<String, GroupData>) {
        let mut data_locked = self
            .group_data
            .lock()
            .expect("failed to get a lock on group data - member function set_group_data()");
        *data_locked = data;
        drop(data_locked);
    }
}

impl eframe::App for JsonApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();
        style.visuals.window_fill = Color32::from_rgb(0, 0, 0);

        custom_window_frame(ctx, "Infinity Manager", |ui| {
            let group_data = self.group_data.clone();
            ui.heading("Infinity Groups Manager");
            let group_data_clone = group_data.clone();
            if ui.button("Fetch group data from repo").clicked() {
                let mut callback =
                    move |result: Result<HashMap<String, GroupData>, String>| match result {
                        Ok(data) => {
                            let mut data_locked = group_data.lock().unwrap();
                            *data_locked = data;
                            drop(data_locked);
                        }
                        Err(e) => {
                            eprintln!("error fetching: {}", e)
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
            let locked_data = self.group_data.lock().unwrap();
            if !locked_data.is_empty() {
                if ui.button("Output group.json file").clicked() {
                    write_locked_data(locked_data.clone())
                }
                let mut selected_item = self.selected_group.clone();
                let mut selected_item_string = String::new();
                ComboBox::from_id_source(Id::new("Groups"))
                    .selected_text(format!("Select Group"))
                    .show_ui(ui, |ui| {
                        for (name, _) in locked_data.iter() {
                            ui.selectable_value(&mut selected_item, name.clone(), name);
                        }
                    });
                self.selected_group = selected_item;
                match locked_data.get(&self.selected_group.clone()) {
                    Some(data) => {
                        ui.label(format!("{:?}", data));
                    }
                    None => (),
                }
            }
            drop(locked_data);
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

/// Show some close/maximize/minimize buttons for the native window.
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
