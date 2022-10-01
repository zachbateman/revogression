use std::collections::HashMap;
use crate::standardize::Standardizer;


pub struct Evolution<'a> {
    target: String,
    data: &'a Vec<HashMap<&'a str, f32>>,
    standardized_data: Vec<HashMap<String, f32>>,
    num_creatures: u32,
    num_cycles: u16,
    standardizer: Standardizer<'a>,
}

impl<'a> Evolution<'a> {
    fn new(
        target: String,
        data: &'a Vec<HashMap<&'a str, f32>>,
        num_creatures: u32,
        num_cycles: u16,
        max_layers: u8,
    ) -> Evolution<'a> {

        let standardizer = Standardizer::new(&data[..]);
        let standardized_data = standardizer.standardized_values(&data);

        Evolution {
            target: target,
            data: data,
            standardized_data: standardized_data,
            num_creatures: num_creatures,
            num_cycles: num_cycles,
            standardizer: standardizer,
        }
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_test() {
        let target = "target_param";
        let data = vec![
            HashMap::from([("target_param", 5.2), ("p2", 7.8), ("p3", 8.3)]),
            HashMap::from([("target_param", 6.0), ("p2", 4.4), ("p3", 8.1)]),
            HashMap::from([("target_param", 7.1), ("p2", 3.9), ("p3", 9.5)]),
            HashMap::from([("target_param", 8.6), ("p2", 2.7), ("p3", 11.6)]),
            HashMap::from([("target_param", 9.4), ("p2", -2.6), ("p3", 13.0)]),
        ];

        let evo = Evolution::new(target.into(), &data, 1000, 10, 3);
        assert_eq!(evo.num_creatures == 1000, true);
    }
}
