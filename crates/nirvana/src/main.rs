mod cli;
mod tui;

fn main() {
    let result = if std::env::args().len() == 1 {
        crate::tui::run()
    } else {
        crate::cli::run()
    };

    if let Err(e) = result {
        eprintln!(
            "{}: {e:#}",
            console::Style::new().bold().red().apply_to("error")
        );
        std::process::exit(1);
    }
}
