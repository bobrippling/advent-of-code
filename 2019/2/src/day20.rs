mod d2;
use d2::Coord;

mod grid;
use grid::Grid;

type GResult<T> = Result<T, Box<dyn std::error::Error>>;

struct Map {
}

impl Map {
    fn read(path) -> GResult<Self> {
        let mut grid = Grid::new();

        for (y, line) in fs::read_to_string(path)?
            .trim_end()
            .split('\n')
            .enumerate()
        {
            for (x, ch) in line.split().enumerate() {
                let coord = Coord { x, y };
                grid.map.insert(coord, ch);
            }
        }

        let (min, max) = grid.minmax();

        for y in min.y..max.y {
            for x in min.x..max.x {
                let coord = Coord { x, y };
                let ch = match grid.map.get(&coord) {
                    Some(x) => x,
                    None => continue,
                };

                match ch {
                    '#' => {},
                    x if x.is_ascii_letter() => {

                    },
                    _ => panic!("unknown entry '{}'", ch),
                };
            }
        }
    }

fn part1() -> GResult<()> {
    let map = Map::read("./input-day20")?;

}

fn main() -> GResult<()> {
    part1()
}
