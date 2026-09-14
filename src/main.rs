use eframe::egui;

#[derive(Default)]
struct App {
    expression: String,
    result: String,
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("Calculator");

        ui.label("expression:");
        ui.text_edit_singleline(&mut self.expression);

        if ui.button("CALCULATE").clicked() {
            let mut digits = vec![String::new()];
            let mut operations = Vec::new();
            let mut index = 0;
            for char in self.expression.chars() {
                if !char.is_digit(10) {
                    digits.push(String::new());
                    index += 1;
                    operations.push(char);
                } else {
                    digits[index].push(char);
                }
            }
            let mut output = String::new();
            for (i, op) in operations.iter().enumerate() {
                output.push_str(&digits[i]);
                output.push(' ');
                output.push(*op);
                output.push(' ');
            }
            output.push_str(&digits[operations.len()]);
            self.result = output.clone();
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
