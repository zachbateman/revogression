use rand::prelude::*;
use rand::Rng;
use rand::seq::SliceRandom;
use rand_distr::{Normal, Triangular};
use std::collections::HashMap;
use std::fmt;
use rayon::prelude::*;


fn num_layers() -> u8 {
    // Generate a random number of Creature modifier layers
    *[1, 1, 1, 2, 2, 3].choose(&mut rand::thread_rng()).unwrap()
}


/// A "Creature" is essentially a randomly generated function.
/// The equation of a creature can be one or more Coefficients in one or more
/// LayerModifiers which function as one or more layers for a simple neural network.
pub struct Creature<'a> {
    equation: Vec<LayerModifiers<'a>>,
}

impl Creature<'_> {
    pub fn num_layers(&self) -> usize {
        self.equation.len()
    }

    /// Calculate the resulting output value for this creature given an input of Key: Value data.
    pub fn calculate(&self, parameters: &HashMap<&str, f32>) -> f32 {
        let mut total = 0.0;
        let mut inner_total = 0.0;

        for layer_modifiers in &self.equation {
            // Run through each input parameter and record impact
            // for each parameter that is used in the curret layer's modifiers.
            for (param, param_value) in parameters {
                match layer_modifiers.modifiers.get(&param as &str) {
                    Some(coefficients) => { inner_total += coefficients.calculate(&param_value); },
                    None => (),
                }
            }

            // Check if current layer applies coefficients to the total after previous layer
            // Since "total" is updated at the end of each full layer, that same "total"
            // is the resulf of the prevous layer used as an input parameter for a "T".
            // if Some(layer_coefficients) == layer_modifiers.T {
            //     inner_total += layer_coefficients.calculate(&total);
            // }
            match &layer_modifiers.previous_layer_coefficients {
                Some(t_coefficients) => { inner_total += t_coefficients.calculate(&total); },
                _ => (),
            }

            // Add in the bias "layer_bias" to the current layer's calculation.
            total = inner_total + layer_modifiers.layer_bias;
        }
        total
    }

    pub fn create_many<'a>(num_creatures: i32, parameter_options: &'a Vec<&str>) -> Vec<Creature<'a>> {
        let creatures: Vec<Creature> = (0..num_creatures)
            .map(|_| Creature::new(&parameter_options))
            .collect();
        creatures
    }

    pub fn create_many_parallel<'a>(num_creatures: i32, parameter_options: &'a Vec<&str>) -> Vec<Creature<'a>> {
        let creatures: Vec<Creature> = (0..num_creatures)
            .into_par_iter()
            .map(|_| Creature::new(&parameter_options))
            .collect();
        creatures
    }

    pub fn new<'a>(parameter_options: &'a Vec<&str>) -> Creature<'a> {
        let mut equation = Vec::new();
        for layer in 0..num_layers() {
            equation.push(LayerModifiers::new(
                if layer == 0 { true } else {false},
                &parameter_options,
            ));
        }
        Creature { equation }
    }
}

impl fmt::Display for Creature<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n", "Creature")?;
        //write!(f, "Creature:\n({}, {})", self.num_layers(), self.equation)
        for (i, layer_mod) in self.equation.iter().enumerate() {
            write!(f, "  Layer {}\n{}", i+1, layer_mod)?;
        }
        Ok(())
    }
}

/// Each "LayerModifiers" represents a full neural network layer.
/// "modifiers" is a collection of Coefficents applied to certain input parameters.
/// The "previous_layer_coefficients" field is Coefficients applied to a previous layer's output, if applicable.
/// The "layer_bias" field is a bias added to the layer's calculation.
struct LayerModifiers<'a> {
    modifiers: HashMap<&'a str, Coefficients>,
    previous_layer_coefficients: Option<Coefficients>,
    layer_bias: f32,
}

