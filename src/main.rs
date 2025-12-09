#![feature(adt_const_params, random)]


use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::app::ScheduleRunnerPlugin;
use bevy::winit::WinitPlugin;
use bevy_ratatui::{RatatuiContext, RatatuiPlugins};
use dioxus::prelude::dioxus_devtools;
use bevy_ratatui_camera::{RatatuiCamera, RatatuiCameraPlugin, RatatuiCameraWidget};
use ratatui::widgets::Block;
use ratatui::prelude::*;

use ratatui_image::picker::Picker;
use ratatui_image::Resize;
use ratatui_image::StatefulImage;
#[cfg(test)]
pub use tracing_test;

use tui_logger::{LevelFilter, TuiLoggerWidget};
use tracing_subscriber::{prelude::*, registry::Registry};
use tui_logger::TuiTracingSubscriberLayer;

fn main() {
    dioxus_devtools::connect_subsecond();


    // set up TUI Logger
    tui_logger::init_logger(LevelFilter::Trace).unwrap();
    tui_logger::set_default_level(LevelFilter::Trace);

    let subscriber = Registry::default().with(tui_logger::TuiTracingSubscriberLayer);
    bevy::log::tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to install tracing subscriber");


    // let mut parameters = Parameters::default();
    let frame_time = std::time::Duration::from_secs_f32(1. / 60.);

    App::new()
        .add_plugins(
            DefaultPlugins.build()
                .disable::<WinitPlugin>()
                .disable::<LogPlugin>()
        )
        .add_plugins(bevy_mod_debugdump::CommandLineArgs)
        .add_plugins(ScheduleRunnerPlugin::run_loop(frame_time))
        .add_plugins(RatatuiPlugins::default())
        .add_plugins(RatatuiCameraPlugin)
        // .insert_resource(ClearColor(bevy::prelude::Color::BLACK))
        // .insert_resource(UI::default())
        .add_systems(Startup, (init_picker, init_scene, init_ratatui_camera))
        .add_systems(PreUpdate, refresh_picker)
        .add_systems(Update, draw_system)
        .run();
}

// #[derive(Default, Resource)]
// struct UI(truncheon::ui::UI);
//

fn init_ratatui_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        RatatuiCamera::default(),
    ));
}


#[derive(Deref, Resource)]
struct PickerResource(Picker);

fn init_picker(mut commands: Commands) {
    trace!("Here!");
    // need to do this before starting the render?
    let mut picker = Picker::from_query_stdio().unwrap();
//    picker.set_protocol_type(ratatui_image::picker::ProtocolType::Kitty);
    info!("picker: {:?}", picker.protocol_type());
    commands.insert_resource(PickerResource(picker));
}

fn init_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut asset_server: Res<AssetServer>,
) {
    let image = asset_server.load("hex/hex1.png");

    commands.spawn(Sprite {
        image,
        ..default()
    });
}

fn refresh_picker(
    mut commands: Commands,
    mut picker: ResMut<PickerResource>
) {
    info!("Picker before update: {:?}", picker.font_size());
    commands.insert_resource(PickerResource(Picker::from_query_stdio().unwrap()));
}

fn draw_system(
    mut context: ResMut<RatatuiContext>,
    camera_widget: Single<&mut RatatuiCameraWidget>,
    picker: Res<PickerResource>
) -> Result {
    context.draw(|frame| {
        let layout = Layout::new(
            Direction::Vertical,
            [Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Fill(1)],
        ).split(frame.area());

        let reference_widget = StatefulImage::default().resize(Resize::Crop(None));
        let ratatui_image_widget = StatefulImage::default().resize(Resize::Crop(None));

        let mut camera_image = picker.new_resize_protocol(camera_widget.camera_image.clone());

        let ref_image = image::ImageReader::open("./assets/hex/hex1.png").unwrap().decode().unwrap();
        let mut ref_image = picker.new_resize_protocol(ref_image);

        reference_widget.render(layout[0], frame.buffer_mut(), &mut ref_image);
        ratatui_image_widget.render(layout[1], frame.buffer_mut(), &mut camera_image);
        frame.render_widget(
            TuiLoggerWidget::default()
                .block(Block::bordered())
                .style(Style::default().bg(ratatui::style::Color::Reset)),
            layout[2]);
    })?;

    Ok(())
}

