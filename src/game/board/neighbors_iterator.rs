use std::collections::BTreeSet;

use grid2d::{Coordinate, Grid};

use super::field::Field;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SafeNeighbors<'grid> {
    fields: &'grid Grid<Field>,
    starting_points: BTreeSet<(usize, usize)>,
    processed: BTreeSet<(usize, usize)>,
}

impl<'grid> SafeNeighbors<'grid> {
    pub fn new(fields: &'grid Grid<Field>, start: Coordinate) -> Self {
        Self {
            fields,
            starting_points: BTreeSet::from([start.into()]),
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
