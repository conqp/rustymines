use std::collections::HashSet;
use std::hash::Hash;

pub trait PopSet<T> {
    fn pop(&mut self) -> Option<T>;
}

impl<T> PopSet<T> for HashSet<T>
where
    T: Clone + Eq + Hash,
{
    fn pop(&mut self) -> Option<T> {
        self.iter()
            .next()
            .cloned()
            .and_then(|starting_point| self.take(&starting_point))
    }
}
