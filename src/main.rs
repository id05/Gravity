mod gravity;

use bevy::prelude::*;

#[derive(Component)]
pub struct Mass(f32);

#[derive(Component)]
pub struct Speed(Vec3);

fn movement_system(time: Res<Time>, mut movable_qery: Query<(&mut Transform, &Speed)>) {
    unimplemented!("implement movement system!");
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //Sphere in center of coords
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 1.0,
                sectors: 128,
                stacks: 64,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(1.0, 0.6, 0.6),
                //emissive: Color::rgba_linear(1.0, 1.0, 1.0, 0.0),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(Mass(1.0))
        .insert(Speed(Vec3::new(0.0, 0.0, 0.0)));

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.8,
                sectors: 128,
                stacks: 64,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(1.0, 0.6, 0.6),

                ..default()
            }),
            transform: Transform::from_xyz(2.0, 3.0, 1.0),
            ..default()
        })
        .insert(Mass(1.0))
        .insert(Speed(Vec3::new(0.0, 0.0, 0.0)));

    //light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    //camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(30.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}
