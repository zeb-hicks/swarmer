use bevy::{prelude::*, render::{camera::RenderTarget, render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages}, texture::ImageSampler, view::RenderLayers}, window::WindowResized};

const RES_WIDTH: u32 = 512;
const RES_HEIGHT: u32 = 288;

pub const PIXEL_LAYER: RenderLayers = RenderLayers::layer(0);
pub const HIRES_LAYER: RenderLayers = RenderLayers::layer(1);

#[derive(Component)]
pub struct Canvas;

#[derive(Component)]
pub struct GameCamera;

#[derive(Component)]
pub struct FinalCamera;

pub struct PixelatePlugin;

impl Plugin for PixelatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::Off);
        app.add_systems(Startup, setup_camera);
		app.add_systems(Update, resize_window);
    }
}

fn setup_camera(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    let canvas_size = Extent3d {
        width: RES_WIDTH,
        height: RES_HEIGHT,
        ..default()
    };

    let mut canvas = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: canvas_size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        sampler: ImageSampler::nearest(),
        ..default()
    };

    canvas.resize(canvas_size);

    let image_handle = images.add(canvas);

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: -1,
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            ..default()
        },
        GameCamera,
        PIXEL_LAYER
    ));

    commands.spawn((
        SpriteBundle {
            texture: image_handle,
            ..default()
        },
        Canvas,
        HIRES_LAYER,
    ));

    commands.spawn((Camera2dBundle::default(), FinalCamera, HIRES_LAYER));
}

fn resize_window(
    mut resize_events: EventReader<WindowResized>,
    mut projections: Query<&mut OrthographicProjection, With<FinalCamera>>,
) {
    for event in resize_events.read() {
        let h_scale = event.width / RES_WIDTH as f32;
        let v_scale = event.height / RES_HEIGHT as f32;
        let mut projection = projections.single_mut();
        projection.scale = 1. / h_scale.min(v_scale).round();
    }
}
