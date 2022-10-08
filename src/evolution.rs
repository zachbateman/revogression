use std::collections::HashMap;
use crate::standardize::Standardizer;
use crate::creature::{Creature, MutateSpeed};
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

        let param_options = data[0].keys().map(|s| s.as_str())
            .filter(|s| s != &target.as_str()).collect();

        let mut creatures = Creature::create_many_parallel(num_creatures, &param_options);
        let mut best_creatures = Vec::new();

        for cycle in 1..=num_cycles {
            for creature in creatures.iter_mut() {
                if creature.cached_error_sum == None {
                    let err = calc_error_sum(&creature, &standardized_data, &target);
                    creature.cached_error_sum = Some(err);
                }
            }

            let (min_error, median_error) = error_results(&creatures);

            let best_creature = creatures
                .iter()
                .find(|creature| creature.cached_error_sum == Some(min_error))
                .expect("Error matching min_error to a creature!");
            best_creatures.push(best_creature.clone());
            print_cycle_data(cycle, median_error, best_creature);

            creatures = kill_weak_creatures(creatures, &median_error);
            creatures.append(&mut mutated_top_creatures(&creatures, &min_error, &median_error));

            // Now ensure creatures is correct length by cutting off extras
            // or adding newly generated Creatures to fill to num_creatures length.
            creatures.truncate(num_creatures as usize);
            if creatures.len() < num_creatures as usize {
                creatures.append(&mut Creature::create_many_parallel(
                    num_creatures - creatures.len() as u32, &param_options
                ));
            }
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

fn print_cycle_data(cycle: u16, median_error: f32, best_creature: &Creature) -> () {
    println!("---------------------------------------");
    println!("Cycle - {} -", cycle);
    println!("Median error: {}", median_error);
    println!("Best Creature:");
    println!("  Generation: {}   Error: {}", best_creature.generation, best_creature.cached_error_sum.unwrap());
    println!("{}", best_creature);
}

fn error_results(creatures: &Vec<Creature>) -> (f32, f32) {
    let mut errors = Vec::new();
    for creature in creatures.iter() {
        errors.push(creature.cached_error_sum.unwrap());
    }
    errors.sort_by(|a, b| a.total_cmp(b));
    let median_error = errors[errors.len() / 2];
    let min_error = errors[0];
    (min_error, median_error)
}

fn kill_weak_creatures(creatures: Vec<Creature>, median_error: &f32) -> Vec<Creature> {
    creatures.into_iter()
        .filter(|creature| creature.cached_error_sum.unwrap() < *median_error)
        .collect()
}

fn mutated_top_creatures(creatures: &Vec<Creature>, min_error: &f32, median_error: &f32) -> Vec<Creature> {
    let error_cutoff = (min_error + median_error) / 2.0;
    let mut mutants = Vec::new();
    for creature in creatures {
        if creature.cached_error_sum.unwrap() < error_cutoff {
            mutants.push(creature.mutate(MutateSpeed::Fast));
        }
    }
    mutants
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

    #[test]
    fn parabola() {
        let parabola_data = vec![
            HashMap::from([("x".to_string(), -20.0), ("y".to_string(), 195.0967073301952)]),
            HashMap::from([("x".to_string(), -19.0), ("y".to_string(), 205.88669941695193)]),
            HashMap::from([("x".to_string(), -18.0), ("y".to_string(), 75.05183418690936)]),
            HashMap::from([("x".to_string(), -17.0), ("y".to_string(), 153.31304897814132)]),
            HashMap::from([("x".to_string(), -16.0), ("y".to_string(), 180.72678834266526)]),
            HashMap::from([("x".to_string(), -15.0), ("y".to_string(), 81.73490536370575)]),
            HashMap::from([("x".to_string(), -14.0), ("y".to_string(), 76.98269474497451)]),
            HashMap::from([("x".to_string(), -13.0), ("y".to_string(), 106.65404246488129)]),
            HashMap::from([("x".to_string(), -12.0), ("y".to_string(), 101.81854634039516)]),
            HashMap::from([("x".to_string(), -11.0), ("y".to_string(), 32.735790537057994)]),
            HashMap::from([("x".to_string(), -10.0), ("y".to_string(), 3.5140689599924273)]),
            HashMap::from([("x".to_string(), -9.0), ("y".to_string(), 21.979234525796137)]),
            HashMap::from([("x".to_string(), -8.0), ("y".to_string(), 2.101943660864327)]),
            HashMap::from([("x".to_string(), -7.0), ("y".to_string(), 4.083877304799986)]),
            HashMap::from([("x".to_string(), -6.0), ("y".to_string(), 0.12110473958116565)]),
            HashMap::from([("x".to_string(), -5.0), ("y".to_string(), 16.57223235311977)]),
            HashMap::from([("x".to_string(), -4.0), ("y".to_string(), 0.14511553873582717)]),
            HashMap::from([("x".to_string(), -3.0), ("y".to_string(), 2.510511396206416)]),
            HashMap::from([("x".to_string(), -2.0), ("y".to_string(), 56.587670650914006)]),
            HashMap::from([("x".to_string(), -1.0), ("y".to_string(), 4.880296227847032)]),
            HashMap::from([("x".to_string(), 0.0), ("y".to_string(), 15.393806879686704)]),
            HashMap::from([("x".to_string(), 1.0), ("y".to_string(), 19.980723972406757)]),
            HashMap::from([("x".to_string(), 2.0), ("y".to_string(), 46.44040802736543)]),
            HashMap::from([("x".to_string(), 3.0), ("y".to_string(), 76.32570640372656)]),
            HashMap::from([("x".to_string(), 4.0), ("y".to_string(), 28.344936970432833)]),
            HashMap::from([("x".to_string(), 5.0), ("y".to_string(), 107.80487596755955)]),
            HashMap::from([("x".to_string(), 6.0), ("y".to_string(), 90.52490037859376)]),
            HashMap::from([("x".to_string(), 7.0), ("y".to_string(), 157.59858818802704)]),
            HashMap::from([("x".to_string(), 8.0), ("y".to_string(), 143.33624805335427)]),
            HashMap::from([("x".to_string(), 9.0), ("y".to_string(), 145.24993288695646)]),
            HashMap::from([("x".to_string(), 10.0), ("y".to_string(), 260.1807578980633)]),
            HashMap::from([("x".to_string(), 11.0), ("y".to_string(), 185.66458035427738)]),
            HashMap::from([("x".to_string(), 12.0), ("y".to_string(), 399.47143038541725)]),
            HashMap::from([("x".to_string(), 13.0), ("y".to_string(), 461.637154269764)]),
            HashMap::from([("x".to_string(), 14.0), ("y".to_string(), 224.52939759007862)]),
            HashMap::from([("x".to_string(), 15.0), ("y".to_string(), 435.1803248133029)]),
            HashMap::from([("x".to_string(), 16.0), ("y".to_string(), 624.3116876259189)]),
            HashMap::from([("x".to_string(), 17.0), ("y".to_string(), 453.5298507352485)]),
            HashMap::from([("x".to_string(), 18.0), ("y".to_string(), 396.33513809585935)]),
            HashMap::from([("x".to_string(), 19.0), ("y".to_string(), 415.8142609595538)]),
            HashMap::from([("x".to_string(), 20.0), ("y".to_string(), 758.0144333664495)]),
        ];

        let target = String::from("y");
        let model = Evolution::new(target, &parabola_data, 10000, 10, 3);
    }

}
