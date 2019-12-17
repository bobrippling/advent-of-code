use std::collections::HashMap;

mod d2;
use d2::Coord;

#[derive(Default)]
pub struct Grid<T> {
    pub map: HashMap<Coord, T>,
}

impl<T> Grid<T> {
    fn minmax(&self) -> (Coord, Coord) {
        self
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
