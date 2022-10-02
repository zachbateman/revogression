use std::collections::{HashSet, HashMap};

/// A RevoData is a more efficient way of storing a list of key: value data
/// Rather than Vec<HashMap<String, f32>> which duplicates key Strings for each row...
/// This structure uses &str references pointing to Strings also in this RevoData
/// resulting in less memory use and hopefully faster performance.
struct RevoData {
    parameters: HashSet<String>,
    data: Vec<HashMap<&str, f32>>,  // each &str points to a String in "parameters"
}

impl RevoData {
    fn new(&mut self, data: &Vec<HashMap<&str, f32>>) -> RevoData {
        for key in data[0].keys() {
            self.parameters.insert(key);
        }

        let mut key_map = HashMap::new();
        for key in self.parameters {
            key_map.insert(&key, &key);
        }

        let mut revo_data = Vector::new();

        for row in data {
            let mut revo_row = HashMap::new();
            for (key, value) in row {
                revo_row.insert(
                    key_map.get(key)
                );
            }
        }


    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_from_data() {
        let input_data = vec![
            HashMap::from([("width", 2.1245), ("height", 0.52412)]),
            HashMap::from([("width", 8.2352), ("height", 1.82521)]),
            HashMap::from([("width", 4.8185), ("height", 3.88152)]),
        ];

        let data = RevoData::new(&input_data);
        assert_eq!(true, true);
    }



}
