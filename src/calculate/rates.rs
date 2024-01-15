use super::consts::{Opts, DEATH};
use rand::{thread_rng, Rng};

pub fn new_rates() -> (Vec<Opts>, Vec<Opts>) {
    let mut rng = thread_rng();
    let mut interest: Vec<Opts> = vec![Opts::Float(0.0); DEATH];
    let mut inflation: Vec<Opts> = vec![Opts::Float(0.0); DEATH];

    for rate in inflation.iter_mut().take(DEATH) {
        *rate = Opts::Float(rng.gen_range(-0.005..0.04) + rng.gen_range(0.0..0.01));
    }

    for (idx, infl) in inflation.iter().enumerate() {
        match idx {
            0..=35 => {
                interest[idx] = Opts::Float(rng.gen_range(
                    (-0.075 + infl.get_float_ref() / 2.0)..(0.20 + infl.get_float_ref() / 2.0),
                ));
            }
            36..=49 => {
                interest[idx] = Opts::Float(rng.gen_range(
                    (-0.05 + infl.get_float_ref() / 2.0)..(0.175 + infl.get_float_ref() / 2.0),
                ));
            }
            50..=64 => {
                interest[idx] = Opts::Float(rng.gen_range(
                    (-0.035 + infl.get_float_ref() / 2.0)..(0.15 + infl.get_float_ref() / 2.0),
                ));
            }
            65..=80 => {
                interest[idx] = Opts::Float(rng.gen_range(
                    (-0.02 + infl.get_float_ref() / 2.0)..(0.125 + infl.get_float_ref() / 2.0),
                ));
            }
            _ => {
                interest[idx] = Opts::Float(rng.gen_range(
                    (-0.005 + infl.get_float_ref() / 2.0)..(0.1 + infl.get_float_ref() / 2.0),
                ));
            }
        }
    }
    (interest, inflation)
}
