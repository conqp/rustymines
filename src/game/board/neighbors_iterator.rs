use crate::game::board::field::Field;
use grid2d::{Coordinate, Grid};
use std::collections::HashSet;

pub struct NeighborsIterator<'grid> {
    fields: &'grid Grid<Field>,
    processed: HashSet<Coordinate>,
    unprocessed: Vec<Coordinate>,
    index: usize,
}

impl<'grid> NeighborsIterator<'grid> {
    pub fn new(fields: &'grid Grid<Field>, start: Coordinate) -> Self {
        Self {
            fields,
            processed: HashSet::new(),
            unprocessed: Vec::from([start]),
            index: 0,
        }
    }
}

impl<'grid> Iterator for NeighborsIterator<'grid> {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(&coordinate) = self.unprocessed.get(self.index) {
            self.index += 1;
            return Some(coordinate);
        }

        self.unprocessed = self
            .unprocessed
            .iter()
            .filter(|&coordinate| {
                self.fields
                    .neighbors(coordinate)
                    .filter(|(coordinate, _)| !self.processed.contains(coordinate))
                    .all(|(_, field)| !field.has_mine())
            })
            .flat_map(|&coordinate| {
                self.fields
                    .neighbors(coordinate)
                    .filter(|(coordinate, neighbor)| {
                        !self.processed.contains(coordinate)
                            && !neighbor.has_mine()
                            && !neighbor.is_flagged()
                    })
                    .map(|(coordinate, _)| coordinate)
            })
            .collect();

        self.processed.extend(&self.unprocessed);

        if let Some(coordinate) = self.unprocessed.pop() {
            self.index = 0;
            return Some(coordinate);
        }

        None
    }
}
