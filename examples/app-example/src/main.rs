use eframe::*;

#[egui_macro::app(title = "My App", size = [1280.0, 720.0], min_size = [640.0, 360.0])]
struct MyApp {
    name: String,
    textedit: String,
}

impl App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.label(self.name.clone());
            ui.text_edit_singleline(&mut self.textedit);
            if ui.button("Confirm").clicked() {
                self.name = self.textedit.clone();
            }
        });
    }
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp {
            name: "User".to_string(),
            textedit: "User".to_string(),
        }
    }
}
