use std::collections::BTreeSet;
use std::iter::once;

use grid2d::{Coordinate, Grid};

use crate::game::board::field::Field;

/// An iterator over neighbors of a coordinate, which are deemed safe to uncover.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SafeNeighbors<'grid> {
    fields: &'grid Grid<Field>,
    starting_points: BTreeSet<(usize, usize)>,
    processed: BTreeSet<(usize, usize)>,
}

impl<'grid> SafeNeighbors<'grid> {
    /// Create a new [`SafeNeighbors`] iterator from the given [`Grid`] and starting coordinate.
    #[must_use]
    pub fn new(fields: &'grid Grid<Field>, start: Coordinate) -> Self {
        Self {
            fields,
            starting_points: once(start.into()).collect(),
            processed: BTreeSet::new(),
        }
    }
}

impl Iterator for SafeNeighbors<'_> {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        let starting_point = self.starting_points.pop_first()?;

        if self
            .fields
            .neighbors(starting_point)
            .all(|(_, neighbor)| neighbor.is_safe())
        {
            self.starting_points.extend(
                self.fields
                    .neighbors(starting_point)
                    .map(|(coordinate, _)| coordinate.into())
                    .filter(|starting_point| !self.processed.contains(starting_point)),
            );
        }

        self.processed.insert(starting_point);
        Some(starting_point.into())
    }
}
