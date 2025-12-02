#![feature(adt_const_params, random)]

use bevy::prelude::*;
use bevy::app::ScheduleRunnerPlugin;
use bevy_ratatui::{RatatuiContext, RatatuiPlugins};

#[cfg(test)]
pub use tracing_test;


fn main() {
    // let mut parameters = Parameters::default();
    let frame_time = std::time::Duration::from_secs_f32(1. / 60.);

    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(frame_time)))
        .add_plugins(RatatuiPlugins::default())
        .add_systems(Update, draw_system)
        .run();
}

fn draw_system(mut context: ResMut<RatatuiContext>) -> Result {
    context.draw(|frame| {
        let text = ratatui::text::Text::raw("hello world");
        frame.render_widget(text, frame.area());
    })?;

    Ok(())
}



// #[tokio::main]
// async fn main() {
//     tracing::info!("Welcome to Truncheon.");
//     // let options = Options::parse();

//     let mut parameters = Parameters::default();

//     // do this up here, pass it down
//     let mut picker = Picker::from_query_stdio().unwrap_or(Picker::from_fontsize((8,12)));

//     parameters.protocol_type = picker.protocol_type();
//     parameters.font_size = picker.font_size();

//     // let renderer_handle = renderer::run(&parameters).await;
//     let _ui_handle = ui::run(&parameters).await;

//     // tokio::join!(renderer_handle, ui_handle)
// }


// // alternate
// //
// fn alt_main() -> Result<(), Box<dyn Error>> {
//     let mut parameters = Parameters::default();

//     let background_rt = tokio::runtime::Builder::new_multi_thread()
//         .worker_threads(2)
//         .thread_name("background_pool")
//         .enable_all()
//         .build()?;

//     let foreground_rt = tokio::runtime::Builder::new_multi_thread()
//         .worker_threads(8)
//         .thread_name("foreground_pool")
//         .enable_all()
//         .build()?;

//     foreground_rt.spawn(ui::run(&parameters))?
// //    background_rt.spawn(renderer::run(&parameters));
// }
