pub mod gravity {
    use bevy::prelude::{Mut, Query, Res, Time, Transform};

    use crate::{Mass, Speed};

    const G: f32 = 6.67E-11;

    //rough calculation of impulse without integration

    fn gforce(m1: f32, m2: f32, r: f32) -> f32 {
        G * (m1 * m2) / (r * r)
    }

    pub fn gravity(time: Res<Time>, mut massive_query: Query<(&Mass, &mut Speed, &Transform)>) {
        let mut vec: Vec<(&Mass, Mut<Speed>, &Transform)> = massive_query.iter_mut().collect();
        for i in 0..vec.len() {
            //first i-1 elements are already processed, so they will be clipped and element i taken
            let (v1, v2) = vec.split_at_mut(i + 1);

            let (mass1, speed1, pos1) = &mut v1[i];

            for (mass2, speed2, pos2) in v2 {
                let axis = (pos2.translation - pos1.translation).normalize(); //norm vector from pos1 to pos2 for calculation of vector of impulse

                let impulse_vec = gforce(
                    mass1.0,
                    mass2.0,
                    pos1.translation.distance(pos2.translation),
                ) * time.delta_seconds()
                    * axis;

                speed1.0 += impulse_vec / mass1.0;
                // negate vector of impulse because is points FROM pos1
                // and we need it pointing TO pos1
                speed2.0 += (-impulse_vec) / mass2.0;
            }
        }
    }
}
