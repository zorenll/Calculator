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

            for ch in self.expression.chars() {
                if ch.is_whitespace() {
                    continue;
                }

                if ch.is_ascii_digit() || ch == '.' {
                    digits[index].push(ch);
                } else {
                    if !matches!(ch, '+' | '-' | '*' | '/' | '^') {
                        self.error = true;
                        return;
                    }

                    digits.push(String::new());
                    operations.push(ch);
                    index += 1;
                }
            }

            let mut nums = Vec::new();

            for digit in digits {
                match digit.parse::<f32>() {
                    Ok(number) => nums.push(Some(number)),
                    Err(_) => {
                        self.error = true;
                        return;
                    }
                }
            }

            // Exponents (right to left)
            for (i, op) in operations.iter().enumerate().rev() {
                if *op != '^' {
                    continue;
                }

                let ai = (0..=i).rev().find(|&j| nums[j].is_some());
                let bi = (i + 1..nums.len()).find(|&j| nums[j].is_some());

                let (Some(ai), Some(bi)) = (ai, bi) else {
                    self.error = true;
                    return;
                };

                let a = nums[ai].unwrap();
                let b = nums[bi].unwrap();

                nums[ai] = Some(a.powf(b));
                nums[bi] = None;
            }

            // Multiplication and division
            for (i, op) in operations.iter().enumerate() {
                if *op != '*' && *op != '/' {
                    continue;
                }

                let ai = (0..=i).rev().find(|&j| nums[j].is_some());
                let bi = (i + 1..nums.len()).find(|&j| nums[j].is_some());

                let (Some(ai), Some(bi)) = (ai, bi) else {
                    self.error = true;
                    return;
                };

                let a = nums[ai].unwrap();
                let b = nums[bi].unwrap();

                if *op == '/' && b == 0.0 {
                    self.error = true;
                    return;
                }

                if *op == '*' {
                    nums[ai] = Some(a * b);
                } else {
                    nums[ai] = Some(a / b);
                }

                nums[bi] = None;
            }

            // Addition and subtraction
            for (i, op) in operations.iter().enumerate() {
                if *op != '+' && *op != '-' {
                    continue;
                }

                let ai = (0..=i).rev().find(|&j| nums[j].is_some());
                let bi = (i + 1..nums.len()).find(|&j| nums[j].is_some());

                let (Some(ai), Some(bi)) = (ai, bi) else {
                    self.error = true;
                    return;
                };

                let a = nums[ai].unwrap();
                let b = nums[bi].unwrap();

                if *op == '+' {
                    nums[ai] = Some(a + b);
                } else {
                    nums[ai] = Some(a - b);
                }

                nums[bi] = None;
            }

            if let Some(result) = nums.into_iter().flatten().next() {
                if result.is_finite() {
                    self.result = result.to_string();
                } else {
                    self.error = true;
                }
            } else {
                self.error = true;
            }
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
