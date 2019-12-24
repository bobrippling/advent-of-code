use std::collections::HashMap;

use crate::d2::Coord;

#[derive(Default, Debug)]
pub struct Grid<T> {
    pub map: HashMap<Coord, T>,
    pub default: Option<T>,
}

impl<T> Grid<T> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            default: None,
        }
    }

    pub fn new_default(default: T) -> Self {
        Self {
            map: HashMap::new(),
            default: Some(default),
        }
    }

    pub fn get_default<'s, 'c>(&'s self, c: &'c Coord) -> &'s T {
        self.map
            .get(c)
            .unwrap_or_else(|| self.default.as_ref().unwrap())
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

/*
impl<T> PartialEq for Grid<T> {
    fn eq(&self, rhs: &Self) -> bool {
    }
}
*/
