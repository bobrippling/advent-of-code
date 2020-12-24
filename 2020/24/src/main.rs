use std::fs;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = fs::read_to_string("./input.txt")?;

    let mut grid = s.parse()?;
    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&mut grid));

    Ok(())
}

struct Grid(HashMap<Pos, Tile>);

impl Grid {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn insert_or_flip(&mut self, pos: &Pos) {
        if let Some(tile) = self.get_mut(pos) {
            tile.flip();
        } else {
            self.0.insert(*pos, Tile::Black);
        }
    }

    fn get_mut(&mut self, pos: &Pos) -> Option<&mut Tile> {
        self.0.get_mut(pos)
    }

    fn tiles(&self) -> impl Iterator<Item = &Tile> {
        self.0.values()
    }

    fn should_flip(&self, pos: &Pos) -> bool {
        let adj = self.count_adj(pos, Tile::Black);

        match self.0.get(pos).unwrap_or(&Tile::White) {
            Tile::Black => adj == 0 || adj > 2,
            Tile::White => adj == 2,
        }
    }

    fn part2_step(&mut self) {
        let mut new = HashMap::new();

        // this could just look for each black tile and gain a slight boost
        for pos in self.iter_positions() {
            let (tile, present) = self.0
                .get(&pos)
                .map(|t| (t, true))
                .unwrap_or((&Tile::White, false));

            let flip = self.should_flip(&pos);

            if present && cfg!(debug) {
                println!("initial pass, {:?} is {:?}. flip={}", pos, tile, flip);
            }

            let mut newtile = tile.clone();
            if flip {
                newtile.flip();

                if cfg!(debug) {
                    let adj = self.count_adj(&pos, Tile::Black);
                    println!(
                        "{}{:?} tile at {} with {} adjacent black tiles:",
                        if present { "" } else { "Implicit " },
                        tile,
                        pos,
                        adj,
                        );

                    if adj > 0 {
                        println!(
                            "  {}",
                            pos
                            .neighbours()
                            .filter(|p| self.0.get(p) == Some(&Tile::Black))
                            .map(|p| format!("{} ", p))
                            .collect::<String>(),
                        );
                    }
                }
            }

            if newtile == Tile::Black {
                new.insert(pos, newtile);
            }
        }

        self.0 = new;
    }

    fn count_tiles(&self, target: Tile) -> usize {
        self.tiles()
            .filter(|&t| *t == target)
            .count()
    }

    fn count_adj(&self, pos: &Pos, of: Tile) -> usize {
        self.adj(pos)
            .cloned()
            .filter(|t| *t == of)
            .count()
    }

    fn adj<'s>(&'s self, pos: &Pos) -> impl Iterator<Item = &'s Tile> + 's {
        pos.neighbours()
            .map(move |ref p| self.0.get(p).unwrap_or(&Tile::White))
    }

    fn start_end(&self) -> ((isize, isize), (isize, isize)) {
        (
            (
                self.0.keys().map(|&Pos { x, .. }| x).min().unwrap(),
                self.0.keys().map(|&Pos { x, .. }| x).max().unwrap(),
            ),
            (
                self.0.keys().map(|&Pos { y, .. }| y).min().unwrap(),
                self.0.keys().map(|&Pos { y, .. }| y).max().unwrap(),
            ),
        )
    }

    fn iter_positions(&self) -> impl Iterator<Item = Pos> {
        let ((mut start_x, mut end_x), (mut start_y, mut end_y)) = self.start_end();

        // look beyond existing grid for more
        start_x -= 2;
        end_x += 2;

        start_y -= 1;
        end_y += 1;

        (start_y..=end_y)
            .flat_map(move |y| {
                // if we're at an odd-y, or an odd-x on even-y,
                // bump by 1 for initial alignment to the hex grid
                let x_off = if y.is_odd() != start_x.is_odd() { 1 } else { 0 };

                (start_x..=end_x)
                    .step_by(2)
                    .map(move |x| Pos::at(x - x_off, y))
            })
    }
}

impl std::fmt::Debug for Grid {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let ((start_x, end_x), (start_y, end_y)) = self.start_end();

        for y in start_y..=end_y {
            for x in start_x..=end_x {
                let pos = Pos::try_at(x, y);
                let tile = pos.and_then(|p| self.0.get(&p));

                write!(
                    fmt,
                    "{} ",
                    tile
                        .map(Tile::to_char)
                        .unwrap_or(if pos.is_some() { '.' } else { ' ' }),
                )?;
            }
            write!(fmt, "    y={}", y)?;
            writeln!(fmt)?;
        }

        for x in start_x..=end_x {
            match x {
                0 => write!(fmt, " 0"),
                x if 0 <= x && x < 10 => write!(fmt, "+{}", x),
                x if -10 < x && x < 0 => write!(fmt, "{}", x),
                _ => write!(fmt, "  ")
            }?;
        }

        Ok(())
    }
}

trait IsEven {
    fn is_even(&self) -> bool;

    fn is_odd(&self) -> bool {
        !self.is_even()
    }
}

impl IsEven for isize {
    fn is_even(&self) -> bool { self % 2 == 0 }
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum Tile {
    Black,
    White,
}

impl Tile {
    fn flip(&mut self) {
        *self = self.flipped();
    }

