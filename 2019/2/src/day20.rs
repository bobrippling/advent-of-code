use std::fs;
use std::collections::{HashMap, HashSet};

mod d2;
use d2::{Coord, Compass};

mod grid;
use grid::Grid;

type GResult<T> = Result<T, Box<dyn std::error::Error>>;

const DEBUG_WALK: bool = false;
const DEBUG_WALK2: bool = true;
const DEBUG_EDGES: bool = true;

#[derive(Debug)]
enum Tile {
    Empty,
    Wall,
    Path,
    Teleport { name: String, is_outer: bool },
    Annotate(char),
}

struct Map {
    grid: Grid<Tile>,
    teleports: HashMap<String, (Option<Coord>, Option<Coord>)>,
    entrance: (String, Coord),
    exit: (String, Coord),
    min: Coord,
    max: Coord,
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Edge<'s> {
    from: &'s str,
    to: &'s str,
}

struct Path<'n> {
    nodes: Vec<&'n str>,
    distance: usize,
}

impl<'s> Edge<'s> {
    fn new(a: &'s str, b: &'s str) -> Self {
        Self {
            from: a.min(b),
            to: a.max(b),
        }
    }
}

impl<'n> Path<'n> {
    fn from(
        prev: &HashMap<&'n str, &'n str>,
        end: &'n str,
        distance: usize
    ) -> Self {
        let mut i = end;
        let mut nodes = Vec::new();

        loop {
            nodes.push(i);
            i = match prev.get(i) {
                Some(x) => x,
                None => break,
            };
        }

        nodes.reverse();

        Self {
            nodes,
            distance,
        }
    }
}

impl<'n> std::fmt::Debug for Path<'n> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "distance {}: ", self.distance)?;
        write!(fmt, "{}", self.nodes.join(" -> "))?;

        Ok(())
    }
}

fn bfs_1step<Elem>(grid: &Grid<Elem>, coord: &Coord, filter: fn(&Elem) -> bool) -> Vec<Coord> {
    [
        *coord + Compass::North,
        *coord + Compass::South,
        *coord + Compass::West,
        *coord + Compass::East,
    ]
        .iter()
        .filter(|c| {
            match grid.map.get(c).map(filter) {
                Some(b) => b,
                None => false,
            }
        })
        .cloned()
        .collect()
}

fn find_nearby<Elem>(grid: &Grid<Elem>, coord: &Coord, filter: fn(&Elem) -> bool) -> Option<Coord>
    where Elem: std::fmt::Debug
{
    let near = bfs_1step(grid, coord, filter);

    match near.len() {
        1 => Some(near[0]),
        0 => None,
        _ => {
            for coord in &near {
                eprintln!("{:?} => {:?}", coord, grid.map.get(&coord));
            }
            panic!("found too many coords near {:?}: {:?}", coord, near);
        }
    }

    /*
    let mut found = None;
    for coord in near {
        match coord {
            Some(c) => {
                assert!(!found);
                found = Some(coord);
            },
            _ => {},
        }
    }

    found
    */
}

fn teleport_name(a: char, b: char) -> String {
    if a < b {
        format!("{}{}", a, b)
    } else {
        format!("{}{}", b, a)
    }
}

/*
fn can_reach_edge_dir(grid: &Grid<char>, minmax: &(Coord, Coord), from: &Coord, dir: Compass) -> bool {
    let mut c = *from;

    while
        minmax.0.x <= c.x && c.x <= minmax.1.x &&
        minmax.0.y <= c.y && c.y <= minmax.1.y
    {
        c += dir;
        match grid.map.get(&c) {
            None => {},
            Some(ch) => {
                match ch {
                    ' ' => {},
                    _ => return false,
                }
            }
        }
    }

    true
}
*/

fn can_reach_edge_from(
    from: &Coord,
    (min, max): &(Coord, Coord),
) -> bool {
    /*return can_reach_edge_dir(grid, minmax, from, Compass::North)
        || can_reach_edge_dir(grid, minmax, from, Compass::South)
        || can_reach_edge_dir(grid, minmax, from, Compass::West)
        || can_reach_edge_dir(grid, minmax, from, Compass::East);*/
    from.x < min.x + 2 ||
    from.x > max.x - 2 ||
    from.y < min.y + 2 ||
    from.y > max.y - 2
}

