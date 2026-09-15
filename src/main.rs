use eframe::egui;

#[derive(Default)]
struct App {
    expression: String,
    result: String,
    error: bool,
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("Calculator");

        ui.label("expression:");
        ui.text_edit_singleline(&mut self.expression);

        if ui.button("CALCULATE").clicked() {
            self.error = false;
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

            let mut output = match digits[0].parse::<f32>() {
                Ok(number) => number,
                Err(_) => 0.0,
            };
            for (i, op) in operations.iter().enumerate() {
                let a = output;
                let b = match digits[i + 1].parse() {
                    Ok(number) => number,
                    Err(_) => {
                        self.error = true;
                        0.0
                    }
                };

                match op {
                    '+' => output = a + b,
                    '-' => output = a - b,
                    '*' => output = a * b,
                    '/' => output = a / b,
                    '^' => {
                        output = {
                            let mut temp = a;
                            for _ in 0..b as usize - 1 {
                                temp *= a;
                            }
                            temp
                        }
                    }
                    _ => {
                        self.error = true;
                        continue;
                    }
                }
            }
            self.result = output.to_string();
        }

        if self.error {
            ui.label(
                egui::RichText::new("Error!")
                    .color(egui::Color32::RED)
                    .size(30.0),
            );
        } else {
            ui.label(&self.result);
        }
    }
}

fn main() -> eframe::Result {
    let option = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder {
            title: Some("Calculator".to_string()),
            transparent: Some(true),
            resizable: Some(true),
            min_inner_size: Some(egui::vec2(150.0, 202.0)),
            inner_size: Some(egui::vec2(200.0, 270.0)),
            ..eframe::egui::ViewportBuilder::default()
        },
        ..Default::default()
    };
    eframe::run_native("jrss", option, Box::new(|_cc| Ok(Box::<App>::default())))
}
