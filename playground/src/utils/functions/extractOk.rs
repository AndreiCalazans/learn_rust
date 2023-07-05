pub fn removeErrorsFromVector(some_vector: Vec<Result<Option<i32>, &str>>) -> Vec<i32> {
    some_vector.into_iter().filter_map(|item| {
        match item {
            Ok(Some(field)) => Some(field),
            _ => None
        }
    }).collect()
}
