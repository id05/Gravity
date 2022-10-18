use bevy::prelude::*;

#[derive(Component)]
pub struct Mass(pub f32);

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct BouncingBall(pub f32);

//primitive collsision system whitch can work only with spheres
pub fn collision_system(mut bounce_qery: Query<(&Mass, &BouncingBall, &Transform, &mut Velocity)>) {
    let mut combinations = bounce_qery.iter_combinations_mut::<2>();
    while let Some([(mass1, radius1, pos1, mut vel1), (mass2, radius2, pos2, mut vel2)]) =
        combinations.fetch_next()
    {
        let distance = (pos1.translation - pos2.translation).length();
        let rad_sum = radius1.0 + radius2.0;
        if distance < rad_sum {
            let axis = (pos1.translation - pos2.translation).normalize(); //normilezed vector from pos1 to pos2

            let vp1 = vel1.0.project_onto(axis);
            let vp2 = vel2.0.project_onto(axis); //velocity projection

            vel1.0 -= vp1;
            vel2.0 -= vp2;

            let mvp1 = vp1.length(); //module of velocity projectoin
            let mvp2 = vp2.length();

            vel1.0 -=
                (2.0 * mass2.0 * mvp2 + (mass1.0 - mass2.0) * mvp1) / (mass1.0 + mass2.0) * -axis;
            vel2.0 -=
                (2.0 * mass1.0 * mvp1 + (mass2.0 - mass1.0) * mvp2) / (mass1.0 + mass2.0) * axis;
        }
    }
}

pub fn movement_system(
    time: Res<Time>,
    mut movable_qery: Query<(&mut Transform, &Velocity), Without<Camera>>,
) {
    for (mut pos, vel) in movable_qery.iter_mut() {
        pos.translation += vel.0 * time.delta_seconds();
    }
}

pub mod gravity {

    use bevy::prelude::{Query, Res, Time, Transform, Vec3};

    use crate::physics::{Mass, Velocity};

    const GRAVITY_MODIFYER: f32 = 2000000f32;
    const G: f32 = 6.67430E-11 * GRAVITY_MODIFYER;

    //rough calculation of impulse without integration

    pub fn gforce(m1: f32, m2: f32, r: f32) -> f32 {
        G * (m1 * m2) / (r * r)
    }

    pub fn gravity_system(
        time: Res<Time>,
        mut movables: Query<(&Mass, &mut Velocity, &Transform)>,
        attractors: Query<(&Mass, &Transform)>,
    ) {
        for (mass1, mut vel, pos1) in movables.iter_mut() {
            vel.0 += attractors
                .iter()
                .filter(|(_, pos2)| pos2.translation != pos1.translation)
                .map(|(mass2, pos2)| {
                    let axis = (pos2.translation - pos1.translation).normalize(); //norm vector from pos1 to pos2 for calculation of vector of impulse

                    let impulse_vec = gforce(
                        mass1.0,
                        mass2.0,
                        (pos1.translation - pos2.translation).length(),
                    ) * time.delta_seconds()
                        * axis;

                    impulse_vec / mass1.0
                })
                .fold(Vec3::new(0f32, 0f32, 0f32), |acc, x| acc + x);
        }
    }
}
