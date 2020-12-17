use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    #[cfg(not(feature = "part2"))]
    let part = 1;
    #[cfg(feature = "part2")]
    let part = 2;

    let (map, min, max) = parse(&input);

    println!("Part {}: {}", part, run(&map, min, max));
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    x: isize,
    y: isize,
    z: isize,
    #[cfg(feature = "part2")]
    w: isize,
}

impl Pos {
    fn neighbours(&self) -> impl Iterator<Item = Pos> {
        let &Pos {
            x,
            y,
            z,
            #[cfg(feature = "part2")]
            w,
        } = self;
        let x = x - 1;
        let y = y - 1;
        let z = z - 1;
        #[cfg(feature = "part2")]
        let w = w - 1;

        PosIter {
            start_x: x,
            start_y: y,
            start_z: z,
            #[cfg(feature = "part2")]
            start_w: w,
            x,
            y,
            z,
            #[cfg(feature = "part2")]
            w,
        }
    }

    fn update(&mut self, other: &Pos, min: bool) {
        if min {
            self.x = self.x.min(other.x);
            self.y = self.y.min(other.y);
            self.z = self.z.min(other.z);
            #[cfg(feature = "part2")]
            {
                self.w = self.w.min(other.w);
            }
        } else {
            self.x = self.x.max(other.x);
            self.y = self.y.max(other.y);
            self.z = self.z.max(other.z);
            #[cfg(feature = "part2")]
            {
                self.w = self.w.max(other.w);
            }
        }
    }
}

struct PosIter {
    start_x: isize,
    start_y: isize,
    start_z: isize,
    #[cfg(feature = "part2")]
    start_w: isize,
    x: isize,
    y: isize,
    z: isize,
    #[cfg(feature = "part2")]
    w: isize,
}

impl Iterator for PosIter {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        #[cfg(not(feature = "part2"))]
        if self.z > self.start_z + 2 {
            return None;
        }
        #[cfg(feature = "part2")]
        if self.w > self.start_w + 2 {
            return None;
        }

        loop {
            let pos = Pos {
                x: self.x,
                y: self.y,
                z: self.z,
                #[cfg(feature = "part2")]
                w: self.w,
            };

            self.x += 1;
            if self.x > self.start_x + 2 {
                self.x = self.start_x;
                self.y += 1;
                if self.y > self.start_y + 2 {
                    self.y = self.start_y;
                    self.z += 1;
                    #[cfg(feature = "part2")]
                    if self.z > self.start_z + 2 {
                        self.z = self.start_z;
                        {
                            self.w += 1;
                        }
                    }
                }
            }
            let origin = Pos {
                x: self.start_x + 1,
                y: self.start_y + 1,
                z: self.start_z + 1,
                #[cfg(feature = "part2")]
                w: self.start_w + 1,
            };
            if pos == origin {
                continue;
            }

            break Some(pos);
        }
    }
}

#[test]
fn test_neighbours() {
    let pos = Pos { x: 0, y: 0, z: 0 };

    assert_eq!(
        pos.neighbours().collect::<Vec<_>>(),
        vec![
            Pos {
                x: -1,
                y: -1,
                z: -1
            },
            Pos { x: 0, y: -1, z: -1 },
            Pos { x: 1, y: -1, z: -1 },
            Pos { x: -1, y: 0, z: -1 },
            Pos { x: 0, y: 0, z: -1 },
            Pos { x: 1, y: 0, z: -1 },
            Pos { x: -1, y: 1, z: -1 },
            Pos { x: 0, y: 1, z: -1 },
            Pos { x: 1, y: 1, z: -1 },
            Pos { x: -1, y: -1, z: 0 },
            Pos { x: 0, y: -1, z: 0 },
            Pos { x: 1, y: -1, z: 0 },
            Pos { x: -1, y: 0, z: 0 },
            //Pos { x:  0, y:  0, z:  0 },
            Pos { x: 1, y: 0, z: 0 },
            Pos { x: -1, y: 1, z: 0 },
            Pos { x: 0, y: 1, z: 0 },
            Pos { x: 1, y: 1, z: 0 },
            Pos { x: -1, y: -1, z: 1 },
            Pos { x: 0, y: -1, z: 1 },
            Pos { x: 1, y: -1, z: 1 },
            Pos { x: -1, y: 0, z: 1 },
            Pos { x: 0, y: 0, z: 1 },
            Pos { x: 1, y: 0, z: 1 },
            Pos { x: -1, y: 1, z: 1 },
            Pos { x: 0, y: 1, z: 1 },
            Pos { x: 1, y: 1, z: 1 },
        ]
    );

    let pos = Pos { x: 1, y: 2, z: 3 };

    assert_eq!(
        pos.neighbours().collect::<Vec<_>>(),
        vec![
            Pos { x: 0, y: 1, z: 2 },
            Pos { x: 1, y: 1, z: 2 },
            Pos { x: 2, y: 1, z: 2 },
            Pos { x: 0, y: 2, z: 2 },
            Pos { x: 1, y: 2, z: 2 },
            Pos { x: 2, y: 2, z: 2 },
            Pos { x: 0, y: 3, z: 2 },
            Pos { x: 1, y: 3, z: 2 },
            Pos { x: 2, y: 3, z: 2 },
            Pos { x: 0, y: 1, z: 3 },
            Pos { x: 1, y: 1, z: 3 },
            Pos { x: 2, y: 1, z: 3 },
            Pos { x: 0, y: 2, z: 3 },
            //Pos { x:  1, y:  2, z: 3 },
            Pos { x: 2, y: 2, z: 3 },
            Pos { x: 0, y: 3, z: 3 },
            Pos { x: 1, y: 3, z: 3 },
            Pos { x: 2, y: 3, z: 3 },
            Pos { x: 0, y: 1, z: 4 },
            Pos { x: 1, y: 1, z: 4 },
            Pos { x: 2, y: 1, z: 4 },
            Pos { x: 0, y: 2, z: 4 },
            Pos { x: 1, y: 2, z: 4 },
            Pos { x: 2, y: 2, z: 4 },
            Pos { x: 0, y: 3, z: 4 },
            Pos { x: 1, y: 3, z: 4 },
            Pos { x: 2, y: 3, z: 4 },
        ]
    );
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Cube {
    Active,
    Inactive,
}

#[allow(dead_code)]
impl Cube {
    fn to_ch(self) -> char {
        match self {
            Cube::Active => '#',
            Cube::Inactive => '.',
        }
    }
}

fn parse(s: &str) -> (HashMap<Pos, Cube>, Pos, Pos) {
    let mut map = HashMap::new();
    let mut max = Pos {
        x: 0,
        y: 0,
        z: 0,
        #[cfg(feature = "part2")]
        w: 0,
    };
    let mut min = Pos {
        x: 0,
        y: 0,
        z: 0,
        #[cfg(feature = "part2")]
        w: 0,
    };

    s.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, ch)| {
            let cube = match ch {
                '.' => Cube::Inactive,
                '#' => Cube::Active,
                _ => panic!(),
            };

            let x = x as isize;
            let y = y as isize;
            let z = 0;
            #[cfg(feature = "part2")]
            let w = 0;

            let pos = Pos {
                x,
                y,
                z,
                #[cfg(feature = "part2")]
                w,
            };
            map.insert(pos, cube);
            min.update(&pos, true);
            max.update(&pos, false);
        })
    });

    (map, min, max)
}

