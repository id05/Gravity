//mod gravity;

use bevy::input::mouse::{MouseMotion, MouseWheel};
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

const SCALE_MULT: f32 = -0.5;

fn camera_scale(
    mut camera: Query<&mut Transform, With<Camera>>,
    mut scroll_evr: EventReader<MouseWheel>,
) {
    if let Ok(mut cam) = camera.get_single_mut() {
        for ev in scroll_evr.iter() {
            let forward = cam.forward();
            cam.translation += ev.y * SCALE_MULT * forward;
        }
    }
}

const ROTATION_SPEED: f32 = 0.01;
fn camera_rotate(
    mut camera: Query<&mut Transform, With<Camera>>,
    buttons: Res<Input<MouseButton>>,
    mut motion_evr: EventReader<MouseMotion>,
) {
    if let Ok(mut cam) = camera.get_single_mut() {
        if buttons.pressed(MouseButton::Left) {
            for drag in motion_evr.iter() {
                let drag_vec = Vec3 {
                    x: drag.delta.x,
                    y: -drag.delta.y,
                    z: 0.0,
                };
                let angle = drag_vec.length() * ROTATION_SPEED;
                let axis = cam
                    .forward()
                    .cross(cam.rotation.mul_vec3(drag_vec))
                    .normalize();
                cam.rotate_around(Vec3::ZERO, Quat::from_axis_angle(axis, angle));
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
    let mat = materials.add(StandardMaterial {
        base_color: Color::rgb(1.0, 0.3, 1.0),

        ..default()
    });

    for _ in 0..5 {
        commands.spawn_bundle(BallBundle::new(
            sphere_handle.clone(),
            mat.clone(),
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
        brightness: 0.02,
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
        .add_system(collision_system)
        .add_system(camera_scale)
        .add_system(camera_rotate)
        .run();
}
