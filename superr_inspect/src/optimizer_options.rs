use eframe::egui::{Response, Ui};

pub struct OptimizerOptions {
    pub max_instructions: usize,
    pub max_number: usize,
}

impl OptimizerOptions {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            // Max Instructions
            ui.label("Max Instructions");
            integer_field(ui, &mut self.max_instructions);

            // Max Number
            ui.label("Max Number");
            integer_field(ui, &mut self.max_number);
        });
    }
}

// https://github.com/emilk/egui/issues/1348#issuecomment-1652168882
fn integer_field(ui: &mut Ui, value: &mut usize) -> Response {
    let mut tmp_value = format!("{}", value);
    let res = ui.text_edit_singleline(&mut tmp_value);
    if let Ok(result) = tmp_value.parse() {
        *value = result;
    }
    res
}
