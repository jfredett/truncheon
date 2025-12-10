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

fn prep() {
    // set up dioxus
    dioxus_devtools::connect_subsecond();

    // set up TUI Logger, needs to be done before the app starts.
    tui_logger::init_logger(LevelFilter::Trace).unwrap();
    tui_logger::set_default_level(LevelFilter::Info);

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
        .add_systems(Startup, (init_picker, init_scene, init_ratatui_camera))
        .add_systems(PreUpdate, refresh_picker)
        .add_systems(Update, (draw_system, zoom_camera).chain())
        .run();
}


#[derive(Component)]
#[require(Camera2d)]
#[require(RatatuiCamera)]
struct MainCamera;

fn zoom_camera(
    mut camera: Single<&mut Transform, With<MainCamera>>,
    time: Res<Time>,
) {
    // info!("Translating!");
    let direction = Vec3::new(0.0, 0.0, 1.0);
    // camera.translation = camera.translation.lerp(direction, time.delta_secs() * 2.0);
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
    let image = asset_server.load("hex/hex1.png");

    commands.spawn(Sprite {
        image,
        image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: true,
                stretch_value: 0.5, // The image will tile every 128px
        },
        custom_size: Some(Vec2::new(248.0, 194.0)),
        ..default()
    });
}

fn refresh_picker(
    mut commands: Commands,
    picker: ResMut<PickerResource>
) {
    // FIXME: Idk if I'm supposed to re-insert a resource to update it, or use interior mutability
    // somehow.
    commands.insert_resource(PickerResource(Picker::from_query_stdio().unwrap()));
}

fn draw_system(
    mut context: ResMut<RatatuiContext>,
    mut camera_widget: Single<&mut RatatuiCameraWidget>,
    picker: Res<PickerResource>
) -> Result {
    context.draw(|frame| {
        let layout = Layout::new(
            Direction::Vertical,
            [Constraint::Percentage(75), Constraint::Fill(1)],
        ).split(frame.area());

        let (font_w, font_h) = picker.font_size();

        // this is resizing the image to the layout space, which is measured in characters (I
        // think), need it in pixels for the camera.
        let new_area = ratatui::prelude::Rect {
            x: layout[1].x * font_w,
            y: layout[1].y * font_h,
            width: (layout[1].width * font_w),
            height: (layout[1].height * font_h)
        };
        let (camera_image, _, _) = camera_widget.resize_images_to_area(new_area);
        let mut camera_image = picker.new_resize_protocol(camera_image);

        // FIXME: this should probably be a component?
        // TODO: Wrap a border on the thing
        let ratatui_image_widget = StatefulImage::default();
        ratatui_image_widget.render(layout[0], frame.buffer_mut(), &mut camera_image);

        frame.render_widget(
            TuiLoggerWidget::default()
                .block(Block::bordered())
                .style(Style::default().bg(ratatui::style::Color::Reset)),
            layout[1]);
    })?;

    Ok(())
}

