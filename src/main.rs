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

use truncheon::ui::widgets::io::IOWidget;

use ratatui_image::picker::Picker;
#[cfg(test)]
pub use tracing_test;

use tui_logger::{LevelFilter, TuiLoggerWidget};
use tracing_subscriber::{prelude::*, registry::Registry};

fn prep() {
    // set up dioxus
    dioxus_devtools::connect_subsecond();

    // set up TUI Logger, needs to be done before the app starts.
    tui_logger::init_logger(LevelFilter::Trace).unwrap();
    tui_logger::set_default_level(LevelFilter::Debug);

    let subscriber = Registry::default().with(tui_logger::TuiTracingSubscriberLayer);
    bevy::log::tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to install tracing subscriber");
}

fn main() {
    prep();

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
        .add_plugins(IOWidget::default())
        .add_systems(Startup, (init_picker, init_scene, init_ratatui_camera).chain())
        .add_systems(PreUpdate, refresh_picker)
        .add_systems(Update, (draw_system, zoom_in).chain())
        .run();
}


#[derive(Component)]
#[require(Camera2d, RatatuiCamera)]
struct MainCamera;

fn zoom_in(
    mut projection: Single<&mut Projection, With<MainCamera>>,
    time: Res<Time>
) {
    match projection.as_mut() {
        Projection::Perspective(perspective) => {
            perspective.fov -= time.delta_secs() * 1.0;
        },
        Projection::Orthographic(ortho) => {
            let mut log_scale = ortho.scale.ln();
            log_scale -= 0.5 * time.delta_secs();
            if log_scale < 0.001 {
                log_scale = 10.;
            }
            ortho.scale = log_scale.exp();
        },
        Projection::Custom(_) => {},
    }
}

fn init_ratatui_camera(mut commands: Commands) {
    commands.spawn(MainCamera);
}

#[derive(Deref, Resource)]
struct PickerResource(Picker);

fn init_picker(mut commands: Commands) {
    let picker = Picker::from_query_stdio().unwrap();
    commands.insert_resource(PickerResource(picker));
}

fn init_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let image = asset_server.load("hex/hex1.webp");

    commands.spawn(Sprite {
        image,
        ..default()
    });
}

fn refresh_picker(
    mut commands: Commands,
    _picker: ResMut<PickerResource>
) {
    // FIXME: Idk if I'm supposed to re-insert a resource to update it, or use interior mutability
    // somehow.
    commands.insert_resource(PickerResource(Picker::from_query_stdio().unwrap()));
}

fn draw_system(
    mut context: ResMut<RatatuiContext>,
    mut camera_widget: Single<&mut RatatuiCameraWidget>,
    _picker: Res<PickerResource>
) -> Result {
    context.draw(|frame| {
        let layout = Layout::new(
            Direction::Vertical,
            [Constraint::Percentage(60), Constraint::Fill(1)],
        ).split(frame.area());

        frame.render_widget(
            &mut **camera_widget,
            layout[0]
        );

        frame.render_widget(
            TuiLoggerWidget::default()
                .block(Block::bordered())
                .style(Style::default().bg(ratatui::style::Color::Reset)),
            layout[1]);
    })?;

    Ok(())
}

