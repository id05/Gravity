use bevy::prelude::*;

#[derive(Component)]
struct Mass(f32);

#[derive(Component)]
struct Speed(Vec3);

fn gravity(time: Res<Time>, mut movables_query: Query<&mut Speed>, massive_query: Query<&Mass>) {
    unimplemented!("implement gravity!")
}

fn movement_system(time: Res<Time>, mut movable_qery: Query<(&mut Transform, &Speed)>) {
    unimplemented!("implement movement system!")
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
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
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
        transform: Transform::from_xyz(4.0, 6.0, 6.0),
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
