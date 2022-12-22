use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rapier3d::dynamics::MotorModel;

use crate::model;

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
    let robot_length = 0.5;
    let robot_width = 0.5;
    let wheel_radius = 0.2;
    let wheel_width = 0.05;
    let caster_radius = 0.1;
    let base_link_transform = Transform::from_xyz(0.0, 0.0, 0.3);

    let base_mesh = Mesh::from(shape::Cube { size: 0.2 });
    let base_material_handle = materials.add(StandardMaterial {
        base_color: Color::CRIMSON,
        ..Default::default()
    });
    let base_entity_id = commands
        .spawn((
            model::LinkName("base_link".into()),
            RigidBody::Dynamic,
            Collider::from_bevy_mesh(&base_mesh, &ComputedColliderShape::TriMesh).unwrap(),
            ColliderMassProperties::Mass(10.0),
            ColliderDebugColor(Color::hsl(220.0, 1.0, 0.3)),
            PbrBundle {
                mesh: meshes.add(base_mesh),
                material: base_material_handle,
                transform: base_link_transform,
                ..Default::default()
            },
        ))
        .id();

    let caster_mesh = Mesh::from(shape::Cube {
        size: caster_radius,
    });
    let caster_material_handle = materials.add(StandardMaterial {
        base_color: Color::BLUE,
        ..Default::default()
    });
    let caster_origin = Vec3::new(-robot_length, 0.0, -wheel_radius + caster_radius);
    let caster_joint = FixedJointBuilder::new().local_anchor1(caster_origin);

    commands.spawn((
        model::LinkName("caster_link".into()),
        RigidBody::Dynamic,
        Collider::ball(caster_radius),
        ColliderMassProperties::Mass(10.0),
        ColliderDebugColor(Color::hsl(220.0, 1.0, 0.3)),
        Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        },
        ImpulseJoint::new(base_entity_id, caster_joint),
        PbrBundle {
            mesh: meshes.add(caster_mesh),
            material: caster_material_handle,
            transform: base_link_transform
                * Transform::from_xyz(caster_origin.x, caster_origin.y, caster_origin.z),
            ..Default::default()
        },
    ));

    // Origins and names of wheels in base_link frame.
    let wheels: &[(Vec3, &str)] = &[
        (Vec3::new(0.0, robot_width / 2.0, 0.0), "left_wheel"),
        (Vec3::new(0.0, -robot_width / 2.0, 0.0), "right_wheel"),
    ];
    for (wheel_origin, link_name) in wheels {
        let wheel_transform = Transform {
            translation: *wheel_origin,
            rotation: Quat::IDENTITY,
            ..default()
        };
        let wheel_mesh = Mesh::from(shape::Torus {
            radius: wheel_radius - wheel_width,
            ring_radius: wheel_width,
            subdivisions_segments: 12,
            subdivisions_sides: 8,
        });
        let wheel_material_handle = materials.add(StandardMaterial {
            base_color: Color::AZURE,
            ..Default::default()
        });
        let joint = RevoluteJointBuilder::new(Vec3::Y)
            .local_anchor1(*wheel_origin)
            .motor_model(MotorModel::ForceBased)
            .motor_max_force(40.0);

        commands.spawn((
            model::LinkName(String::from(*link_name)),
            RigidBody::Dynamic,
            Collider::cylinder(wheel_width, wheel_radius),
            ColliderMassProperties::Mass(1.0),
            ColliderDebugColor(Color::hsl(220.0, 1.0, 0.3)),
            Friction {
                coefficient: 0.8,
                combine_rule: CoefficientCombineRule::Max,
            },
            ImpulseJoint::new(base_entity_id, joint),
            // If we don't disable sleeping, the joint motor seems to fail
            // sometimes.
            Sleeping::disabled(),
            PbrBundle {
                mesh: meshes.add(wheel_mesh),
                material: wheel_material_handle,
                transform: base_link_transform * wheel_transform,
                ..Default::default()
            },
        ));
    }
}
