//mod gravity;

use bevy::prelude::*;
//use gravity::gravity_system;

pub mod basic_physics;

use crate::basic_physics::gravity::*;
use crate::basic_physics::*;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(movement_system)
        .add_system(gravity_system)
        .add_system(collision_system)
        .run();
}