fn run(map: &HashMap<Pos, Cube>, mut min: Pos, mut max: Pos) -> usize {
    let mut map = (*map).clone();

    //dump(&map, &min, &max);

    for _ in 0..6 {
        //println!("cycle, min={:?}, max={:?}", min, max);
        let new = cycle(&map, &mut min, &mut max);

        //dump(&new, &min, &max);

        map = new;
    }

    map.values().filter(|&c| *c == Cube::Active).count()
}

fn cycle(map: &HashMap<Pos, Cube>, min: &mut Pos, max: &mut Pos) -> HashMap<Pos, Cube> {
    let mut new = HashMap::new();

    let (min, min_ref) = (*min, min);
    let (max, max_ref) = (*max, max);

    let mut iteration = |pos: Pos| {
        let active_neighbours = pos
            .neighbours()
            .map(|neighbour_pos| map.get(&neighbour_pos).unwrap_or(&Cube::Inactive))
            .filter(|&c| *c == Cube::Active)
            .count();

        let current = map.get(&pos).unwrap_or(&Cube::Inactive);

        match (current, active_neighbours) {
            (Cube::Active, 2) | (Cube::Active, 3) => {
                //println!("{:?}, Active & {} neighbours --> Active", pos, active_neighbours);
                new.insert(pos, Cube::Active);

                min_ref.update(&pos, true);
                max_ref.update(&pos, false);
            }
            (Cube::Inactive, 3) => {
                //println!("{:?}, Inactive & {} neighbours --> Active", pos, active_neighbours);
                new.insert(pos, Cube::Active);

                min_ref.update(&pos, true);
                max_ref.update(&pos, false);
            }
            (Cube::Active, _) => {
                //println!("{:?}, Active & !2/3 neighbours --> Inactive", pos);
                new.remove(&pos); // set inactive
            }
            _ => {}
        }
    };

    #[cfg(not(feature = "part2"))]
    for z in min.z - 1..=max.z + 1 {
        for y in min.y - 1..=max.y + 1 {
            for x in min.x - 1..=max.x + 1 {
                let pos = Pos { x, y, z };

                iteration(pos);
            }
        }
    }

    #[cfg(feature = "part2")]
    for w in min.w - 1..=max.w + 1 {
        for z in min.z - 1..=max.z + 1 {
            for y in min.y - 1..=max.y + 1 {
                for x in min.x - 1..=max.x + 1 {
                    let pos = Pos { x, y, z, w };

                    iteration(pos);
                }
            }
        }
    }

    new
}

/*
#[allow(dead_code)]
fn dump(map: &HashMap<Pos, Cube>, min: &Pos, max: &Pos) {
    for z in min.z..=max.z {
        println!();
        println!("z={}", z);

        for y in min.y..=max.y {
            for x in min.x..=max.x {
                let pos = Pos {
                    x,
                    y,
                    z,
                    #[cfg(feature = "part2")]
                    w,
                };
                print!("{}", map.get(&pos).unwrap_or(&Cube::Inactive).to_ch());
            }
            println!();
        }
    }
    println!("---------");
}
*/

#[test]
fn test_part1() {
    let init = ".#.
..#
###";
    let (map, min, max) = parse(&init);
    assert_eq!(run(&map, min, max), 112);
}
