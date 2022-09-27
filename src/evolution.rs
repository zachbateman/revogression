

pub struct Evolution<'a, T> {
    target: &'a str,
    data: Vec<T>,
    standardized_data: Vec<T>,
    num_creatures: u32,
    num_cycles: u32,
}

impl<T> Evolution<'_, T> {
    fn new() -> Evolution<'static, T> {
        todo!();
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_test() {
        assert_eq!(true, true);
    }
}
