use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();
    if let Err(e) = eframe::run_native(
        "TODO List",
        options,
        Box::new(|_cc| Box::new(TodoApp::default())),
    ) {
        eprintln!("Error: {}", e);
    }
}

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
            for i in remove_indices.iter().rev() {
                self.todos.remove(*i);
            }
        });
    }
    
}