use color_eyre::Result;
use y2024::ui::app_ui::AppUI;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = AppUI::default().run(terminal);
    ratatui::restore();
    app_result
}

