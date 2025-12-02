use std::sync::Arc;
use tokio::sync::RwLock;
use std::error::Error;

mod app;
mod widgets;
mod tui;

use app::UI;

use crate::{ui::tui::Tui, util::options::Parameters};


pub async fn run(p: &Parameters) -> Result<(), Box<dyn Error>> {
    let app = Arc::new(RwLock::new(UI::new(p)));
    let mut tui = Tui::new(app.clone(), 1.0 / 10.0, 1.0).unwrap();
    tui.run(p).await?;

    Ok(())
}
