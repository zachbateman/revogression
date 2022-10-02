use std::collections::HashMap;
use crate::standardize::Standardizer;
use crate::creature::Creature;
// use rayon::prelude::*;


pub struct Evolution {
    target: String,
    standardized_data: Vec<HashMap<String, f32>>,
    num_creatures: u32,
    num_cycles: u16,
    standardizer: Standardizer,
    best_creatures: Vec<Creature>,
}

impl Evolution {
    fn new(
        target: String,
        data: &Vec<HashMap<String, f32>>,
        num_creatures: u32,
        num_cycles: u16,
        max_layers: u8,
    ) -> Evolution {

        let standardizer = Standardizer::new(&data[..]);
        let standardized_data = standardizer.standardized_values(data);

        let param_options = data[0].keys().map(|s| s.as_str()).collect();
        //let mut creatures = Creature::create_many_parallel(num_creatures, &param_options);

        let mut best_creatures = Vec::new();


        for cycle in 1..=num_cycles {
            let mut creatures = Creature::create_many_parallel(num_creatures, &param_options);

            for creature in creatures.iter_mut() {
                if creature.cached_error_sum == None {
                    let err = calc_error_sum(&creature, &standardized_data, &target);
                    creature.cached_error_sum = Some(err);
                }
            }

            let mut errors = Vec::new();
            for creature in creatures.iter() {
                errors.push(creature.cached_error_sum.unwrap());
            }
            errors.sort_by(|a, b| a.total_cmp(b));
            let median_error = errors[errors.len() / 2];
            let min_error = errors[0];
            let best_creature = creatures
                .iter()
                .find(|creature| creature.cached_error_sum == Some(min_error))
                .expect("Error matching min_error to a creature!");

            best_creatures.push(best_creature.clone());

            println!("Cycle {} complete!", cycle);
        }


        Evolution {
            target: target,
            standardized_data: standardized_data,
            num_creatures: num_creatures,
            num_cycles: num_cycles,
            standardizer: standardizer,
            best_creatures: best_creatures,
        }
    }
}


fn calc_error_sum(creature: &Creature,
                  data_points: &Vec<HashMap<String, f32>>,
                  target_param: &str) -> f32 {
    let mut total: f32 = 0.0;
    for point in data_points {
        let calc = creature.calculate(&point);
        let diff = calc - point.get(target_param)
                               .expect("Data point missing target_param");
        total += diff.powi(2);
    }
    total / (data_points.len() as f32)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_evolution() {
        let target = "target_param";
        let data = vec![
            HashMap::from([("target_param".to_string(), 5.2), ("p2".to_string(), 7.8), ("p3".to_string(), 8.3)]),
            HashMap::from([("target_param".to_string(), 6.0), ("p2".to_string(), 4.4), ("p3".to_string(), 8.1)]),
            HashMap::from([("target_param".to_string(), 7.1), ("p2".to_string(), 3.9), ("p3".to_string(), 9.5)]),
            HashMap::from([("target_param".to_string(), 8.6), ("p2".to_string(), 2.7), ("p3".to_string(), 11.6)]),
            HashMap::from([("target_param".to_string(), 9.4), ("p2".to_string(), -2.6), ("p3".to_string(), 13.0)]),
        ];

        let evo = Evolution::new(target.into(), &data, 10000, 10, 3);
        assert_eq!(evo.num_creatures == 10000, true);
    }
}
