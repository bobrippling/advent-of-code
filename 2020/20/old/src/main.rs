use std::fs;
use std::ops::Range;
use std::cell::RefCell;

use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::HashSet;

const DEBUG_MATCH: bool = false;
const DEBUG: bool = true;

fn main() {
    let s = fs::read_to_string("./input.txt").unwrap();

    let tiles = parse(&s);
    println!("Part 1: {}", part1(&tiles));
}

struct Tile {
    grid: HashMap<Pos, Square>,
    neighbours: Neighbours,
}

struct Neighbours(HashMap<Neighbour, HashMap<usize, Flip>>);

#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
enum Neighbour {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
enum Flip {
    Normal,
    Flipped,
    Both,
}

impl Neighbours {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn has_top(&self) -> bool { self.has(Neighbour::Top) }
    fn has_bottom(&self) -> bool { self.has(Neighbour::Bottom) }
    fn has_left(&self) -> bool { self.has(Neighbour::Left) }
    fn has_right(&self) -> bool { self.has(Neighbour::Right) }

    fn has(&self, n: Neighbour) -> bool {
        match self.0.get(&n) {
            Some(v) => !v.is_empty(),
            None => false,
        }
    }
}

impl std::fmt::Debug for Neighbours {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let mut maybe_write = |desc, n| {
            if let Some(v) = self.0.get(&n) {
                if !v.is_empty() {
                    write!(fmt, "{}: {:?}, ", desc, v)?
                }
            }
            Ok(())
        };

        maybe_write("top", Neighbour::Top)?;
        maybe_write("bottom", Neighbour::Bottom)?;
        maybe_write("left", Neighbour::Left)?;
        maybe_write("right", Neighbour::Right)?;

