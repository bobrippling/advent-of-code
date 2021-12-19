use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let scanners = std::fs::read_to_string("input.txt")?.parse()?;
    let r = go(&scanners);

    println!("Part 1: {}", part1(&r));
    println!("Part 2: {}", part2(&r));

    Ok(())
}

fn part1(&(count, _): &(usize, HashMap<usize, Beacon>)) -> usize {
    count
}

fn part2((_, scanner_positions): &(usize, HashMap<usize, Beacon>)) -> i64 {
    scanner_positions
        .iter()
        .permutations(2)
        .filter_map(|scanners| {
            if let [(i, a), (j, b)] = scanners[..] {
                if i == j {
                    None
                } else {
                    Some((a, b))
                }
            } else {
                panic!()
            }
        })
        .map(|(a, b)| manhatten(a, b))
        .max()
        .unwrap()
}

fn manhatten(a: &Beacon, b: &Beacon) -> i64 {
    (a - b).magnitude()
}

fn go(scanners: &Scanners) -> (usize, HashMap<usize, Beacon>) {
    let coord_remaps = [
        (0, 1, 2),
        (0, 2, 1),
        (1, 0, 2),
        (1, 2, 0),
        (2, 0, 1),
        (2, 1, 0),
    ];
    let coord_flips = [
        (1, 1, 1),
        (1, 1, -1),
        (1, -1, 1),
        (1, -1, -1),
        (-1, 1, 1),
        (-1, 1, -1),
        (-1, -1, 1),
        (-1, -1, -1),
    ];

    let apply = |coord: &Beacon, remap: (_, _, _), flip: (_, _, _)| Beacon {
        x: flip.0 * coord[remap.0],
        y: flip.1 * coord[remap.1],
        z: flip.2 * coord[remap.2],
    };

    let mut all_beacons = HashSet::new(); // all in alignment of [0]
    let mut scanner_positions = HashMap::new();

    for b in &scanners.0[0].0 {
        all_beacons.insert(b.clone());
    }
    scanner_positions.insert(0, Default::default());

    while scanner_positions.len() != scanners.0.len() {
        for (i, scanner) in scanners.0.iter().enumerate() {
            if scanner_positions.contains_key(&i) {
                continue;
            }
            let beacons = &scanner.0;

            'outer: for remap in coord_remaps {
                for flip in coord_flips {
                    let mapped: HashSet<_> = beacons
                        .iter()
                        .map(|beacon| apply(beacon, remap, flip))
                        .collect();

                    for beacon in &all_beacons {
                        for beacon2 in &mapped {
                            let ref diff = beacon - beacon2;

                            let transformed: HashSet<_> = mapped.iter().map(|b| b + diff).collect();
                            let overlap = transformed.intersection(&all_beacons);

                            if overlap.count() >= 12 {
                                let scanner_pos = Beacon::default();
                                let scanner_pos = &scanner_pos + diff;
                                scanner_positions.insert(i, scanner_pos);

                                for b in transformed {
                                    all_beacons.insert(b);
                                }
                                break 'outer;
                            }
                        }
                    }
                }
            }
        }
    }

    (all_beacons.len() as _, scanner_positions)
}

struct Scanners(Vec<Beacons>);

struct Beacons(HashSet<Beacon>);

#[derive(Hash, PartialEq, Eq, Clone, Default)]
struct Beacon {
    x: i64,
    y: i64,
    z: i64,
}

