use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct SpawnRobotPlugin;
impl Plugin for SpawnRobotPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_robot);
    }
}

pub fn spawn_robot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let base_link_transform = Transform::from_xyz(0.0, 0.0, 0.3);

    let base_mesh = Mesh::from(shape::Cube { size: 0.2 });
    let base_material_handle = materials.add(StandardMaterial {
        base_color: Color::CRIMSON,
        ..Default::default()
    });
    let base_entity_id = commands
        .spawn((
            RigidBody::Dynamic,
            Collider::from_bevy_mesh(&base_mesh, &ComputedColliderShape::TriMesh).unwrap(),
            ColliderDebugColor(Color::hsl(220.0, 1.0, 0.3)),
            PbrBundle {
                mesh: meshes.add(base_mesh),
                material: base_material_handle,
                transform: base_link_transform,
                ..Default::default()
            },
        ))
        .id();

    // Origins of wheels in base_link frame.
    let l = 0.5;
    let w = 0.5;
    let wheels: &[Vec3] = &[
        Vec3::new(l / 2.0, w / 2.0, 0.0), 
        Vec3::new(-l / 2.0, w / 2.0, 0.0),
        Vec3::new(l / 2.0, -w / 2.0, 0.0),
        Vec3::new(-l / 2.0, -w / 2.0, 0.0),
    ];
    for wheel_origin in wheels {
        let wheel_transform = Transform {
            translation: *wheel_origin,
            rotation: Quat::IDENTITY,
            ..default()
        };
        let wheel_mesh = Mesh::from(shape::Torus {
            radius: 0.13,
            ring_radius: 0.1,
            subdivisions_segments: 12,
            subdivisions_sides: 8,
        });
        let wheel_material_handle = materials.add(StandardMaterial {
            base_color: Color::AZURE,
            ..Default::default()
        });
        let joint = RevoluteJointBuilder::new(Vec3::Y).local_anchor1(*wheel_origin);

        commands.spawn((
            RigidBody::Dynamic,
            Collider::from_bevy_mesh(&wheel_mesh, &ComputedColliderShape::TriMesh).unwrap(),
            ColliderDebugColor(Color::hsl(220.0, 1.0, 0.3)),
            ImpulseJoint::new(base_entity_id, joint),
            PbrBundle {
                mesh: meshes.add(wheel_mesh),
                material: wheel_material_handle,
                transform: base_link_transform * wheel_transform,
                ..Default::default()
            },
        ));
    }
}
