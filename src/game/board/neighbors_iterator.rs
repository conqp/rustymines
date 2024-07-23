use crate::game::board::field::Field;
use grid2d::{Coordinate, Grid};
use itertools::Itertools;
use std::collections::HashSet;

pub struct SafeNeighbors<'grid> {
    fields: &'grid Grid<Field>,
    processed: HashSet<Coordinate>,
    unprocessed: Vec<Coordinate>,
    index: usize,
}

impl<'grid> SafeNeighbors<'grid> {
    pub fn new(fields: &'grid Grid<Field>, start: Coordinate) -> Self {
        Self {
            fields,
            processed: HashSet::new(),
            unprocessed: Vec::from([start]),
            index: 0,
        }
    }
}

impl Iterator for SafeNeighbors<'_> {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(&coordinate) = self.unprocessed.get(self.index) {
            self.index += 1;
            return Some(coordinate);
        }

        self.unprocessed = self
            .unprocessed
            .iter()
            .flat_map(|&coordinate| {
                let neighbors = self.fields.neighbors(coordinate).collect_vec();
                if neighbors
                    .iter()
                    .all(|(_, neighbor)| !neighbor.has_mine() && !neighbor.is_flagged())
                {
                    neighbors
                } else {
                    Vec::new()
                }
            })
            .map(|(coordinate, _)| coordinate)
            .filter(|coordinate| !self.processed.contains(coordinate))
            .collect();

        self.processed.extend(&self.unprocessed);

        if let Some(&coordinate) = self.unprocessed.first() {
            self.index = 1;
            return Some(coordinate);
        }

        None
    }
}
