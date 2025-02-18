use egui_code_editor::Syntax;
use std::collections::BTreeSet;

pub fn superr() -> Syntax {
    Syntax {
        language: "superr",
        case_sensitive: true,
        comment: ";",
        comment_multiline: ["/*", "*/"],
        hyperlinks: BTreeSet::from(["http"]),
        keywords: BTreeSet::from([
            "LOAD", "SWAP", "XOR", "INC", "DECR", "ADD", "SUB", "PUT", "JMP",
        ]),
        types: BTreeSet::from([]),
        special: BTreeSet::from([]),
    }
}
