use std::fs;
use std::collections::HashSet;

type N = i64;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = fs::read_to_string("./input.txt")?;

    let nums = s.split('\n')
        .filter(|l| !l.is_empty())
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    let window_size = 25;

    let n = part1(&nums, window_size);
    println!("Part 1: {}", n);

    let n = part2(&nums, n);
    println!("Part 2: {}", n);

    Ok(())
}

fn part1(nums: &[N], window_size: usize) -> N {
    let mut window = nums[..window_size].iter().cloned().collect::<HashSet<_>>();

    let iter = &nums[window_size..];
    for (&n, &last) in iter.iter().zip(nums.iter()) {
        if !contains_sum(&window, n) {
            return n;
        }

        window.remove(&last);
        window.insert(n);
    }

    panic!("not found!");
}

fn contains_sum(window: &HashSet<N>, target: N) -> bool {
    for n in window {
        let other = target - n;

        if window.contains(&other) {
            return true;
        }
    }

    false
}

static EG: &[N] = &[
    35,
    20,
    15,
    25,
    47,
    40,
    62,
    55,
    65,
    95,
    102,
    117,
    150,
    182,
    127,
    219,
    299,
    277,
    309,
    576,
    ];

#[test]
fn test_part1() {
    assert_eq!(part1(&EG, 5), 127);
}

fn part2(nums: &[N], target: N) -> N {
    let range = find_range_summing_to(nums, target);

    let ns = &nums[range];
    ns.iter().min().unwrap() + ns.iter().max().unwrap()
}

fn find_range_summing_to(nums: &[N], target: N) -> std::ops::Range<usize> {
    for i in 0..nums.len() {
        for last in i+3..nums.len() {
            let sum: N = nums[i..last].iter().sum();
            if sum == target {
                return i..last;
            }
        }
    }
    panic!();
}

#[test]
fn test_part2() {
    let part1_answer = 127;
    assert_eq!(part2(&EG, part1_answer), 62);
}