impl Map {
    fn read(path: &str) -> Self {
        let mut chgrid = Grid::new();

        for (y, line) in fs::read_to_string(path)
            .expect("couldn't read file")
            .trim_end()
            .split('\n')
            .enumerate()
        {
            for (x, ch) in line.chars().enumerate() {
                let coord = Coord { x: x as isize, y: y as isize };
                chgrid.map.insert(coord, ch);
            }
        }

        let (min, max) = chgrid.minmax();

        let mut grid = Grid::new();
        let mut teleports = HashMap::<String, (Option<Coord>, Option<Coord>)>::new();

        for y in min.y..=max.y {
            for x in min.x..=max.x {
                let coord = Coord { x, y };
                let ch = match chgrid.map.get(&coord) {
                    Some(x) => x,
                    None => continue,
                };

                match ch {
                    '.' | ' ' | '#' => {
                        if grid.map.get(&coord).is_none() {
                            match ch {
                                '.' => grid.map.insert(coord, Tile::Path),
                                ' ' => grid.map.insert(coord, Tile::Empty),
                                '#' => grid.map.insert(coord, Tile::Wall),
                                _ => unreachable!(),
                            };
                        }
                    },
                    &x if x.is_ascii_uppercase() => {
                        grid.map.insert(coord, Tile::Annotate(x));

                        let near = match find_nearby(&chgrid, &coord, char::is_ascii_uppercase) {
                            Some(x) => x,
                            None => panic!("couldn't find neighbour"),
                        };

                        let name = teleport_name(x, chgrid.map.get(&near).cloned().unwrap());

                        let candidates = [
                            &coord,
                            &near,
                        ]
                            .iter()
                            .flat_map(|c| {
                                find_nearby(&chgrid, &c, |&ch| ch == '.')
                            })
                            .collect::<Vec<_>>();

                        assert_eq!(candidates.len(), 1);
                        let teleport_coord = candidates[0];

                        let is_outer = can_reach_edge_from(&coord, &(min, max));

                        grid.map.insert(teleport_coord, Tile::Teleport { name: name.clone(), is_outer });
                        //println!("teleport {} @ {:?}", name, teleport_coord);

                        match teleports.get(&name) {
                            Some((Some(a), None)) => {
                                if teleport_coord != *a {
                                    let copy = a.clone();
                                    teleports.insert(name, (Some(copy), Some(teleport_coord)));
                                }
                            },
                            None => {
                                teleports.insert(name, (Some(teleport_coord), None));
                            },
                            Some((Some(occupied1), Some(occupied2))) => {
                                if teleport_coord != *occupied1
                                && teleport_coord != *occupied2
                                {
                                    panic!("trying to set teleports[{}], occupied1 = {:?}, occupied2 = {:?}, new coord: {:?}",
                                           name, occupied1, occupied2, teleport_coord);
                                }
                            },
                            _ => panic!("invalid"),
                        };
                    },
                    _ => panic!("unknown entry '{}'", ch),
                };
            }
        }

        let mut entrance = None;
        let mut exit = None;

        for (name, (a, b)) in &teleports {
            if b.is_none() {
                match a {
                    None => {},
                    Some(a) => {
                        if entrance.is_none() {
                            entrance = Some((name.clone(), *a));
                            continue;
                        }
                        if exit.is_none() {
                            exit = Some((name.clone(), *a));
                            continue;
                        }
                        panic!("no matching teleport for {}: got {:?} and {:?}", name, a, b);
                    }
                }
            }
        }

        match (entrance, exit) {
            (Some(entrance), Some(exit)) => Self {
                grid,
                teleports,
                entrance,
                exit,
                min,
                max,
            },
            _ => panic!("no entrance/exit"),
        }
    }
}

fn show_map(map: &Map, custom: Option<fn(&Tile) -> String>) {
    let custom = custom.unwrap_or(|t| match t {
        Tile::Path => ".".into(),
        Tile::Wall => "#".into(),
        Tile::Empty => " ".into(),
        Tile::Teleport { is_outer: false, .. } => "*".into(),
        Tile::Teleport { is_outer: true, .. } => "@".into(),
        Tile::Annotate(c) => format!("{}", c),
    });

    for y in map.min.y..=map.max.y {
        for x in map.min.x..=map.max.x {
            let c = Coord { x, y };
            match map.grid.map.get(&c) {
                Some(tile) => print!("{}", custom(&tile)),
                None => print!(" "),
            };
        }
        print!(" {}\n", y);
    }
}

