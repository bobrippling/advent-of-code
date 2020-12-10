use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

use pathfinding::prelude::dfs as depth_first_search;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = fs::read_to_string("./input.txt")?;

    let adapters = parse(&s)?;

    let info = part1(&adapters);
    println!("Part 1: {}", info.diff_1jolt * info.diff_3jolt);

    let npaths = part2(&adapters);
    println!("Part 2: {}", npaths);

    Ok(())
}

fn parse(s: &str) -> Result<HashSet<i32>, Box<dyn std::error::Error>> {
    let adapters = s.split('\n')
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|s| s.parse())
        .collect::<Result<HashSet<_>, _>>()?;

    Ok(adapters)
}

#[derive(PartialEq, Eq, Debug)]
struct Part1Info {
    max: i32,
    diff_1jolt: usize,
    diff_3jolt: usize,
}

fn part1(adapters: &HashSet<i32>) -> Part1Info {
    let chain = get_chain(adapters);

    let max = chain
        .values()
        .flat_map(|vals| vals)
        .cloned()
        .max()
        .unwrap();

    let path = get_route(&chain, max);
    let mut info = Part1Info {
        max,
        diff_1jolt: 0,
        diff_3jolt: 1, // account for the existing jolt from max up to our device
    };

    let mut last = None;
    for ent in path {
        if let Some(last) = last {
            let diff = ent - last;
            match diff {
                1 => info.diff_1jolt += 1,
                3 => info.diff_3jolt += 1,
                x if x > 0 => {}
                _ => {
                    panic!("going down an adapter!");
                }
            };
        }
        last = Some(ent);
    }

    info
}

fn get_chain(adapters: &HashSet<i32>) -> HashMap<i32, HashSet<i32>> {
    // adapter can take input 1, 2 or 3 jolts lower
    // device is rated for 3 jolts higher than the higest adapter
    let mut links = HashMap::new();
    let mut work = vec![0];
    let mut done = HashSet::new();

    while let Some(current) = work.pop() {
        if done.contains(&current) {
            continue;
        }
        done.insert(current);

        [
            adapters.get(&(current + 1)),
            adapters.get(&(current + 2)),
            adapters.get(&(current + 3)),
        ]
            .iter()
            .filter_map(|&ent| ent.map(|x| *x))
            .for_each(|candidate| {
                // from 'current' we can go up to 'candidate'
                let entry = links
                    .entry(current)
                    .or_insert(HashSet::new());
                entry.insert(candidate);

                // and let's look at candidate
                work.push(candidate);
            });
    }

    links
}

fn get_route(chain: &HashMap<i32, HashSet<i32>>, end: i32) -> Vec<i32> {
    depth_first_search(
        0,
        |node| {
            let mut next = chain
                .get(node)
                .unwrap()
                .iter()
                .cloned()
                .collect::<Vec<_>>();

            // sort the possible next adapters, so we
            // try the smallest one first (as per requirements)
            next.sort();

            next
        },
        |&node| node == end,
    ).unwrap()
}

#[test]
fn test_part1() {
    let eg = r#"
        16
        10
        15
        5
        1
        11
        7
        19
        6
        12
        4
    "#;

    let adapters = parse(eg).unwrap();

    // max = 19, so 22 to account for our device
    let max = part1(&adapters);
    assert_eq!(
        max,
        Part1Info {
            max: 19,
            diff_1jolt: 7,
            diff_3jolt: 5,
        });
}

fn part2(adapters: &HashSet<i32>) -> usize {
    let chain = get_chain(adapters);
    let mut memo = HashMap::new();

    arrangements_from(0, &chain, &mut memo)
}

fn arrangements_from(
    start: i32,
    chain: &HashMap<i32, HashSet<i32>>,
    memo: &mut HashMap<i32, usize>,
) -> usize {
    if let Some(&x) = memo.get(&start) {
        return x;
    }

    let subarrangements = match chain.get(&start) {
        Some(subadapters) => {
            subadapters
                .iter()
                .cloned()
                .map(|adapter| arrangements_from(adapter, chain, memo))
                .sum()
        }
        None => {
            // done, only one way to arrange nothing else
            1
        }
    };

    memo.insert(start, subarrangements);
    subarrangements
}

#[test]
fn test_part2() {
    let eg = r#"
        16
        10
        15
        5
        1
        11
        7
        19
        6
        12
        4
    "#;

    let adapters = parse(eg).unwrap();

    // max = 19, so 22 to account for our device
    assert_eq!(
        part2(&adapters),
        8,
    );
}
