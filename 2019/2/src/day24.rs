use std::fs;

use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

mod d2;
use d2::{Coord, Compass};

mod grid;
use grid::Grid;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Tile {
	Empty,
	Bug,
}

fn parse(s: &str) -> Grid<Tile> {
	let mut grid = Grid::new_default(Tile::Empty);

	s
		.split('\n')
        .filter(|s| !s.is_empty())
		.map(|line| {
			line
				.trim()
				.chars()
				.map(|ch| {
                    //println!("char: {}", ch);
                    match ch {
                        '#' => Tile::Bug,
                        '.' => Tile::Empty,
                        _ => panic!("unknown ch '{}'", ch),
                    }
                })
		})
        .enumerate()
		.for_each(|(y, cells)| {
			cells
				.enumerate()
				.for_each(|(x, cell)| {
                    //println!("char: {},{} = {:?}", x, y, cell);
					let c = Coord { x: x as _, y: y as _ };
					grid.map.insert(c, cell);

                    assert_eq!(grid.get_default(&c), &cell);
				})
		});

	grid
}

fn surroundings(g: &Grid<Tile>, c: &Coord) -> Vec<Tile> { //impl Iterator<Item = Tile>
    vec![
        *g.get_default(&(*c + Compass::North)),
        *g.get_default(&(*c + Compass::South)),
        *g.get_default(&(*c + Compass::East)),
        *g.get_default(&(*c + Compass::West)),
    ]
}

fn iterate(g: &mut Grid<Tile>) {
    let (min, max) = g.minmax();
    let mut next = Grid::<Tile>::new_default(g.default.unwrap());

    for y in min.y..=max.y {
        for x in min.x..=max.x {
            let c = Coord { x, y };
            let surrounding = surroundings(g, &c);
            let adjacent_bugs = surrounding
                .iter()
                .cloned()
                .filter(|t| *t == Tile::Bug)
                .count();

            let bug = match g.get_default(&c) {
                Tile::Empty => {
                    if 1 <= adjacent_bugs && adjacent_bugs <= 2 {
                        true
                    } else {
                        false
                    }
                },
                Tile::Bug => {
                    if adjacent_bugs == 1 {
                        true
                    } else {
                        false
                    }
                },
            };

            next.map.insert(
                c,
                if bug {
                    Tile::Bug
                } else {
                    Tile::Empty
                }
            );
        }
    }

    *g = next;
}

fn grid_eq(g1: &Grid<Tile>, g2: &Grid<Tile>) -> bool {
    fn bug_count(g: &Grid<Tile>) -> usize {
        g
            .map
            .values()
            .filter(|t| **t == Tile::Bug)
            .count()
    }

    if bug_count(g1) != bug_count(g2) {
        return false;
    }

    for coord in g1.map.keys() {
        let t1 = g1.get_default(&coord);
        let t2 = g2.get_default(&coord);

        if t1 != t2 {
            return false;
        }
    }

    true
}

fn gethash(g: &Grid<Tile>) -> u64 {
    let (min, max) = g.minmax();

	let mut hasher = DefaultHasher::new();

    for y in min.y..=max.y {
        for x in min.x..=max.x {
            let c = Coord { x, y };
            c.hash(&mut hasher);

            let v = g.get_default(&c);
            v.hash(&mut hasher);
        }
    }

	hasher.finish()

}

fn biodiversity(g: &Grid<Tile>) -> usize {
    let (min, max) = g.minmax();
    let mut i = 1;
    let mut biod = 0;

    for y in min.y..=max.y {
        for x in min.x..=max.x {
            if *g.get_default(&Coord { x, y }) == Tile::Bug {
                biod += i;
            }

            i <<= 1;
        }
    }

    biod
}

