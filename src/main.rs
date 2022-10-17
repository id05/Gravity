use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

pub mod physics;

use crate::physics::gravity::*;
use crate::physics::*;
use rand::prelude::*;

#[derive(Bundle)]
pub struct BallBundle {
    #[bundle]
    pbr: PbrBundle,
    mass: Mass,
    vel: Velocity,
    collision: BouncingBall,
}

impl BallBundle {
    ///sphere must be with 1 unit radius!
    pub fn new(
        mesh: Handle<Mesh>,
        material: Handle<StandardMaterial>,
        density: f32,
        radius: f32,
        pos: Vec3,
        vel: Vec3,
    ) -> BallBundle {
        BallBundle {
            pbr: PbrBundle {
                mesh,
                material,
                transform: Transform::from_translation(pos)
                    .with_scale(Vec3::new(radius, radius, radius)),
                ..default()
            },
            mass: Mass(4.0 / 3.0 * 3.14159 * radius.powi(3) * density),
            vel: Velocity(vel),
            collision: BouncingBall(radius),
        }
    }
}

const CAMERA_SPEED_MULT: f32 = 5.0;

fn camera_movement(
    time: Res<Time>,
    mut movable_qery: Query<(&mut Transform, &Velocity), With<Camera>>,
) {
    for (mut pos, vel) in movable_qery.iter_mut() {
        let vel = pos.rotation.mul_vec3(vel.0);
        pos.translation += vel * time.delta_seconds() * CAMERA_SPEED_MULT;
    }
}

fn keyboard_controls(mut camera: Query<&mut Velocity, With<Camera>>, keys: Res<Input<KeyCode>>) {
    use KeyCode::{LControl, Space, A, D, S, W};
    if let Ok(mut vel) = camera.get_single_mut() {
        keys.get_just_pressed().for_each(|key| {
            match key {
                W => vel.0 += Vec3::NEG_Z,
                A => vel.0 += Vec3::NEG_X,
                S => vel.0 += Vec3::Z,
                D => vel.0 += Vec3::X,
                LControl => vel.0 += Vec3::NEG_Y,
                Space => vel.0 += Vec3::Y,
                _ => {}
            };
            //vel.0 += direction * CAMERA_SPEED_MULT;
        });
        keys.get_just_released().for_each(|key| {
            match key {
                W => vel.0 -= Vec3::NEG_Z,
                A => vel.0 -= Vec3::NEG_X,
                S => vel.0 -= Vec3::Z,
                D => vel.0 -= Vec3::X,
                LControl => vel.0 -= Vec3::NEG_Y,
                Space => vel.0 -= Vec3::Y,
                _ => {}
            };
            //vel.0 -= direction * CAMERA_SPEED_MULT;
        });
        //println!("camera speed: {}", vel.0);
    }
}

const ROTATION_OFFSET: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: -15.0,
};
const ROTATION_SPEED: f32 = 0.005;
fn camera_rotate(
    mut camera: Query<&mut Transform, With<Camera>>,
    buttons: Res<Input<MouseButton>>,
    mut motion_evr: EventReader<MouseMotion>,
) {
    if let Ok(mut camera) = camera.get_single_mut() {
        if buttons.pressed(MouseButton::Left) {
            for drag in motion_evr.iter() {
                let drag_vec = Vec3 {
                    x: drag.delta.x,
                    y: -drag.delta.y,
                    z: 0.0,
                };
                let angle = drag_vec.length() * ROTATION_SPEED;
                let axis = camera
                    .forward()
                    .cross(camera.rotation.mul_vec3(drag_vec))
                    .normalize();
                let pos = camera.translation;
                let offset = camera.rotation.mul_vec3(ROTATION_OFFSET);
                camera.rotate_around(pos + offset, Quat::from_axis_angle(axis, angle));
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();
    let sphere_handle = meshes.add(Mesh::from(shape::UVSphere {
        radius: 1.0,
        sectors: 128,
        stacks: 64,
    }));
    /*let mat = materials.add(StandardMaterial {
        base_color: Color::rgb(1.0, 0.3, 1.0),

        ..default()
    });
    */

    for _ in 0..=12 {
        commands.spawn_bundle(BallBundle::new(
            sphere_handle.clone(),
            materials.add(StandardMaterial {
                base_color: Color::rgb(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()),

                ..default()
            }),
            40000.0,
            0.4,
            Vec3::new(
                rng.gen::<f32>() * -10.0 + 5.0,
                rng.gen::<f32>() * -10.0 + 5.0,
                rng.gen::<f32>() * -10.0 + 5.0,
            ),
            Vec3::ZERO,
        ));
    }

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

    //embient
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.10,
    });

    //camera
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(15.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(Velocity(Vec3::ZERO));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(movement_system)
        .add_system(gravity_system)
        .add_system(collision_system)
        .add_system(keyboard_controls)
        .add_system(camera_rotate)
        .add_system(camera_movement)
        .run();
}