impl BeaconDiff {
    fn magnitude(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl std::fmt::Debug for Beacon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

struct BeaconDiff {
    x: i64,
    y: i64,
    z: i64,
}

impl std::ops::Sub for &Beacon {
    type Output = BeaconDiff;

    fn sub(self, rhs: Self) -> Self::Output {
        BeaconDiff {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Add<&BeaconDiff> for &Beacon {
    type Output = Beacon;

    fn add(self, rhs: &BeaconDiff) -> Self::Output {
        Beacon {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Index<u8> for Beacon {
    type Output = i64;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!(),
        }
    }
}

impl std::str::FromStr for Scanners {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split("\n\n")
                .map(str::parse)
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

impl std::str::FromStr for Beacons {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .skip(1)
                .map(str::trim)
                .map(str::parse)
                .collect::<Result<HashSet<_>, _>>()?,
        ))
    }
}

impl std::str::FromStr for Beacon {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(',').collect();

        if let [x, y, z] = parts[..] {
            let x = x.parse().map_err(|_| "parse int")?;
            let y = y.parse().map_err(|_| "parse int")?;
            let z = z.parse().map_err(|_| "parse int")?;
            Ok(Self { x, y, z })
        } else {
            Err("expected three coords")
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let r = go(&EG.parse().unwrap());
        assert_eq!(part1(&r), 79);
    }

    #[test]
    fn test_manhatten() {
        let a = Beacon {
            x: 1105,
            y: -1205,
            z: 1229,
        };
        let b = Beacon {
            x: -92,
            y: -2380,
            z: -20,
        };

        assert_eq!(manhatten(&a, &b), 3621);
    }

    #[test]
    fn test_part2() {
        let r = go(&EG.parse().unwrap());
        assert_eq!(part2(&r), 3621);
    }

    static EG: &'static str = "\
    --- scanner 0 ---
    404,-588,-901
    528,-643,409
    -838,591,734
    390,-675,-793
    -537,-823,-458
    -485,-357,347
    -345,-311,381
    -661,-816,-575
    -876,649,763
    -618,-824,-621
    553,345,-567
    474,580,667
    -447,-329,318
    -584,868,-557
    544,-627,-890
    564,392,-477
    455,729,728
    -892,524,684
    -689,845,-530
    423,-701,434
    7,-33,-71
    630,319,-379
    443,580,662
    -789,900,-551
    459,-707,401

    --- scanner 1 ---
    686,422,578
    605,423,415
    515,917,-361
    -336,658,858
    95,138,22
    -476,619,847
    -340,-569,-846
    567,-361,727
    -460,603,-452
    669,-402,600
    729,430,532
    -500,-761,534
    -322,571,750
    -466,-666,-811
    -429,-592,574
    -355,545,-477
    703,-491,-529
    -328,-685,520
    413,935,-424
    -391,539,-444
    586,-435,557
    -364,-763,-893
    807,-499,-711
    755,-354,-619
    553,889,-390

    --- scanner 2 ---
    649,640,665
    682,-795,504
    -784,533,-524
    -644,584,-595
    -588,-843,648
    -30,6,44
    -674,560,763
    500,723,-460
    609,671,-379
    -555,-800,653
    -675,-892,-343
    697,-426,-610
    578,704,681
    493,664,-388
    -671,-858,530
    -667,343,800
    571,-461,-707
    -138,-166,112
    -889,563,-600
    646,-828,498
    640,759,510
    -630,509,768
    -681,-892,-333
    673,-379,-804
    -742,-814,-386
    577,-820,562

    --- scanner 3 ---
    -589,542,597
    605,-692,669
    -500,565,-823
    -660,373,557
    -458,-679,-417
    -488,449,543
    -626,468,-788
    338,-750,-386
    528,-832,-391
    562,-778,733
    -938,-730,414
    543,643,-506
    -524,371,-870
    407,773,750
    -104,29,83
    378,-903,-323
    -778,-728,485
    426,699,580
    -438,-605,-362
    -469,-447,-387
    509,732,623
    647,635,-688
    -868,-804,481
    614,-800,639
    595,780,-596

    --- scanner 4 ---
    727,592,562
    -293,-554,779
    441,611,-461
    -714,465,-776
    -743,427,-804
    -660,-479,-426
    832,-632,460
    927,-485,-438
    408,393,-506
    466,436,-512
    110,16,151
    -258,-428,682
    -393,719,612
    -211,-452,876
    808,-476,-593
    -575,615,604
    -485,667,467
    -680,325,-822
    -627,-443,-432
    872,-547,-609
    833,512,582
    807,604,487
    839,-516,451
    891,-625,532
    -652,-548,-490
    30,-46,-14\
    ";
}
