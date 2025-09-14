use bevy::{
    app::PluginGroupBuilder,
    render::view::{Layer, RenderLayers},
};
use bevy_inspector_egui::{
    bevy_egui::{EguiGlobalSettings, EguiPlugin, PrimaryEguiContext},
    quick::WorldInspectorPlugin,
};

use crate::core::cameras::{CameraPaintOrder, PaintLayer};
use crate::prelude::*;

pub struct InspectorPlugins;

impl PluginGroup for InspectorPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(EguiPlugin::default())
            .add(WorldInspectorPlugin::default())
            .add(inspector_camera_plugin)
    }
}

pub fn inspector_camera_plugin(app: &mut App) {
    app.add_systems(Startup, setup_inspector_camera);
}

fn setup_inspector_camera(
    mut commands: Commands,
    mut egui_global_settings: ResMut<EguiGlobalSettings>,
) {
    egui_global_settings.auto_create_primary_context = false;

    commands.spawn((OverlayCamera, PrimaryEguiContext));
}

#[derive(Component)]
#[require(Camera2d, Name::new("Overlay Camera"), RenderLayers::layer(PaintLayer::Overlay as Layer), Camera{order: CameraPaintOrder::Overlay as isize, ..default()})]
pub struct OverlayCamera;
