use std::fs;
//use std::ops::Range;

use std::cell::RefCell;

use std::collections::HashMap;

fn main() {
    let s = fs::read_to_string("./input.txt").unwrap();

    let tiles = parse(&s);
    let neighbours = calc_neighbours(&tiles);
    println!("Part 1: {}", part1(&neighbours));
    println!("Part 2: {}", part2(&tiles, &neighbours));
}

fn part1(neighbours: &HashMap<TileId, HashMap<TileId, String>>) -> u64 {
    neighbours
        .iter()
        .filter(|(_, neighbours)| neighbours.len() == 2)
        .map(|(&id, _)| id.0 as u64)
        .product()
}

fn part2(
    tiles: &HashMap<TileId, RefCell<Tile>>,
    neighbours: &HashMap<TileId, HashMap<TileId, String>>,
) -> u64 {
    /* sea monster:
     *                   #
     * #    ##    ##    ###
     *  #  #  #  #  #  #
     * ^~~~~~~~~~~~~~~~~~~~
     *
     * Need to:
     * - construct grid
     * - look for pattern
     *   - iterate Pos{} over the grid, testing with hardcoded offsets if we get a match
     * - do the sum
     */
    let grid = construct_grid(tiles, neighbours);

    todo!()
}

fn construct_grid(
    tiles: &HashMap<TileId, RefCell<Tile>>,
    neighbours: &HashMap<TileId, HashMap<TileId, String>>,
) -> HashMap<Pos, Square> {
    let corner = neighbours
        .iter()
        .filter(|(_, neighbours)| neighbours.len() == 2)
        .map(|(&id, _)| id)
        .next()
        .unwrap();

    let mut current = corner;
    let mut direction = None;

    let mut grid = HashMap::new();
    let mut top_left = Pos::default();

    loop {
        loop {
            let tile = tiles[&current].borrow();

            if direction.is_none() {
                // pick a side
                let neighbours = ...;
            }
        }
    }
}

fn calc_neighbours(
    tiles: &HashMap<TileId, RefCell<Tile>>,
) -> HashMap<TileId, HashMap<TileId, String>> {
    let mut neighbours = HashMap::new();

    for (&a, tile_a) in tiles {
        let tile_a = tile_a.borrow();
        let sides_a = tile_a.sides();

        for (&b, tile_b) in tiles {
            if a == b {
                continue;
            }

            let tile_b = tile_b.borrow();
            let sides_b = tile_b.sides();

            for side in sides_b {
                if sides_a.contains(&side) {
                    append_neighbour(&mut neighbours, a, (b, side.to_owned()));
                    append_neighbour(&mut neighbours, b, (a, side.to_owned()));
                }
            }
        }
    }

    neighbours
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct TileId(u32);

struct Tile {
    grid: HashMap<Pos, Square>,
    normal_sides: HashMap<Neighbour, String>,
    reverse_sides: HashMap<Neighbour, String>,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug, Default)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
enum Neighbour {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Square {
    Full,
    Empty,
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "{}", self.border(Neighbour::Top))?;

        let left: Vec<_> = self.border(Neighbour::Left).chars().collect();
        let right: Vec<_> = self.border(Neighbour::Right).chars().collect();

        for y in 1..9 {
            writeln!(fmt, "{}        {}", left[y], right[y])?;
        }

        writeln!(fmt, "{}", self.border(Neighbour::Bottom))?;

        Ok(())
    }
}

fn parse(s: &str) -> HashMap<TileId, RefCell<Tile>> {
    let lines = s.lines().collect::<Vec<_>>();
    let mut i = 0;
    let mut m = HashMap::new();

    loop {
        let t = match lines.get(i) {
            Some(t) => t,
            None => break,
        };
        if !t.starts_with("Tile ") || !t.ends_with(":") {
            panic!("unknown line '{}'", t);
        }
        let id = TileId(t[5..t.len() - 1].parse().unwrap());
        let tile_lines = lines[i + 1..i + 11].iter().cloned();

        m.insert(id, RefCell::new(parse_tile(tile_lines)));

        assert!(
            matches!(lines.get(i + 11), None | Some(&"")),
            "line isn't empty: {:?}",
            lines.get(i + 12),
        );

        i += 12;
    }

    m
}

fn parse_tile<'a>(lines: impl Iterator<Item = &'a str>) -> Tile {
    let mut m = HashMap::new();

    lines.enumerate().for_each(|(y, line)| {
        line.chars()
            .map(|c| match c {
                '#' => Square::Full,
                '.' => Square::Empty,
                _ => panic!(),
            })
            .enumerate()
            .for_each(|(x, s)| {
                let pos = Pos { x, y };
                m.insert(pos, s);
            });
    });

    let border = |n: Neighbour| {
        let mut s = String::new();

        match n {
            Neighbour::Top => {
                for x in 0..10 {
                    s.push(m[&Pos { x, y: 0 }].into());
                }
                s
            }
            Neighbour::Bottom => {
                for x in 0..10 {
                    s.push(m[&Pos { x, y: 9 }].into());
                }
                s
            }
            Neighbour::Left => {
                for y in 0..10 {
                    s.push(m[&Pos { x: 0, y }].into());
                }
                s
            }
            Neighbour::Right => {
                for y in 0..10 {
                    s.push(m[&Pos { x: 9, y }].into());
                }
                s
            }
        }
    };

    let mut normal_sides = HashMap::new();
    let mut reverse_sides = HashMap::new();
    use Neighbour::*;
    for n in [Top, Bottom, Left, Right] {
        normal_sides.insert(n, border(n));
        reverse_sides.insert(n, border(n).reverse());
    }

    Tile {
        grid: m,
        normal_sides,
        reverse_sides,
    }
}

impl Tile {
    fn border(&self, n: Neighbour) -> &str {
        &self.normal_sides[&n]
    }

    fn sides(&self) -> Vec<&str> {
        vec![
            &self.normal_sides[&Neighbour::Top],
            &self.normal_sides[&Neighbour::Bottom],
            &self.normal_sides[&Neighbour::Left],
            &self.normal_sides[&Neighbour::Right],
            &self.reverse_sides[&Neighbour::Top],
            &self.reverse_sides[&Neighbour::Bottom],
            &self.reverse_sides[&Neighbour::Left],
            &self.reverse_sides[&Neighbour::Right],
        ]
    }
}

impl From<Square> for char {
    fn from(sq: Square) -> Self {
        match sq {
            Square::Empty => '.',
            Square::Full => '#',
        }
    }
}

trait Reversible {
    fn reverse(&self) -> String;
}

impl Reversible for str {
    fn reverse(&self) -> String {
        self.chars().rev().collect()
    }
}

fn append_neighbour(
    neighbours: &mut HashMap<TileId, HashMap<TileId, String>>,
    from: TileId,
    to: (TileId, String),
) {
    let ent = neighbours.entry(from).or_insert_with(HashMap::new);

    ent.insert(to.0, to.1);
}