impl LayerModifiers<'_> {
    fn new<'a>(first_layer: bool, parameter_options: &'a Vec<&str>) -> LayerModifiers<'a> {
        let mut rng = thread_rng();

        let mut modifiers = HashMap::new();
        let param_usage_scalar = 2.5 / (parameter_options.len() as f64 + 1.0);
        for &param in parameter_options {
            if rng.gen::<f64>() < param_usage_scalar {
                modifiers.insert(param, Coefficients::new());
            }
        }

        let previous_layer_coefficients = match first_layer {
            false => Some(Coefficients::new()),
            true => None,
        };

        let norm = Normal::new(0.0, 0.1).unwrap();
        let layer_bias = match rng.gen::<f64>() {
            x if x >= 0.0 && x <= 0.2 => 0.0,
            _ => rng.sample(norm),
        };
        LayerModifiers { modifiers, previous_layer_coefficients, layer_bias }
    }
}
impl fmt::Display for LayerModifiers<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "    Bias:  {}\n", self.layer_bias)?;
        match &self.previous_layer_coefficients {
            Some(coeff) => write!(f, "    Previous Layer:   ->  {}\n", coeff)?,
            _ => (),
        }
        for (key, coeff) in &self.modifiers {
            write!(f, "    Param \"{}\"   ->   {}\n", key, coeff)?;
        }
        Ok(())
    }
}

/// A "Coefficients" struct contains 4 values which
/// are used to form the following equation given input "param":
/// Value = C * (B * param + Z) ^ X
struct Coefficients { c: f32, b: f32, z: f32, x: u8 }

impl Coefficients {
    fn calculate(&self, &param_value: &f32) -> f32 {
        &self.c * (&self.b * &param_value + &self.z).powi(self.x as i32)
    }
    fn new() -> Coefficients {
        let mut rng = thread_rng();
        let tri_a = Triangular::new(0.0, 2.0, 1.0).unwrap();
        let tri_b = Triangular::new(-2.0, 2.0, 0.0).unwrap();
        // let norm = Normal::new(0.0, 0.1).unwrap();

        let mut c = if rng.gen::<f64>() < 0.4 { 1.0 } else { rng.sample(tri_a) };
        let mut b = if rng.gen::<f64>() < 0.3 { 1.0 } else { rng.sample(tri_a) };
        let z = if rng.gen::<f64>() < 0.4 { 0.0 } else { rng.sample(tri_b) };

        if rng.gen::<f64>() < 0.5 { c = -c; }
        if rng.gen::<f64>() < 0.5 { b = -b; }

        let x = match rng.gen::<f64>() {
            x if x >= 0.0 && x <= 0.4 => 1,
            x if x >= 0.4 && x <= 0.75 => 2,
            _ => 3,
        };
        Coefficients { c, b, z, x }
    }
}
impl fmt::Display for Coefficients {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} * ({} * param + {}) ^ {}", self.c, self.b, self.z, self.x)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn creature_creation() {
        let param_options = vec!["width", "height", "weight"];
        let creature = Creature::new(&param_options);
        println!("\n\n{}\n", creature);

        assert_eq!(creature.num_layers() >= 1 && creature.num_layers() <= 3, true);

        let test_coeff = creature.equation[0].modifiers.values().next()
            .expect("\n--> OKAY if this fails occasionally as it is possible to \
                     \ngenerate a creature with no modifiers for the first layer.");
        println!("{}", test_coeff);
        assert_eq!((test_coeff.c.abs() + test_coeff.b.abs()) > 0.0, true);

        let input_data = HashMap::from([("width", 2.1245), ("height", 0.52412)]);

        let mut creatures = Vec::new();
        for _ in 0..15 {
            creatures.push(Creature::new(&param_options));
        }
        println!("\n{}", creatures[5]);
        println!("\n{}", creatures[10]);
        println!("\n{}\n", creatures[12]);

        let mut total = 0.0;
        let mut result;
        for cr in creatures {
            result = cr.calculate(&input_data);
            println!("{}", result);
            total += result;
        }
        assert_eq!(total != 0.0, true);
    }

    #[test]
    fn generate_many_creatures() {
        let param_options = vec!["width", "height", "weight"];
        //let mut creatures = Vec::new();

        let t0 = Instant::now();
        Creature::create_many(100000, &param_options);
        let single = Instant::now() - t0;
        println!("\nSingle Thread: {:.2?}", single);

        let t0 = Instant::now();
        Creature::create_many_parallel(100000, &param_options);
        let multi = Instant::now() - t0;
        println!("Multiple Threads: {:.2?}", multi);

        println!("Multicore Speed: {:.1}x\n", single.as_millis() as f32 / multi.as_millis() as f32);
    }

    #[test]
    fn num_layer_bounds() {
        let layers: Vec<u8> = (0..10000).map(|_| num_layers()).collect();
        assert_eq!(*layers.iter().min().unwrap(), 1 as u8);
        assert_eq!(*layers.iter().max().unwrap(), 3 as u8);
    }
}