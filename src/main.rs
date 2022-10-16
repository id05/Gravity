mod gravity;

use bevy::prelude::*;
use gravity::gravity::gravity_system;

#[derive(Component)]
pub struct Mass(f32);

#[derive(Component)]
pub struct Speed(Vec3);

fn movement_system(time: Res<Time>, mut movable_qery: Query<(&mut Transform, &Speed)>) {
    for (mut pos, speed) in movable_qery.iter_mut() {
        pos.translation += speed.0 * time.delta_seconds();
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn()
        .insert(Transform::from_translation(Vec3::ZERO))
        .insert(Mass(1000.0));
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.2,
                sectors: 128,
                stacks: 64,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(1.0, 0.3, 1.0),

                ..default()
            }),
            transform: Transform::from_xyz(1.0, 0.5, 0.5),
            ..default()
        })
        .insert(Mass(10000.0))
        .insert(Speed(Vec3::new(0.0, 0.0, 0.0)));

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.2,
                sectors: 128,
                stacks: 64,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.0, 0.6, 1.0),

                ..default()
            }),
            transform: Transform::from_xyz(1.0, 1.0, 0.0),
            ..default()
        })
        .insert(Mass(10000.0))
        .insert(Speed(Vec3::new(0.5, 0.0, -0.5)));

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.2,
                sectors: 128,
                stacks: 64,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(1.0, 0.6, 0.6),

                ..default()
            }),
            transform: Transform::from_xyz(0.0, 1.0, 1.0),
            ..default()
        })
        .insert(Mass(10000.0))
        .insert(Speed(Vec3::new(0.0, -0.5, 0.5)));

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.2,
                sectors: 128,
                stacks: 64,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.0, 1.0, 0.6),

                ..default()
            }),
            transform: Transform::from_xyz(0.0, -1.0, -1.0),
            ..default()
        })
        .insert(Mass(10000.0))
        .insert(Speed(Vec3::new(0.0, 0.5, -0.5)));

    //light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(3.0, 3.0, 3.0),
        ..default()
    });

    //camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(15.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(movement_system)
        .add_system(gravity_system)
        .run();
}
