use std::time::Duration;

use bevy::render::view::{Layer, RenderLayers};

use crate::core::cameras::PaintLayer;
use crate::prelude::*;

pub fn placeholder_plugin(app: &mut App) {
    #[cfg(feature = "dev")]
    app.register_type::<PlaceholderInitialState>();
    app.add_systems(Startup, setup_assets)
        .add_systems(Update, animate_assets);
}

fn setup_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for i in 0..4 {
        let color = Color::hsl(i as f32 * 40.0, 0.6, 0.6);
        commands.spawn((
            Placeholder,
            PlaceholderInitialState {
                start: i as f32 * 1.0,
                speed: 1.0,
                color,
            },
            Mesh2d(meshes.add(Rectangle::new(30.0, 30.0))),
            MeshMaterial2d(materials.add(color)),
            Transform::from_translation(Vec3::ZERO),
        ));
    }
}

fn animate_assets(
    time: Res<Time>,
    mut query: Query<
        (
            &mut Transform,
            &MeshMaterial2d<ColorMaterial>,
            &PlaceholderInitialState,
        ),
        With<Placeholder>,
    >,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let elapsed = time.elapsed();

    for (mut xf, material, state) in &mut query {
        let (pos, color) = state.state(elapsed);
        xf.translation = pos.extend(0.0);
        if let Some(m) = materials.get_mut(material.clone()) {
            m.color = color;
        }
    }
}

#[derive(Component)]
#[require(Name::new("Placeholder"), RenderLayers::layer(PaintLayer::World as Layer))]
pub struct Placeholder;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "dev", reflect(Component, InspectorOptions))]
pub struct PlaceholderInitialState {
    start: f32,
    speed: f32,
    color: Color,
}

impl PlaceholderInitialState {
    pub fn state(&self, elapsed: Duration) -> (Vec2, Color) {
        let t = self.start + elapsed.as_secs_f32() * self.speed;
        let side = (t % 4.0).floor() as isize;
        let offset = (t % 4.0).fract();
        match side {
            0 => (Vec2::new(-100.0 + 200.0 * offset, -100.0), self.color),
            1 => (Vec2::new(100.0, -100.0 + 200.0 * offset), self.color),
            2 => (Vec2::new(100.0 - 200.0 * offset, 100.0), self.color),
            3 => (Vec2::new(-100.0, 100.0 - 200.0 * offset), self.color),
            _ => unreachable!(),
        }
    }
}
