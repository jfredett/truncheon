use std::sync::Arc;
use tokio::sync::RwLock;
use std::error::Error;

mod app;
mod widgets;
mod tui;

use app::UI;

use crate::ui::tui::Tui;


pub async fn run() -> Result<(), Box<dyn Error>> {
    let app = Arc::new(RwLock::new(UI::new()));
    let mut tui = Tui::new(app.clone(), 10.0, 2.0).unwrap();
    tui.run().await?;

    Ok(())
}