fn inner_outer(b: bool) -> &'static str {
    if b { "outer" } else { "inner" }
}

fn reachable_from<'m, 'c>(map: &'m Map, cur_level: usize, level_change: bool, coord: &'c Coord) -> HashSet<(&'m str, Coord, usize)> {
    let mut reachable = HashSet::<(&'m str, Coord, usize)>::new();

    let mut visited = HashSet::<Coord>::new();
    visited.insert(*coord);

    let mut todo = Vec::<(Coord, usize)>::new();
    todo.push((*coord, 0));

    while !todo.is_empty() {
        let (c, dist) = todo.pop().unwrap();

        let next = bfs_1step(
            &map.grid,
            &c,
            |tile| match tile {
                Tile::Path | Tile::Teleport { .. } => true,
                _ => false,
            });

        if DEBUG_WALK {
            println!("  {} step(s)", dist + 1);
        }

        for neighbour in next {
            if visited.contains(&neighbour) {
                continue;
            }
            visited.insert(neighbour);

            let tile = map.grid.map.get(&neighbour).unwrap();

            if DEBUG_WALK {
                println!("    neighbour: {:?} {:?}", neighbour, tile);
            }

            match tile {
                Tile::Path => todo.push((neighbour, dist + 1)),
                Tile::Teleport { name: s, is_outer } => {
                    let can_use = can_use_teleport(level_change, *is_outer, cur_level, s);

                    if DEBUG_WALK2 {
                        println!("  found {} ({}) - can use: {}, cur_level: {}",
                            s,
                            inner_outer(*is_outer),
                            can_use,
                            cur_level);
                    }

                    if can_use {
                        reachable.insert((&s, neighbour, dist + 1));
                    }

                    /*
                    let tele_coords = map.teleports.get(&s[..]).unwrap();
                    match tele_coords {
                        (Some(ref first), Some(ref second)) => {
                            if neighbour == *first {
                                todo.push((*second, dist + 1));
                            } else {
                                assert_eq!(neighbour, *second);
                                todo.push((*first, dist + 1));
                            }
                        },
                        (Some(ref first), None) => {
                            assert_eq!(neighbour, *first);
                        },
                        _ => {
                            panic!("unknown teleport state");
                        },
                    };
                    */
                },
                _ => unreachable!(),
            };
        }
    }

    reachable
}

fn dijkstra<'m>(
    map: &'m Map,
    edges: &HashMap<Edge<'m>, usize>,
) -> Option<Path<'m>> {
    let vertices = edges
        .keys()
        .flat_map(|Edge { from, to }| vec![from, to])
        .collect::<Vec<_>>();

    let mut distance = HashMap::<&str, usize>::new();

    let mut queue = HashSet::<&str>::new();
    let mut prev = HashMap::<&str, &str>::new();

    for &vertex in vertices {
        distance.insert(vertex, std::usize::MAX);
        //prev.insert(vertex, None);
        queue.insert(&vertex);
    }
    distance.insert(&map.entrance.0[..], 0);

    while !queue.is_empty() {
        let current: &str = queue
            .iter()
            .min_by_key(|&c| distance[c])
            .unwrap();

        queue.remove(current);

        //println!("chosen {} - distance {}", current, distance[current]);

        let neighbours = edges
            .iter()
            .filter(|(Edge { from, to }, _)| from == &current || to == &current);

        let dist_current = distance[current];

        for (Edge { from, to }, &dist) in neighbours {
            let other = if from == &current { to } else { from };

            let alt = dist_current.saturating_add(dist).saturating_add(1);
            if alt < distance[other] {
                distance.insert(other, alt);
                prev.insert(other, current);
            }

            //println!("  distance to {} is {}", other, alt);
        }
    }

    distance
        .get(&map.exit.0[..])
        .map(|d| Path::from(
            &prev,
            &map.exit.0[..],
            d - 1,
        ))
}