        Ok(())
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Square {
    Full,
    Empty,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
struct Pos {
    x: usize,
    y: usize,
}

fn parse(s: &str) -> HashMap<usize, RefCell<Tile>> {
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
        let id = t[5..t.len()-1].parse().unwrap();
        let lines = lines[i + 1..i+11].iter().cloned();

        m.insert(id, RefCell::new(parse_tile(lines)));

        i += 12;
        // assert empty lines[next]
    }
    m
}

fn parse_tile<'a>(lines: impl Iterator<Item = &'a str>) -> Tile {
    let mut m = HashMap::new();
    lines
        .enumerate()
        .for_each(|(y, line)| {
            line.chars()
                .map(|c| match c{
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
    Tile { grid: m, neighbours: Neighbours::new() }
}

trait Odd {
    fn is_odd(self) -> bool;
}

impl Odd for usize {
    fn is_odd(self) -> bool {
        self % 2 == 1
    }
}

fn part1(tiles: &HashMap<usize, RefCell<Tile>>) -> usize {
    let keys = tiles.keys().cloned().collect::<Vec<_>>();

    for &t in &keys {
        for &t2 in &keys {
            if t == t2 { continue }
            let mut tile = tiles[&t].borrow_mut();
            let mut tile2 = tiles[&t2].borrow_mut();

            if DEBUG_MATCH {
                println!("{} neighbour to {}?", t, t2);
            }

            calc_neighbours((t, &mut *tile), (t2, &mut *tile2));

            // if tile.neighbours.is_full() {
            //     break;
            // }
        }
    }

    // try to find all 4 corners
    let corners = tiles.iter()
        .inspect(|&(&k, tile)| {
            if DEBUG {
                dump_id_and_tile(None, k, &tile.borrow());
            }
        })
        .filter(|(_, tile)| {
            tile.borrow().neighbours.is_corner()
        })
        .collect::<Vec<_>>();

    println!();
    let edges = tiles.iter()
        .filter(|(_, tile)| tile.borrow().neighbours.is_edge());
    edges
        .for_each(|(&id, tile)| {
            dump_id_and_tile(Some("edge"), id, &tile.borrow());
        });

    println!();
    corners
        .iter()
        .for_each(|&(&k, tile)| {
            dump_id_and_tile(Some("corner"), k, &tile.borrow());
            //println!("corner {}: {:?}", id, tiles[&id].borrow().neighbours);
        });

    // println!();
    // let mut candidates = HashSet::new();
    // for tile in tiles.values() {
    //     let tile = tile.borrow();
    //     let mut odd_counts = HashMap::<usize, usize>::new();
    //     tile.neighbours.0
    //         .values()
    //         .flatten()
    //         .map(|(&id, _flip)| id)
    //         .for_each(|id| {
    //             let p = odd_counts.entry(id)
    //                 .or_insert(0);
    //             *p += 1;
    //         });
    //     for (id, n) in odd_counts {
    //         if n.is_odd() {
    //             candidates.insert(id);
    //         }
    //     }
    // }
    // println!();
    // let candidates = candidates.difference(&corner_ids).collect::<HashSet<_>>();
    // for id in &candidates {
    //     println!("candidate {}", id);
    // }

    // println!();
    // println!("{} candidates", candidates.len());

    //println!("hunting...");
    //for (&id, tile) in tiles {
    //    let tile = tile.borrow();
    //    if tile.neighbours(Neighbour::Top).len() == 0 &&
    //        tile.neighbours(Neighbour::Left).len() == 0 {
    //        //for below_id in tile.neighbours(Neighbour::Bottom) {
    //        //    // if it can't go on top of any tile, it must go here
    //        //    let n_above = count_relative_tiles_in_dir(tiles, below_id, Neighbour::Top);
    //        //    if n_above == 0 {
    //                println!("Top left: {:?}", id);
    //        //    }
    //        //}
    //    }
    //    if tile.neighbours(Neighbour::Bottom).len() == 0 && tile.neighbours(Neighbour::Left).len() == 0 {
    //        println!("Bottom left: {:?}", id);
    //    }
    //    if tile.neighbours(Neighbour::Top).len() == 0 && tile.neighbours(Neighbour::Right).len() == 0 {
    //        println!("Top right: {:?}", id);
    //    }
    //    if tile.neighbours(Neighbour::Bottom).len() == 0 && tile.neighbours(Neighbour::Right).len() == 0 {
    //        println!("Bottom right: {:?}", id);
    //    }
    //}

    panic!();

    // corners
    //     .iter()
    //     .map(|&(&k, ..)| k)
    //     .product()
}

fn count_relative_tiles_in_dir(
    tiles: &HashMap<usize, RefCell<Tile>>,
    from_id: usize,
    dir: Neighbour,
) -> usize {
    tiles.iter()
        .filter(|&(&this_id, this_tile)| {
            let this_tile = this_tile.borrow();

            if this_id == from_id {
                return false;
            }

            let could_attach = this_tile.neighbours(dir)
                .into_iter()
                .find(|&id| id == from_id)
                .is_some();

            !could_attach
        })
        .count()
}

fn dump_id_and_tile(pre: Option<&str>, id: usize, tile: &Tile) {
    match pre {
        Some(pre) => println!("{} tile[{}]:", pre, id),
        _ => println!("tile[{}]:", id),
    };
    let neighbours = &tile.neighbours;
    for (n, vec) in &neighbours.0 {
        println!("    {:?}: {:?}", n, vec);
    }
}

static RANGES: [(Neighbour, Range<Pos>); 4] = [
        (Neighbour::Top, Pos { x: 0, y: 0 }..Pos { x: 9, y: 0 }),
        (Neighbour::Bottom, Pos { x: 0, y: 9 }..Pos { x: 9, y: 9 }),
        (Neighbour::Left, Pos { x: 0, y: 0 }..Pos { x: 0, y: 9 }),
        (Neighbour::Right, Pos { x: 9, y: 0 }..Pos { x: 9, y: 9 }),
    ];

fn calc_neighbours(
    (id_a, tile_a): (usize, &mut Tile),
    (id_b, tile_b): (usize, &mut Tile),
) {
    let revs = [
        (false, false),
        (false, true),
        (true, false),
        (true, true),
    ];

    for (n_a, range_a) in &RANGES {
        for (n_b, range_b) in &RANGES {
            for &(rev_a, rev_b) in &revs {
                if tile_a.matches_across(
                    tile_b,
                    range_a, range_b,
                    rev_a, rev_b,
                ) {
                    tile_a.add_neighbour(*n_a, rev_a, id_b);
                    tile_b.add_neighbour(*n_b, rev_b, id_a);
                }
            }
        }
    }
}

struct R<'a> {
    range: &'a Range<Pos>,
    diff: Pos,
    cur: Pos,
    rev: bool,
}

impl<'a> R<'a> {
    fn new(range: &'a Range<Pos>, rev: bool) -> Self {
        let diff = Pos {
            x: if range.start.x < range.end.x { 1 } else { 0 },
            y: if range.start.y < range.end.y { 1 } else { 0 },
        };
        R {
            range,
            diff,
            cur: if rev { range.end } else { range.start },
            rev,
        }
    }
}

impl<'a> Iterator for R<'a> {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rev {
            if self.cur == self.range.start {
                return None;
            }

            let r = self.cur;
            self.cur.x -= self.diff.x;
            self.cur.y -= self.diff.y;
            Some(r)
        } else {
            if self.cur == self.range.end {
                return None;
            }

            let r = self.cur;
            self.cur.x += self.diff.x;
            self.cur.y += self.diff.y;
            Some(r)
        }
    }
}

impl<'a> DoubleEndedIterator for R<'a> {
    fn next_back(&mut self) -> Option<<Self as Iterator>::Item> {
        todo!()
    }
}

impl Tile {
    fn matches_across(
        &self, other: &Self,
        range_a: &Range<Pos>, range_b: &Range<Pos>,
        rev_a: bool, rev_b: bool,
    ) -> bool {
        if DEBUG_MATCH {
            println!("  trying to match, {:?} / {:?}", range_a, range_b);
        }

        let iter_a = R::new(range_a, rev_a);
        let iter_b = R::new(range_b, rev_b);

        /*
        if rev_a {
            iter_a = iter_a.rev();
        }
        if rev_b {
            iter_b = iter_b.rev();
        }
        */

        for (pos_a, pos_b) in iter_a.zip(iter_b) {
            let my_sq = &self.grid[&pos_a];
            let o_sq = &other.grid[&pos_b];

            if DEBUG_MATCH {
                println!("    {:?} / {:?}: {:?} vs {:?}", pos_a, pos_b, my_sq, o_sq);
            }
            if my_sq != o_sq {
                return false;
            }
        }

        true
    }

    fn add_neighbour(&mut self, n: Neighbour, flipped: bool, id: usize) {
        let set = self
            .neighbours.0
            .entry(n)
            .or_insert_with(Default::default);

        if let Some(flip) = set.get(&id) {
            match flip {
                Flip::Normal => {
                    if flipped {
                        set.insert(id, Flip::Both);
                    }
                }
                Flip::Flipped => {
                    if !flipped {
                        set.insert(id, Flip::Both);
                    }
                }
                _ => {}
            }
        } else {
            set.insert(id, if flipped { Flip::Flipped } else { Flip::Normal });
        }
    }

    // fn edge_as_num(&self, n: Neighbour) -> u8 {
    //     let range = RANGES
    //         .iter()
    //         .find(|&&(n2, _)| n == n2)
    //         .map(|(_, r)| r)
    //         .unwrap();

    //     for pos in range {
    //     }
    // }

    fn neighbours(&self, n: Neighbour) -> Vec<usize> {
        match self.neighbours.0.get(&n) {
            Some(v) => v.keys().cloned().collect(),
            None => vec![],
        }
    }
}

impl Neighbours {
    // fn is_full(&self) -> bool {
    //     self.has_top() &&
    //     self.has_bottom() &&
    //     self.has_left() &&
    //     self.has_right()
    // }

    fn is_corner(&self) -> bool {
        let top = self.has_top();
        let left = self.has_left();
        let bottom = self.has_bottom();
        let right = self.has_right();

        (top as u8 +
            bottom as u8 +
            left as u8 +
            right as u8) == 2
    }

    fn is_edge(&self) -> bool {
        let top = self.has_top();
        let left = self.has_left();
        let bottom = self.has_bottom();
        let right = self.has_right();

        (top as u8 +
            bottom as u8 +
            left as u8 +
            right as u8) == 3
    }

    /*
    fn merge(&mut self, from: &Neighbours) {
        if let Some(top) = from.top() {
            self.0.insert(Neighbour::Top, top);
        }
        if let Some(bottom) = from.bottom() {
            self.bottom.get_or_insert(bottom);
        }
        if let Some(left) = from.left() {
            self.left.get_or_insert(left);
        }
        if let Some(right) = from.right() {
            self.right.get_or_insert(right);
        }
    }
    */
}

#[test]
fn test_part1() {
    /*
    By rotating, flipping, and rearranging them, you can find a square arrangement that causes all adjacent borders to line up:

    #...##.#.. ..###..### #.#.#####.
    ..#.#..#.# ###...#.#. .#..######
    .###....#. ..#....#.. ..#.......
    ###.##.##. .#.#.#..## ######....
    .###.##### ##...#.### ####.#..#.
    .##.#....# ##.##.###. .#...#.##.
    #...###### ####.#...# #.#####.##
    .....#..## #...##..#. ..#.###...
    #.####...# ##..#..... ..#.......
    #.##...##. ..##.#..#. ..#.###...

    #.##...##. ..##.#..#. ..#.###...
    ##..#.##.. ..#..###.# ##.##....#
    ##.####... .#.####.#. ..#.###..#
    ####.#.#.. ...#.##### ###.#..###
    .#.####... ...##..##. .######.##
    .##..##.#. ....#...## #.#.#.#...
    ....#..#.# #.#.#.##.# #.###.###.
    ..#.#..... .#.##.#..# #.###.##..
    ####.#.... .#..#.##.. .######...
    ...#.#.#.# ###.##.#.. .##...####

    ...#.#.#.# ###.##.#.. .##...####
    ..#.#.###. ..##.##.## #..#.##..#
    ..####.### ##.#...##. .#.#..#.##
    #..#.#..#. ...#.#.#.. .####.###.
    .#..####.# #..#.#.#.# ####.###..
    .#####..## #####...#. .##....##.
    ##.##..#.. ..#...#... .####...#.
    #.#.###... .##..##... .####.##.#
    #...###... ..##...#.. ...#..####
    ..#.#....# ##.#.#.... ...##.....

    For reference, the IDs of the above tiles are:

    1951    2311    3079
    2729    1427    2473
    2971    1489    1171

    */
    let s = fs::read_to_string("./eg.txt").unwrap();

    let mut tiles = parse(&s);
    assert_eq!(part1(&mut tiles), 20899048083289);
}
