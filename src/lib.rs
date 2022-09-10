use rand::prelude::*;
use rand::Rng;
use rand::seq::SliceRandom;
use rand_distr::{Normal, Triangular};
use std::fmt::Display;


fn num_layers() -> u8 {
    // Generate a random number of Creature modifier layers
    *[1, 1, 1, 2, 2, 3].choose(&mut rand::thread_rng()).unwrap()
}

pub struct Creature {
    layers: u8,
    modifiers: Vec<Coefficients>,
}

struct Coefficients {
    c: f64,
    b: f64,
    z: f64,
    x: u8,
    n: f64,
}

impl Coefficients {
    fn new() -> Coefficients {
        let mut rng = thread_rng();
        let tri_a = Triangular::new(0.0, 2.0, 1.0).unwrap();
        let tri_b = Triangular::new(-2.0, 2.0, 0.0).unwrap();
        let norm = Normal::new(0.0, 0.1).unwrap();

        let mut c = if rng.gen::<f64>() < 0.4 { 1.0 } else { rng.sample(tri_a) };
        let mut b = if rng.gen::<f64>() < 0.3 { 1.0 } else { rng.sample(tri_a) };
        let z = if rng.gen::<f64>() < 0.4 { 0.0 } else { rng.sample(tri_b) };

        if rng.gen::<f64>() < 0.5 { c = -c; }
        if rng.gen::<f64>() < 0.5 { b = -b; }

        let x = match rng.gen::<f64>() {
            0.0..=0.4 => 1,
            0.4..=0.75 => 2,
            _ => 3,
        };

        let n = if rng.gen::<f64>() < 0.2 { 0.0 } else { rng.sample(norm) };

        Coefficients { c, b, z, x, n }
    }
}

impl Creature {
    pub fn new() -> Creature {
        let layers = num_layers();
        let mut modifiers = Vec::new();
        for _ in 0..layers {
            modifiers.push(Coefficients::new());
        }
        Creature { layers, modifiers }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creature_creation() {
        let creature = Creature::new();
        assert_eq!((creature.modifiers[0].c.abs() + creature.modifiers[0].b.abs()) > 0.0, true)
    }

    #[test]
    fn num_layer_bounds() {
        let layers: Vec<u8> = (0..100).map(|_| num_layers()).collect();
        assert_eq!(*layers.iter().min().unwrap(), 1 as u8);
        assert_eq!(*layers.iter().max().unwrap(), 3 as u8);
    }
}
