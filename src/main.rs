mod camera_control;

use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::render::texture::ImageSampler;

#[derive(Resource)]
pub struct CreateWorldState {
    assets_loaded: bool,
    world_created: bool,
    map_texture_i: usize,
    map_texture_uris: Vec<String>,
    map_texture_handle: Handle<Image>,
}

impl Default for CreateWorldState {
    fn default() -> Self {
        Self {
            assets_loaded: false,
            world_created: false,
            map_texture_i: 0,
            map_texture_uris: vec!["willow_garage_map.png".to_string()],
            map_texture_handle: Default::default(),
        }
    }
}

fn make_default_texture() -> Image {
    let width: u32 = 32;
    let height: u32 = 32;
    let mut image_data = Vec::with_capacity(width as usize * height as usize);
    for ii in 0..width {
        for jj in 0..height {
            if (ii + jj) % 2 == 0 {
                image_data.extend_from_slice(&[255, 255, 255, 0]);
            } else {
                image_data.extend_from_slice(&[100, 100, 100, 100]);
            }
        }
    }
    let mut image = Image::new(
        Extent3d {
            width: width,
            height: height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        image_data,
        TextureFormat::Rgba8Unorm,
    );
    image.sampler_descriptor = ImageSampler::nearest();
    return image;
}

fn create_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut create_world_state: ResMut<CreateWorldState>,
) {
    if create_world_state.world_created {
    } else if create_world_state.assets_loaded {
        let material_handle = materials.add(StandardMaterial {
            base_color_texture: Some(create_world_state.map_texture_handle.clone()),
            unlit: true,
            ..Default::default()
        });

        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: material_handle,
            ..Default::default()
        });
        commands.spawn(PointLightBundle {
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..Default::default()
        });
        commands.spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        });
        create_world_state.world_created = true;
    } else {
        match asset_server.get_load_state(create_world_state.map_texture_handle.id()) {
            bevy::asset::LoadState::NotLoaded => {
                if create_world_state.map_texture_i < create_world_state.map_texture_uris.len() {
                    let map_uri = create_world_state.map_texture_uris
                        [create_world_state.map_texture_i]
                        .clone();
                    println!("Loading map texture: {}", map_uri);
                    create_world_state.map_texture_handle = asset_server.load(map_uri);
                } else {
                    println!("Failed to load any map texture, using default");
                    create_world_state.map_texture_handle = images.add(make_default_texture());
                    create_world_state.assets_loaded = true;
                }
            }
            bevy::asset::LoadState::Loading => {
                // Continue waiting for texture to load
            }
            bevy::asset::LoadState::Loaded => {
                create_world_state.assets_loaded = true;
            }
            bevy::asset::LoadState::Failed => {
                // Reset the texture and try the next one
                create_world_state.map_texture_handle = Default::default();
                create_world_state.map_texture_i += 1;
            }
            bevy::asset::LoadState::Unloaded => {
                panic!("Map texture unexpectedly unloaded");
            }
        }
    }
}

pub struct MapEditorPlugin;
impl Plugin for MapEditorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CreateWorldState>()
            .add_system(create_world)
            .add_plugin(camera_control::CameraControlPlugin);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MapEditorPlugin)
        .run();
}
