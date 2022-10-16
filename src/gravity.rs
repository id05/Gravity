pub mod gravity {
    use bevy::prelude::{Mut, Query, Res, Time, Transform, Vec3};
    //use glam::f32::Vec3;

    use crate::{Mass, Speed};

    const G: f32 = 6.67430E-11;
    const GRAVITY_BOOSTER: f32 = 1000000f32;

    //rough calculation of impulse without integration

    fn gforce(m1: f32, m2: f32, r: f32) -> f32 {
        GRAVITY_BOOSTER * G * (m1 * m2) / (r * r)
    }

    pub fn gravity_system(
        time: Res<Time>,
        mut movables: Query<(&Mass, &mut Speed, &Transform)>,
        attractors: Query<(&Mass, &Transform)>,
    ) {
        for (mass1, mut speed, pos1) in movables.iter_mut() {
            speed.0 += attractors
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
