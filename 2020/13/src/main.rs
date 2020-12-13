use std::fs;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let s = fs::read_to_string("./input.txt")?;

    let (earliest, times) = parse(&s)?;
    println!("Part 1: {}", part1(earliest, &times));

    let times = parse2(&s)?;
    println!("Part 2: {}", part2(&times) + 1);

    Ok(())
}

fn parse(s: &str) -> Result<(usize, Vec<usize>)> {
    let parts: Vec<_> = s.lines().collect();

    if parts.len() != 2 {
        return Err("invalid line count".into());
    }

    let earliest = parts[0].parse()?;
    let times = parts[1]
            .split(',')
            .filter_map(|l| match l {
                "" | "x" => None,
                _ => Some(l.parse()),
            })
            .collect::<std::result::Result<Vec<_>, _>>()?;

    Ok((earliest, times))
}

fn part1(earliest: usize, times: &Vec<usize>) -> usize {
    let (bus_id, wait) = times.iter()
        .map(|&time| {
            let mut t = time;
            let mut mul = 2;
            while t < earliest {
                t = time * mul;
                mul += 1;
            }
            (time, t - earliest)
        })
        .min_by_key(|&(_bus_id, wait)| wait)
        .unwrap();

    bus_id * wait
}

#[test]
fn test_part1() {
    let earliest = 939;
    let times = vec![7,13,59,31,19];
    assert_eq!(part1(earliest, &times), 295);
}

fn parse2(s: &str) -> Result<Vec<Option<usize>>> {
    let parts = s.lines()
        .nth(1)
        .ok_or("too few lines")?
        .split(',')
        .filter(|l| !l.is_empty())
        .map(|l| -> Result<Option<usize>> {
            if l == "x" {
                Ok(None)
            } else {
                Ok(Some(l.parse()?))
            }
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(parts)
}

fn to_f64(u: usize) -> f64 { u as _ }

fn part2(times: &Vec<Option<usize>>) -> usize {
    struct R {
        // find some 'x' such that for all entries,
        // we get a remainder of 'a' when divided by 'n'
        a: usize,
        n: usize,
    }

    let start_modulo = times
        .iter()
        .enumerate()
        .map(|(i, x)| x.map(|_| i))
        .filter_map(|x| x)
        .last()
        .unwrap();

    let times = times
        .iter()
        .enumerate()
        .map(|(i, time)| time.map(|time| R { a: i, n: time }))
        .filter_map(|x| x)
        .collect::<Vec<_>>();

    let modulos_adjusted: Vec<_> = times.iter().map(|&R { a, n: _ }| {
        start_modulo - a
    }).map(to_f64).collect();

    #[cfg(feature = "show-buses")]
    times.iter().enumerate().for_each(|(i, R { a, n })| {
        println!("bus {} leaves every {} mins, using remainder {}", a, n, modulos_adjusted[i]);
    });

    let x = chinese_remainder(
        modulos_adjusted,
        times.iter().map(|&R { a: _, n }| n).map(to_f64).collect(),
    );

    x as usize - start_modulo

    // problems:
    // 1)
    //   need to reverse the modulo we want, since we're looking for
    //     1068781    D       .       .       .       .
    //     1068782    .       D       .       .       .
    //     1068783    .       .       .       .       .
    //     1068784    .       .       .       .       .
    //     1068785    .       .       D       .       .
    //     1068786    .       .       .       .       .
    //     1068787    .       .       .       D       .
    //     1068788    D       .       .       .       D
    //   but chinese-remainder gives us:
    //     1068781            .       .       .       D
    //     1068782    .       .       .       D       .
    //     1068783    .       .       .       .       .
    //     1068784    .       .       D       .       .
    //     1068785    .       .       .       .       .
    //     1068786    .       .       .       .       .
    //     1068787    .       D       .       .       .
    //     1068788    D       .       .       .       .
    //
    // we then adjust this by the modulo adjustment to get flip back to our answer
}

fn chinese_remainder(a: Vec<f64>, n: Vec<f64>) -> f64 {
    let common_product: f64 = n.iter().cloned().product();

    let n_divided = n.iter().cloned().map(|ni| common_product / ni).collect::<Vec<f64>>();
    let mut cr = 0.0;

    for i in 0..n_divided.len() {
        cr = (
            cr + (
                (a[i] * n_divided[i]) % common_product
                * modular_multiplicative_inverse_brute(n_divided[i] as _, n[i] as _) as f64)
            % common_product
        ) % common_product;
    }

    cr
}

fn modular_multiplicative_inverse_brute(e: usize, modu: usize) -> usize {
    for i in 1..modu {
        if (e * i) % modu == 1 {
            return i;
        }
    }
    panic!();
}

#[test]
fn test_part2_2() {
    let times = vec![Some(7),Some(13),None,None,Some(59),None,Some(31),Some(19)];
    assert_eq!(part2(&times), 1068781);
}

#[test]
fn test_part2_3() {
    let times = vec![Some(17),None,Some(13),Some(19)];
    assert_eq!(part2(&times), 3417);
}

#[test]
fn test_part2_4() {
    let times = vec![Some(67),Some(7),Some(59),Some(61)];
    assert_eq!(part2(&times), 754018);
}

#[test]
fn test_part2_5() {
    let times = vec![Some(67),None,Some(7),Some(59),Some(61)];
    assert_eq!(part2(&times), 779210);
}

#[test]
fn test_part2_6() {
    let times = vec![Some(67),Some(7),None,Some(59),Some(61)];
    assert_eq!(part2(&times), 1261476);
}

#[test]
fn test_part2_7() {
    let times = vec![Some(1789),Some(37),Some(47),Some(1889)];
    assert_eq!(part2(&times), 1202161486);
}
