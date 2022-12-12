use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::render::texture::ImageSampler;
use bevy_rapier3d::prelude::*;

pub struct CreateWorldPlugin;
impl Plugin for CreateWorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CreateWorldState>()
            .add_startup_system(create_axes)
            .add_system(create_world);
    }
}

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

pub fn create_world(
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

        // Graphics for the ground.
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: material_handle.clone(),
            transform: Transform::from_rotation(Quat::from_axis_angle(
                Vec3::X,
                std::f32::consts::FRAC_PI_2,
            )),
            ..Default::default()
        });
        // Collider for the ground.
        commands.spawn((
            TransformBundle::from(Transform::from_xyz(0.0, 0.0, -0.1)),
            Collider::cuboid(2.5, 2.5, 0.1),
        ));

        commands.spawn(PointLightBundle {
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..Default::default()
        });

        let wheel_mesh = Mesh::from(shape::Torus {
            radius: 0.3,
            ring_radius: 0.05,
            subdivisions_segments: 12,
            subdivisions_sides: 8,
        });
        let wheel_material_handle = materials.add(StandardMaterial {
            base_color: Color::AZURE,
            ..Default::default()
        });

        // Collider for falling object.
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(wheel_mesh),
                material: wheel_material_handle,
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..Default::default()
            },
            RigidBody::Dynamic,
            Collider::cuboid(0.1, 0.1, 0.1),
            ColliderDebugColor(Color::hsl(220.0, 1.0, 0.3)),
        ));
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

pub fn create_axes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Extents and colors for each arrow.
    let r = 0.03;
    let l = 1.0;
    let arrows: &[([f32; 6], [f32; 3])] = &[
        ([-r, l, -r, r, -r, r], [1.0, 0.0, 0.0]),
        ([-r, r, -r, l, -r, r], [0.0, 1.0, 0.0]),
        ([-r, r, -r, r, -r, l], [0.0, 0.0, 1.0]),
    ];

    for ([min_x, max_x, min_y, max_y, min_z, max_z], [r, g, b]) in arrows.iter() {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box {
                min_x: *min_x,
                max_x: *max_x,
                min_y: *min_y,
                max_y: *max_y,
                min_z: *min_z,
                max_z: *max_z,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::Rgba {
                    red: *r,
                    green: *g,
                    blue: *b,
                    alpha: 0.8,
                },
                alpha_mode: AlphaMode::Blend,
                ..default()
            }),
            ..default()
        });
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
