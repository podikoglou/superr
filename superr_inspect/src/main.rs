pub mod memory_viewer;
pub mod optimizer_options;
pub mod syntax;

use eframe::egui;
use egui_code_editor::{CodeEditor, ColorTheme};
use memory_viewer::MemoryViewer;
use optimizer_options::OptimizerOptions;
use superr_vm::{instruction::Instruction, program::Program, vm::VM};

static DEFAULT_PROGRAM: &str = "
LOAD 3
SWAP 0 1
LOAD 3
SWAP 0 2
LOAD 3
SWAP 0 3
LOAD 3";

struct SuperrInspect {
    vm: VM,

    code_buffer: String,

    editor: CodeEditor,
    memory_viewer: MemoryViewer,
    optimizer_options: OptimizerOptions,
}
impl SuperrInspect {
    fn execute_program(&mut self) {
        let instructions = self
            .code_buffer
            .lines()
            .into_iter()
            .filter(|line| !line.is_empty())
            .map(|line| Instruction::from(line.to_string()))
            .collect::<Vec<Instruction>>();

        let program = Program { instructions };

        self.vm.execute_program(program);
    }
}

impl Default for SuperrInspect {
    fn default() -> Self {
        let vm = VM::default();

        Self {
            vm,
            code_buffer: DEFAULT_PROGRAM.to_string(),
            memory_viewer: MemoryViewer::default(),
            editor: CodeEditor::default()
                .id_source("code editor")
                .with_rows(12)
                .with_fontsize(14.0)
                .with_theme(ColorTheme::GRUVBOX)
                .with_syntax(syntax::superr())
                .with_numlines(true),
            optimizer_options: OptimizerOptions {
                max_instructions: 12,
                max_number: 8,
            },
        }
    }
}

impl eframe::App for SuperrInspect {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                // Code Editor
                self.editor.show(ui, &mut self.code_buffer);

                ui.separator();

                // Controls
                ui.horizontal(|ui| {
                    // Run Button
                    let run_button = ui.button("Run");

                    if run_button.clicked() {
                        self.execute_program();
                    }

                    // Optimize Button
                    let optimize_button = ui.button("Optimize");

                    if optimize_button.clicked() {
                        // self.execute_program();
                    }
                });

                ui.separator();

                // VM Info
                ui.horizontal(|ui| {
                    self.memory_viewer.ui(ui, &self.vm.state);

                    ui.separator();

                    self.optimizer_options.ui(ui);
                });
            });
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Superr Inspect",
        options,
        Box::new(|_| Ok(Box::<SuperrInspect>::default())),
    )
}
