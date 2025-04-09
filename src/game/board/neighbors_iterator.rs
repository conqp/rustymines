use std::collections::HashSet;

use grid2d::{Coordinate, Grid};

use super::field::Field;
use crate::pop_set::PopSet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SafeNeighbors<'grid> {
    fields: &'grid Grid<Field>,
    starting_points: HashSet<Coordinate>,
    candidates: Vec<(Coordinate, &'grid Field)>,
    neighbors: Vec<Coordinate>,
    processed: HashSet<Coordinate>,
}

impl<'grid> SafeNeighbors<'grid> {
    pub fn new(fields: &'grid Grid<Field>, start: Coordinate) -> Self {
        Self {
            fields,
            starting_points: HashSet::from([start]),
            candidates: Vec::new(),
            neighbors: Vec::new(),
            processed: HashSet::new(),
        }
    }
}

impl Iterator for SafeNeighbors<'_> {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(neighbor) = self.neighbors.pop() {
            self.starting_points.insert(neighbor);
            self.processed.insert(neighbor);
            return Some(neighbor);
        }

        while let Some(starting_point) = self.starting_points.pop() {
            self.candidates.clear();
            self.candidates
                .extend(self.fields.neighbors(starting_point));

            if self
                .candidates
                .iter()
                .all(|(_, neighbor)| !neighbor.has_mine() && !neighbor.is_flagged())
            {
                self.neighbors.extend(
                    self.candidates
                        .drain(..)
                        .map(|(neighbor, _)| neighbor)
                        .filter(|coordinate| {
                            !self.processed.contains(coordinate)
                                && !self.starting_points.contains(coordinate)
                        }),
                );
                return self.next();
            }
        }

        None
    }
}
