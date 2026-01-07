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
    tui_logger::set_default_level(LevelFilter::Trace);

    let subscriber = Registry::default().with(tui_logger::TuiTracingSubscriberLayer);
    bevy::log::tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to install tracing subscriber");
}

fn main() {
    prep();

    // let mut parameters = Parameters::default();
    let frame_time = std::time::Duration::from_secs_f32(1. / 10.);

    App::new()
        .add_plugins(
            DefaultPlugins.build()
                .disable::<WinitPlugin>()
                .disable::<LogPlugin>()
        )
        .add_plugins(bevy_mod_debugdump::CommandLineArgs)
        .add_plugins(ScheduleRunnerPlugin::run_loop(frame_time))
        .add_plugins(RatatuiPlugins {
            enable_input_forwarding: true,
            ..default()
        })
        .add_plugins(RatatuiCameraPlugin)
        .add_plugins(IOWidget::default())
        .add_systems(Startup, (init_picker, init_scene, init_ratatui_camera).chain())
        .add_systems(PreUpdate, refresh_picker)
        .add_systems(Update, draw_system)
        .run();
}


#[derive(Component)]
#[require(Camera2d, RatatuiCamera)]
struct MainCamera;

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
    mut io_widget: Single<&mut IOWidget>,
    _picker: Res<PickerResource>
) -> Result {
    context.draw(|frame| {
        // main two-row layout
        let layout = Layout::new(
            Direction::Vertical,
            [Constraint::Percentage(60), Constraint::Fill(1)],
        ).split(frame.area());

        // upper section two columns, IO on one side, image on the other
        let sublayout = Layout::new(
            Direction::Horizontal,
            [Constraint::Percentage(70), Constraint::Fill(1)],
        ).split(layout[0]);

        let map_section = sublayout[0];
        let io_section = sublayout[1];
        let log_section = layout[1];

        frame.render_widget(
            &mut **camera_widget,
            map_section
        );

        frame.render_widget(
            &mut **io_widget,
            io_section
        );

        frame.render_widget(
            TuiLoggerWidget::default()
                .block(Block::bordered())
                .style(Style::default().bg(ratatui::style::Color::Reset)),
            log_section);
    })?;

    Ok(())
}

