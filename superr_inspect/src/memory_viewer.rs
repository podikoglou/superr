use eframe::egui::{Grid, ScrollArea, Ui};
use superr_vm::vm::State;

#[derive(Default)]
pub struct MemoryViewer();

impl MemoryViewer {
    pub fn ui(&mut self, ui: &mut Ui, memory: &State) {
        Grid::new("memory_grid")
            .striped(true)
            .spacing([40.0, 4.0])
            .show(ui, |ui| {
                // Header
                ui.label("Address");
                ui.label("Value (Dec)");
                ui.label("Value (Hex)");
                ui.end_row();

                // Memory Contents
                for (addr, value) in memory.iter().enumerate() {
                    ui.label(format!("{}", addr));
                    ui.label(format!("{}", value));
                    ui.label(format!("{:#04x}", value));
                    ui.end_row();
                }
            });
    }
}
