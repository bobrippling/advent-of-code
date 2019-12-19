use std::collections::HashMap;

use crate::d2::Coord;

#[derive(Default)]
pub struct Grid<T> {
    pub map: HashMap<Coord, T>,
}

impl<T> Grid<T> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn minmax(&self) -> (Coord, Coord) {
        self
            .map
            .keys()
            .fold(
                (Coord::zero(), Coord::zero()),
                |(min, max), Coord { x, y }| (
                    Coord {
                        x: *x.min(&min.x),
                        y: *y.min(&min.y),
                    },
                    Coord {
                        x: *x.max(&max.x),
                        y: *y.max(&max.y),
                    },
                ))
    }
}
