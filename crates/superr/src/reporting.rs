use core::fmt;

use ariadne::{sources, Color, Label, Report, ReportKind};
use chumsky::{error::Rich, span::SimpleSpan};

fn failure(
    msg: String,
    label: (String, SimpleSpan),
    extra_labels: impl IntoIterator<Item = (String, SimpleSpan)>,
    src: &str,
    file_name: String,
) -> ! {
    Report::build(ReportKind::Error, (file_name.clone(), label.1.into_range()))
        .with_config(ariadne::Config::new().with_index_type(ariadne::IndexType::Byte))
        .with_message(&msg)
        .with_label(
            Label::new((file_name.clone(), label.1.into_range()))
                .with_message(label.0)
                .with_color(Color::Red),
        )
        .with_labels(extra_labels.into_iter().map(|label2| {
            Label::new((file_name.clone(), label2.1.into_range()))
                .with_message(label2.0)
                .with_color(Color::Yellow)
        }))
        .finish()
        .print(sources([(file_name, src)]))
        .unwrap();

    std::process::exit(1)
}

pub fn parse_failure(err: &Rich<impl fmt::Display>, src: &str, file_name: String) -> ! {
    failure(
        err.reason().to_string(),
        (
            err.found()
                .map(|c| c.to_string())
                .unwrap_or_else(|| "end of input".to_string()),
            *err.span(),
        ),
        err.contexts()
            .map(|(l, s)| (format!("while parsing this {l}"), *s)),
        src,
        file_name,
    )
}
