#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}

impl Coord {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self::new(0, 0)
    }
}

#[derive(Clone, Copy)]
enum Compass {
    North,
    South,
    West,
    East,
}

impl std::ops::Add<Compass> for Coord {
    type Output = Self;
    fn add(mut self, rhs: Compass) -> Self {
        self += rhs;
        self
    }
}

impl std::ops::AddAssign<Compass> for Coord {
    fn add_assign(&mut self, rhs: Compass) {
        *self = match rhs {
            Compass::North => Coord::new(self.x, self.y - 1),
            Compass::South => Coord::new(self.x, self.y + 1),
            Compass::West => Coord::new(self.x - 1, self.y),
            Compass::East => Coord::new(self.x + 1, self.y),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_compass() {
        let coord = Coord { x: 5, y: 2 };

        assert_eq!(
            coord + Compass::North,
            Coord { x: 5, y: 1 });

        assert_eq!(
            coord + Compass::South,
            Coord { x: 5, y: 3 });

        assert_eq!(
            coord + Compass::East,
            Coord { x: 6, y: 2 });

        assert_eq!(
            coord + Compass::West,
            Coord { x: 4, y: 2 });

        let mut coord = coord;

        coord += Compass::North;
        assert_eq!(coord, Coord { x: 5, y: 1 });

        coord += Compass::South;
        assert_eq!(coord, Coord { x: 5, y: 2 });

        coord += Compass::East;
        assert_eq!(coord, Coord { x: 6, y: 2 });

        coord += Compass::West;
        assert_eq!(coord, Coord { x: 5, y: 2 });
    }
}
