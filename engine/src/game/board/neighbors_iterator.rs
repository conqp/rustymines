use std::collections::BTreeSet;

use grid2d::{Coordinate, Grid};

use crate::game::board::field::Field;

/// An iterator over neighbors of a coordinate, which are deemed safe to uncover.
#[derive(Debug, Eq, PartialEq)]
pub struct SafeNeighbors<'grid> {
    fields: &'grid mut Grid<Field>,
    starting_points: BTreeSet<(usize, usize)>,
    processed: BTreeSet<(usize, usize)>,
}

impl<'grid> SafeNeighbors<'grid> {
    /// Create a new [`SafeNeighbors`] iterator from the given [`Grid`] and starting coordinate.
    #[must_use]
    pub fn new(fields: &'grid mut Grid<Field>, start: Coordinate) -> Self {
        Self {
            fields,
            starting_points: BTreeSet::from([start.into()]),
            processed: BTreeSet::new(),
        }
    }

    /// Returns the next reference to a safe neighboring field.
    pub fn next(&mut self) -> Option<&mut Field> {
        let starting_point = self.starting_points.pop_first()?;
        self.processed.insert(starting_point);

        if self.fields.get(starting_point)?.adjacent_mines() == 0 {
            self.starting_points.extend(
                self.fields
                    .neighbors(starting_point)
                    .map(|(coordinate, _)| coordinate.into())
                    .filter(|starting_point| !self.processed.contains(starting_point)),
            );
        }

        self.fields.get_mut(starting_point)
    }
}
