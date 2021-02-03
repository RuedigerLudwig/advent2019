use std::fmt::Display;

pub fn join<T: Display>(lst: &[T], sep: &str) -> String {
    lst.iter()
        .map(|item| item.to_string())
        .collect::<Vec<_>>()
        .join(sep)
}