fn show_grid(g: &Grid<Tile>) {
    let (min, max) = g.minmax();

    for y in min.y..=max.y {
        for x in min.x..=max.x {
            match g.get_default(&Coord { x, y }) {
                Tile::Empty => print!("."),
                Tile::Bug => print!("#"),
            }
        }
        println!();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut grid = parse(&fs::read_to_string("./input-day24")?);

    show_grid(&grid);

    let mut layouts = HashSet::new();

    for i in 1.. {
        let h = gethash(&grid);
        if layouts.contains(&h) {
            println!("dup after {}", i);
            show_grid(&grid);
            println!("diversity: {}", biodiversity(&grid));
            break;
        }
        layouts.insert(h);

        iterate(&mut grid);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse() {
		let grid = parse("
			....#
			#..#.
			#..##
			..#..
			#....
		");

		assert_eq!(grid.get_default(&Coord { x: 0, y: 0 }), &Tile::Empty);
		assert_eq!(grid.get_default(&Coord { x: 1, y: 0 }), &Tile::Empty);
		assert_eq!(grid.get_default(&Coord { x: 2, y: 0 }), &Tile::Empty);
		assert_eq!(grid.get_default(&Coord { x: 3, y: 0 }), &Tile::Empty);
		assert_eq!(grid.get_default(&Coord { x: 4, y: 0 }), &Tile::Bug);

		assert_eq!(grid.get_default(&Coord { x: 0, y: 1 }), &Tile::Bug);
		assert_eq!(grid.get_default(&Coord { x: 1, y: 1 }), &Tile::Empty);
		assert_eq!(grid.get_default(&Coord { x: 2, y: 1 }), &Tile::Empty);
		assert_eq!(grid.get_default(&Coord { x: 3, y: 1 }), &Tile::Bug);
		assert_eq!(grid.get_default(&Coord { x: 4, y: 1 }), &Tile::Empty);

		assert_eq!(grid.get_default(&Coord { x: 0, y: 2 }), &Tile::Bug);
		assert_eq!(grid.get_default(&Coord { x: 1, y: 2 }), &Tile::Empty);
		assert_eq!(grid.get_default(&Coord { x: 2, y: 2 }), &Tile::Empty);
		assert_eq!(grid.get_default(&Coord { x: 3, y: 2 }), &Tile::Bug);
		assert_eq!(grid.get_default(&Coord { x: 4, y: 2 }), &Tile::Bug);

		assert_eq!(grid.get_default(&Coord { x: 0, y: 3 }), &Tile::Empty);
		assert_eq!(grid.get_default(&Coord { x: 1, y: 3 }), &Tile::Empty);
		assert_eq!(grid.get_default(&Coord { x: 2, y: 3 }), &Tile::Bug);
		assert_eq!(grid.get_default(&Coord { x: 3, y: 3 }), &Tile::Empty);
		assert_eq!(grid.get_default(&Coord { x: 4, y: 3 }), &Tile::Empty);

		assert_eq!(grid.get_default(&Coord { x: 0, y: 4 }), &Tile::Bug);
		assert_eq!(grid.get_default(&Coord { x: 1, y: 4 }), &Tile::Empty);
		assert_eq!(grid.get_default(&Coord { x: 2, y: 4 }), &Tile::Empty);
		assert_eq!(grid.get_default(&Coord { x: 3, y: 4 }), &Tile::Empty);
		assert_eq!(grid.get_default(&Coord { x: 4, y: 4 }), &Tile::Empty);
	}

    #[test]
    fn test_iterate() {
        let initial = "
            ....#
            #..#.
            #..##
            ..#..
            #....
        ";

        let mut g = parse(&initial);
        iterate(&mut g);
        assert!(
            grid_eq(
                &g,
                &parse(&"
                    #..#.
                    ####.
                    ###.#
                    ##.##
                    .##..
                ")));

        iterate(&mut g);
        assert!(
            grid_eq(
                &g,
                &parse(&"
                    #####
                    ....#
                    ....#
                    ...#.
                    #.###
                ")));

        iterate(&mut g);
        assert!(
            grid_eq(
                &g,
                &parse(&"
                    #....
                    ####.
                    ...##
                    #.##.
                    .##.#
                ")));

        iterate(&mut g);
        assert!(
            grid_eq(
                &g,
                &parse(&"
                    ####.
                    ....#
                    ##..#
                    .....
                    ##...
                ")));
    }
}
