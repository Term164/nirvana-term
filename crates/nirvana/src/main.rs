mod cli;
mod tui;

fn main() -> anyhow::Result<()> {
    if std::env::args().len() == 1 {
        crate::tui::run()
    } else {
        crate::cli::run()
    }
}
