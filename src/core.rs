use avian2d::{PhysicsPlugins, prelude::Gravity};
use bevy::{
    app::PluginGroupBuilder,
    prelude::*,
    window::{EnabledButtons, WindowMode},
};

use crate::cameras::cameras_plugin;

pub struct CorePlugins {
    pub fullscreen: bool,
}

impl PluginGroup for CorePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add_group(
                DefaultPlugins
                    .set(ImagePlugin::default_nearest())
                    .set(new_window_plugin(self.fullscreen)),
            )
            .add_group(PhysicsPlugins::default())
            .add(cameras_plugin)
            .add(GluePlugin)
    }
}

pub fn glue_plugin(app: &mut App) {
    app.insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Gravity::ZERO);
}

pub struct GluePlugin;

impl Plugin for GluePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .insert_resource(Gravity::ZERO);
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
        resizable: false,
        enabled_buttons: EnabledButtons {
            minimize: true,
            maximize: false,
            close: true,
        },
        ..default()
    };
    WindowPlugin {
        primary_window: Some(window),
        ..default()
    }
}