    fn flipped(&self) -> Self {
        match self {
            Tile::White => Tile::Black,
            Tile::Black => Tile::White,
        }
    }

    fn to_char(&self) -> char {
        match self {
            Tile::White => 'w',
            Tile::Black => 'b',
        }
    }
}

#[derive(Clone)]
enum Dir {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Pos {
    // 1 coords for odd y, double for even y
    y: isize,
    x: isize,
}

impl Pos {
    fn origin() -> Self {
        Pos::at(0, 0)
    }

    fn at(x: isize, y: isize) -> Self {
        Self::try_at(x, y)
            .expect(&format!("tried to create x={}, y={}", x, y))
    }

    fn try_at(x: isize, y: isize) -> Option<Self> {
        (x + y)
            .is_even()
            // if we're on origin row, we're 2 to the left or right
            // if we're on an odd row, we're 2 to the left or right, but offset by one left/right
            .then(|| Pos { x, y })
    }

    fn step(&mut self, dir: Dir) {
        match dir {
            Dir::E => self.x += 2,
            Dir::W => self.x -= 2,

            Dir::SE => {
                self.x += 1;
                self.y += 1;
            }
            Dir::SW => {
                self.x -= 1;
                self.y += 1;
            }

            Dir::NE => {
                self.x += 1;
                self.y -= 1;
            }
            Dir::NW => {
                self.x -= 1;
                self.y -= 1;
            }
        }
    }

    fn stepped(&self, dir: Dir) -> Pos {
        let mut pos = *self;
        pos.step(dir);
        pos
    }

    fn neighbours(self) -> impl Iterator<Item = Pos> {
        [
            Dir::E,
            Dir::SE,
            Dir::SW,
            Dir::W,
            Dir::NW,
            Dir::NE,
        ]
            .iter()
            .cloned()
            .map(move |d| self.stepped(d))
    }
}

impl std::fmt::Display for Pos {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "(x {}, y {})", self.x, self.y)
    }
}

#[test]
fn test_pos() {
    let mut pos = Pos::origin();

    pos.step(Dir::E);
    pos.step(Dir::E);
    pos.step(Dir::SW);

    assert_eq!(
        pos,
        Pos::at(3, 1),
    );

    assert_eq!(
        pos.neighbours().collect::<Vec<_>>(),
        vec![
            Pos::at(5, 1), // E
            Pos::at(4, 2), // NE
            Pos::at(2, 2), // NW
            Pos::at(1, 1), // W
            Pos::at(2, 0), // SW
            Pos::at(4, 0), // SE
        ],
    );

    pos.step(Dir::W);
    pos.step(Dir::NE);

    assert_eq!(pos.stepped(Dir::W), Pos::origin());
}

impl std::str::FromStr for Grid {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Grid, Self::Err> {
        let tile_dirs = s
            .lines()
            .map(|l| {
                let byte_str = l.as_bytes();
                let mut dirs = Vec::new();
                let mut i = 0;

                while i < byte_str.len() {
                    match byte_str[i] {
                        b'e' => dirs.push(Dir::E),
                        b's' => {
                            i += 1;
                            match byte_str[i] {
                                b'e' => dirs.push(Dir::SE),
                                b'w' => dirs.push(Dir::SW),
                                _ => return Err("expected 'e' or 'w' after 's'"),
                            }
                        }
                        b'w' => dirs.push(Dir::W),
                        b'n' => {
                            i += 1;
                            match byte_str[i] {
                                b'e' => dirs.push(Dir::NE),
                                b'w' => dirs.push(Dir::NW),
                                _ => return Err("expected 'e' or 'w' after 'n'"),
                            }
                        }
                        _ => return Err("expected 'n'/'s'/'e'/'w'"),
                    }
                    i += 1;
                }
                Ok(dirs)
            })
            .collect::<Result<Vec<_>, _>>()?;

        let mut grid = Grid::new();
        for tile_dir in tile_dirs {
            let mut pos = Pos::origin();
            for dir in tile_dir {
                pos.step(dir);
            }
            grid.insert_or_flip(&pos);
        }

        Ok(grid)
    }
}

fn part1(grid: &Grid) -> usize {
    grid.count_tiles(Tile::Black)
}

#[test]
fn test_part1() {
    let s = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

    let grid = s.parse().unwrap();
    assert_eq!(part1(&grid), 10);
}

fn part2(grid: &mut Grid) -> usize {
    for _ in 1..=100 {
        grid.part2_step();
    }

    grid.count_tiles(Tile::Black)
}

#[test]
fn test_part2() {
    let s = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

    let mut grid: Grid = s.parse().unwrap();

    let expected = [
		(1, 15),
		(2, 12),
		(3, 25),
		(4, 14),
		(5, 23),
		(6, 28),
		(7, 41),
		(8, 37),
		(9, 49),
		(10, 37),

		(20, 132),
		(30, 259),
		(40, 406),
		(50, 566),
		(60, 788),
		(70, 1106),
		(80, 1373),
		(90, 1844),
		(100, 2208),
    ];

    let mut cur_day = 0;
    for &(day, expected_black) in expected.iter() {
        assert!(cur_day < day);
        while cur_day < day {
            grid.part2_step();
            cur_day += 1;
        }

        assert_eq!(
            grid.count_tiles(Tile::Black),
            expected_black,
        );
    }
}
