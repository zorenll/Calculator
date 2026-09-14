use eframe::egui;

#[derive(Default)]
struct App {
    expression: String,
    result: String,
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("CALCULATE");

        ui.label("expression:");
        ui.text_edit_singleline(&mut self.expression);

        if ui.button("Evaluate").clicked() {
            self.result = format!("You put in: {}", self.expression);
        }

        ui.label(&self.result);
    }
}

fn main() -> eframe::Result {
    let option = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder {
            title: Some("Calculator".to_string()),
            ..eframe::egui::ViewportBuilder::default()
        },
        ..Default::default()
    };
    eframe::run_native("jrss", option, Box::new(|_cc| Ok(Box::<App>::default())))
}
