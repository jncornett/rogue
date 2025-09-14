use bevy::{app::PluginGroupBuilder, asset::AssetMetaCheck, window::WindowMode};

use crate::{cameras::cameras_plugin, prelude::*};

pub struct AppPlugins {
    pub fullscreen: bool,
}

impl PluginGroup for AppPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add_group(
                DefaultPlugins
                    .set(AssetPlugin {
                        meta_check: AssetMetaCheck::Never,
                        ..default()
                    })
                    .set(ImagePlugin::default_nearest())
                    .set(new_window_plugin(self.fullscreen)),
            )
            .add_group(PhysicsPlugins::default())
            .add(cameras_plugin)
            .add(|app: &mut App| {
                app.insert_resource(ClearColor(Color::BLACK))
                    .insert_resource(Gravity::ZERO);
            })
    }
}

fn new_window_plugin(fullscreen: bool) -> WindowPlugin {
    let mode = if fullscreen {
        WindowMode::BorderlessFullscreen(MonitorSelection::Current)
    } else {
        WindowMode::Windowed
    };
    let window = Window {
        mode,
        position: WindowPosition::Centered(MonitorSelection::Primary),
        resizable: true,
        ..default()
    };
    WindowPlugin {
        primary_window: Some(window),
        ..default()
    }
}
