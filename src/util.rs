

/// CAUTION!  Not sure if even need many data checks...
/// Needed them in Python, but Rust will ensure no
/// improper types get used at runtime... TBD on this.


fn fill_none_with_median<T>(data: &[T]) -> &[T] {
    todo!();
}

/// Check cleaned input data for potentail issues.
/// At point when this is called on cleaned data,
/// there should be no issues.
/// If this method panics, need to add more data-
/// cleaning checks/capabilities to handle the issue!
fn data_checks<T>(data: &[T]) -> () {
    println!("Data looks clean!");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_test() {
        assert_eq!(true, true);
    }
}
