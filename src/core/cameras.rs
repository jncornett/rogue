use bevy::{
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::{Layer, RenderLayers},
    },
    window::PrimaryWindow,
};

use crate::prelude::*;

pub enum CameraPaintOrder {
    World = -2,
    Canvas = -1,
    Overlay = 0,
}

pub enum PaintLayer {
    World = 0,
    Canvas = 1,
    Overlay = 2,
}

#[derive(Component)]
#[require(Camera2d, Name::new("World Camera"), Msaa::Off, Pan, Zoom, RenderLayers::layer(PaintLayer::World as Layer))]
pub struct WorldCamera;

#[derive(Component, Default, Deref)]
#[cfg_attr(feature = "dev", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "dev", reflect(Component, InspectorOptions))]
pub struct Pan(pub Vec2);

#[derive(Component, Deref)]
#[cfg_attr(feature = "dev", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "dev", reflect(Component, InspectorOptions))]
pub struct Zoom(pub f32);

impl Default for Zoom {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(Component)]
#[require(Name::new("Canvas"),RenderLayers::layer(PaintLayer::Canvas as Layer))]
pub struct Canvas;

#[derive(Resource)]
pub struct CanvasSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Component)]
#[require(Camera2d, Name::new("Screen Camera"), Camera{order: CameraPaintOrder::Canvas as isize, ..default()}, RenderLayers::layer(PaintLayer::Canvas as Layer))]
pub struct ScreenCamera;

impl Default for CanvasSize {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
        }
    }
}

pub fn cameras_plugin(app: &mut App) {
    #[cfg(feature = "dev")]
    app.register_type::<Zoom>().register_type::<Pan>();
    app.init_resource::<CanvasSize>()
        .add_systems(Startup, setup_cameras)
        .add_systems(PostUpdate, (pan_and_zoom, fit_screen).chain());
}

fn setup_cameras(
    mut commands: Commands,
    mut textures: ResMut<Assets<Image>>,
    canvas_size: Res<CanvasSize>,
) {
    let canvas_size = Extent3d {
        width: canvas_size.width,
        height: canvas_size.height,
        ..default()
    };

    let mut canvas_texture = Image {
        texture_descriptor: new_canvas_texture_descriptor(canvas_size),
        ..default()
    };
    canvas_texture.resize(canvas_size);

    let canvas_handle = textures.add(canvas_texture);

    commands.spawn((WorldCamera, new_world_camera(canvas_handle.clone())));
    commands.spawn((Canvas, Sprite::from_image(canvas_handle)));
    commands.spawn(ScreenCamera);
}

fn fit_screen(
    mut projection: Single<&mut Projection, With<ScreenCamera>>,
    canvas: Single<&Sprite, (With<Canvas>, Without<ScreenCamera>)>,
    textures: Res<Assets<Image>>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let Projection::Orthographic(projection) = &mut **projection else {
        return;
    };

    let Some(canvas_size) = textures.get(&canvas.image).map(Image::size) else {
        return;
    };

    let window_size = Vec2::new(window.resolution.width(), window.resolution.height());

    let h_scale = window_size.x / canvas_size.y as f32;
    let v_scale = window_size.y / canvas_size.x as f32;
    projection.scale = 1. / h_scale.min(v_scale).round();
}

fn pan_and_zoom(
    camera: Single<(&mut Transform, &mut Projection, &Pan, &Zoom), With<WorldCamera>>,
    mut canvas: Single<&mut Sprite, With<Canvas>>,
    textures: Res<Assets<Image>>,
) {
    let (mut xf, mut projection, pan, zoom) = camera.into_inner();

    if let Projection::Orthographic(p) = &mut *projection {
        p.scale = **zoom;
    } else {
        warn!("cameras: unsupported camera perspective: {projection:?}");
    }

    let Some(image) = textures.get(&canvas.image) else {
        return;
    };

    xf.translation = pan.trunc().extend(xf.translation.z);

    let remainder = Vec2::new(pan.x, -pan.y) % 1.0;

    canvas.rect = Some(Rect {
        min: Vec2::ONE + remainder,
        max: image.size_f32() - Vec2::ONE + remainder,
    });
}

fn new_world_camera(canvas: Handle<Image>) -> Camera {
    Camera {
        order: CameraPaintOrder::World as isize,
        target: RenderTarget::Image(canvas.into()),
        msaa_writeback: false,
        ..default()
    }
}

fn new_canvas_texture_descriptor<'a>(size: Extent3d) -> TextureDescriptor<'a> {
    TextureDescriptor {
        label: None,
        size,
        dimension: TextureDimension::D2,
        format: TextureFormat::Bgra8UnormSrgb,
        mip_level_count: 1,
        sample_count: 1,
        usage: TextureUsages::TEXTURE_BINDING
            | TextureUsages::COPY_DST
            | TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    }
}