fn can_use_teleport(level_change: bool, is_outer: bool, level: usize, name: &str) -> bool {
    if !level_change {
        return true;
    }

    if !is_outer {
        return true;
    }

    if level == 0 {
        name == "AA" || name == "ZZ"
    } else {
        true
    }
    /*
    if name == "AA" || name == "ZZ" {
        level == 0
    } else {
        level != 0
    }
    */
}

fn walk_map<'m>(map: &'m Map, level_change: bool) -> Option<Path<'m>> {
    let mut edges = HashMap::<Edge<'m>, usize>::new();

    let mut visited = HashSet::<(&str, usize)>::new();

    let mut todo = Vec::<(&str, Coord, usize)>::new();
    todo.push((&map.entrance.0, map.entrance.1, 0));

    while !todo.is_empty() {
        let (cur_name, cur_coord, cur_level) = todo.pop().unwrap();

        if cur_level > 300 {
            continue;
        }

        let tile = map.grid.map.get(&cur_coord).unwrap();
        // assume we can use this tile, since we've got it in our todo list
        let is_outer = match tile {
            Tile::Teleport { ref name, is_outer } => {
                assert_eq!(name, cur_name);

                *is_outer
            },
            _ => panic!("not starting at label!?"),
        };

        if visited.contains(&(&cur_name[..], cur_level)) {
            continue;
        }
        visited.insert((&cur_name, cur_level));

        if DEBUG_WALK || DEBUG_WALK2 {
            println!("reachable_from({:?}) // for {} ({}, level = {})", cur_coord, cur_name, inner_outer(is_outer), cur_level);
        }

        let reachable = reachable_from(map, cur_level, level_change, &cur_coord);

        for (to_name, to_coord, to_dist) in reachable {
            if cur_name != to_name {
                let edge = Edge::new(cur_name, to_name);

                let should_insert = match edges.get(&edge) {
                    None => true,
                    Some(&curdist) => to_dist < curdist,
                };

                if should_insert {
                    if DEBUG_WALK2 {
                        println!("  adding to edges: {}", to_name);
                    }
                    edges.insert(edge, to_dist);
                }
            }

            let to_tile = map.grid.map.get(&to_coord).unwrap();
            let level_dir = match to_tile {
                Tile::Teleport { is_outer, .. } => {
                    if *is_outer { -1 } else { 1 }
                },
                _ => panic!("expected teleport"),
            };

            let new_level = ((cur_level as isize) + level_dir) as usize;
            if DEBUG_WALK2 {
                println!("  adding to todo: {}, level {}", to_name, new_level);
            }
            todo.push((to_name, to_coord, new_level));

            let tele_coords = map.teleports.get(&to_name[..]).unwrap();
            match tele_coords {
                (Some(ref first), Some(ref second)) => {
                    if to_coord == *first {
                        todo.push((to_name, *second, new_level));
                    } else {
                        assert_eq!(to_coord, *second);
                        todo.push((to_name, *first, new_level));
                    }
                },
                (Some(ref first), None) => {
                    assert_eq!(to_coord, *first);
                },
                _ => {
                    panic!("unknown teleport state");
                },
            };
        }
    }

    // now we have a set of edges, from AA --> BB, with weights.
    // find the cheapest route from map.entrance to map.exit
    if DEBUG_EDGES {
        for (Edge { from, to }, dist) in &edges {
            println!("{}:\t{} --> {}", dist, from, to);
        }
    }

    dijkstra(map, &edges)
}

fn eg(filename: &str, expected: usize, level_change: bool) {
    let map = Map::read(filename);

    println!("--- eg {} ---", filename);
    show_map(&map, None);

    let path = walk_map(&map, level_change);
    println!("path: {:?}", path);
    assert_eq!(path.unwrap().distance, expected);
    println!();
}

fn egs() {
    //eg("eg1-day20", 23, false);
    //eg("eg2-day20", 58, false);
    eg("eg1-day20", 26, true);
    eg("eg2-2-day20", 26, true);
}

fn part1() {
    let map = Map::read("./input-day20");

    show_map(&map, None);

    println!("part1: {:?}", walk_map(&map, false));
}

fn part2() {
    let map = Map::read("./input-day20");

    show_map(&map, None);

    println!("part2: {:?}", walk_map(&map, true));
}

fn main() -> GResult<()> {
    egs();
    //part1();
    //part2();

    Ok(())
}
