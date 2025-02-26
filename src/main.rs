use eframe::egui;
use serde::{Deserialize, Serialize};
use std::fs;

const FILE_PATH: &str = "todo_list.json";

fn main() {
    let options = eframe::NativeOptions::default();
    if let Err(e) = eframe::run_native(
        "TODO List",
        options,
        Box::new(|_cc| Box::new(TodoApp::load_todos())),
    ) {
        eprintln!("Error: {}", e);
    }
}

#[derive(Serialize, Deserialize)]
struct TodoApp {
    todos: Vec<String>,
    new_todo: String,
}

impl Default for TodoApp {
    fn default() -> Self {
        Self {
            todos: vec![],
            new_todo: Default::default(),
        }
    }
}

impl TodoApp {
    fn load_todos() -> Self {
        if let Ok(data) = fs::read_to_string(FILE_PATH) {
            if let Ok(app) = serde_json::from_str(&data) {
                return app;
            }
        }
        Self::default()
    }

    fn save_todos(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(FILE_PATH, json);
        }
    }
}

impl eframe::App for TodoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("TODO List");

            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.new_todo);
                if ui.button("Add").clicked() {
                    if !self.new_todo.is_empty() {
                        self.todos.push(self.new_todo.clone());
                        self.new_todo.clear();
                        self.save_todos();
                    }
                }
            });

            ui.separator();

            let mut remove_indices = vec![];
            for (i, todo) in self.todos.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(todo);
                    if ui.button("❌").clicked() {
                        remove_indices.push(i);
                    }
                });
            }

            if !remove_indices.is_empty() {
                for i in remove_indices.iter().rev() {
                    self.todos.remove(*i);
                }
                self.save_todos();
            }
        });
    }
}
