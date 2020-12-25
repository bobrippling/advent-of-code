fn main() {
    let ns = [15733400, 6408062];
    let subject = 7;

    println!("{}", part1(ns, subject));
}

fn part1([a, b]: [usize; 2], subject: usize) -> usize {
    let mut value = subject;

    for i in 2.. {
        value = transform(value, subject);

        if value == a {
            // a loop size is `i`

            let subject = b;
            let mut sk = subject;
            for _ in 0..=i-2 {
                sk = transform(sk, subject);
            }
            return sk;
        }
        if value == b {
            // b loop size is `i`

            let subject = a;
            let mut sk = subject;
            for _ in 0..=i-2 {
                sk = transform(sk, subject);
            }
            return sk;
        }
    }
    unreachable!();
}

fn transform(value: usize, subject: usize) -> usize {
    (value * subject) % 20201227
}

#[test]
fn test_part1() {
    // card loop size 8: 5764801
    // door loop size 11: 17807724
    let ns = [5764801, 17807724];
    let subject = 7;

    assert_eq!(part1(ns, subject), 14897079);
}
